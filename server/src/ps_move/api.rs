use std::ffi::CString;
use std::str;

use hidapi::{DeviceInfo, HidApi, HidDevice};
use log::{error, trace};
use palette::Hsv;

use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::{ConnectionType, ControllerInfo, MoveRequestType};

const MAGIC_PATH: &str = "&Col01#";
const WINDOWS_BLUETOOTH_MAGIC_PATH: &str = "&Col02#";
const PS_MOVE_VENDOR_ID: u16 = 0x054c;
const PS_MOVE_PRODUCT_ID: u16 = 0x03d5;

const PS_MOVE_BT_ADDR_GET_SIZE: usize = 16;

pub struct PsMoveApi {
    hid: HidApi,
}

pub struct ListingResult {
    pub disconnected: Vec<ControllerInfo>,
    pub connected: Vec<ControllerInfo>,
}

impl ListingResult {
    pub(super) fn new() -> ListingResult {
        ListingResult {
            disconnected: Vec::new(),
            connected: Vec::new(),
        }
    }
}

impl PsMoveApi {
    pub fn new() -> PsMoveApi {
        PsMoveApi {
            hid: HidApi::new().unwrap_or_else(|_| panic!("Couldn't init hidapi")),
        }
    }

    pub fn refresh(&mut self) {
        let result = self.hid.refresh_devices();

        if result.is_err() {
            error!("Failed to refresh devices {}", result.unwrap_err());
        }
    }

    /// Returns the controller changes found in the last [`Self::refresh()`] call.
    /// (the refresh is expensive)
    ///
    /// Note: has all raw devices info, so one controller can appear twice,
    /// if connected via both USB and BT
    pub fn list(&mut self, old_controllers: &[PsMoveController]) -> ListingResult {
        let mut result = ListingResult::new();
        let current_controllers = self.list_psmove_devices();

        Self::get_disconnected_controllers(old_controllers, &mut result, &current_controllers);
        Self::get_connected_controllers(old_controllers, &mut result, current_controllers);

        result
    }

    pub fn connect_controllers(
        &self,
        controllers_info: Vec<ControllerInfo>,
    ) -> Vec<PsMoveController> {
        controllers_info
            .iter()
            .filter_map(|dev_info| {
                let path = if dev_info.bt_path.is_empty() {
                    &dev_info.usb_path
                } else {
                    &dev_info.bt_path
                };

                self.connect_controller(&dev_info.serial_number, path)
            })
            .collect()
    }

    fn list_psmove_devices(&mut self) -> Vec<ControllerInfo> {
        self.hid
            .device_list()
            .filter(|dev_info| Self::is_move_controller(dev_info))
            .map(|dev_info| {
                let serial_number = dev_info.serial_number().unwrap_or("");
                let path = dev_info.path().to_str().unwrap();

                ControllerInfo::from(serial_number, path)
            })
            .collect()
    }

    /// Adds the `old_controllers` not present in `current_controllers` to `result::disconnected`.
    fn get_disconnected_controllers(
        old_controllers: &[PsMoveController],
        result: &mut ListingResult,
        current_controllers: &[ControllerInfo],
    ) {
        old_controllers
            .iter()
            .filter(|ctl| ctl.connection_type != ConnectionType::UsbAndBluetooth)
            .filter(|ctl| {
                !current_controllers
                    .iter()
                    .any(|info| ctl.is_same_device(info))
            })
            .for_each(|ctl| result.disconnected.push(ctl.info.clone()));

        old_controllers
            .iter()
            .filter(|ctl| ctl.connection_type == ConnectionType::UsbAndBluetooth)
            .filter(|ctl| {
                // USB and Bluetooth must appear twice in the listing
                !current_controllers.iter().any(|info| {
                    ctl.is_same_device(info)
                        && current_controllers
                        .iter()
                        .any(|other| ctl.is_same_device(other) && info != other)
                })
            })
            .for_each(|ctl| result.disconnected.push(ctl.info.clone()));
    }

    /// Adds the `new_controllers` not present in `old_controllers` to `result::connected`.
    fn get_connected_controllers(
        old_controllers: &[PsMoveController],
        result: &mut ListingResult,
        current_controllers: Vec<ControllerInfo>,
    ) {
        current_controllers
            .iter()
            .filter(|info| !old_controllers.iter().any(|ctl| ctl.is_same_device(info)))
            .for_each(|info| result.connected.push(info.clone()));
    }

    fn connect_controller(&self, serial_number: &str, path: &str) -> Option<PsMoveController> {
        let mut bt_address = String::from(serial_number);
        let path = String::from(path);

        let connection_type = if bt_address.is_empty() {
            ConnectionType::Usb
        } else {
            ConnectionType::Bluetooth
        };

        let device = if bt_address.is_empty() && !path.is_empty() {
            self.hid.open_path(&CString::new(path.clone()).unwrap())
        } else {
            self.hid
                .open_serial(PS_MOVE_VENDOR_ID, PS_MOVE_PRODUCT_ID, &bt_address)
        };

        match device {
            Ok(device) => {
                let mut bt_path = String::new();
                let mut usb_path = String::new();

                match device.set_blocking_mode(false) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Unable to set '{bt_address}' to nonblocking {err}");
                        return None;
                    }
                }

                if connection_type == ConnectionType::Usb {
                    usb_path = path.clone();
                    bt_address = if cfg!(windows) {
                        self.get_bt_address_on_windows(&path)
                    } else {
                        Self::get_bt_address(&device).unwrap_or_else(|| String::from(""))
                    }
                } else {
                    bt_path = path
                }

                Some(PsMoveController::new(
                    device,
                    serial_number,
                    bt_path,
                    usb_path,
                    bt_address,
                    connection_type,
                ))
            }
            Err(err) => {
                error!("Couldn't open '{path}'. Caused by {err}");
                None
            }
        }
    }

    fn get_bt_address_on_windows(&self, path_str: &str) -> String {
        trace!("Getting bluetooth address by special device, due to Windows.");

        let magic_bt_path = path_str
            .replace(MAGIC_PATH, WINDOWS_BLUETOOTH_MAGIC_PATH)
            .replace("&0000#", "&0001#");

        match self
            .hid
            .open_path(&CString::new(magic_bt_path).unwrap())
        {
            Ok(special_bt_device) => {
                trace!("Got special device for bluetooth.");
                Self::get_bt_address(&special_bt_device).unwrap_or_else(|| String::from(""))
            }
            Err(err) => {
                error!("Couldn't open device. Caused by: {err}");
                String::from("")
            }
        }
    }

    fn is_move_controller(dev_info: &DeviceInfo) -> bool {
        let path = dev_info.path().to_str();

        let path = match path {
            Ok(path) => path,
            Err(_) => return false,
        };
        let vendor_id = dev_info.vendor_id();
        let product_id = dev_info.product_id();

        if cfg!(windows) && !path.contains(MAGIC_PATH) {
            return false;
        }

        vendor_id == PS_MOVE_VENDOR_ID && product_id == PS_MOVE_PRODUCT_ID
    }

    fn get_bt_address(device: &HidDevice) -> Option<String> {
        let mut bt_addr_report = build_get_bt_addr_request();

        let report_status = device.get_feature_report(&mut bt_addr_report);

        match report_status {
            Ok(_) => {
                let addr = &bt_addr_report[1..7];
                let addr = format!(
                    "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    addr[5], addr[4], addr[3], addr[2], addr[1], addr[0]
                );

                trace!("Got bluetooth address {}", addr);

                Some(addr)
            }
            Err(err) => {
                error!("Failed to get bt address {err}");
                None
            }
        }
    }
}

fn build_get_bt_addr_request() -> [u8; PS_MOVE_BT_ADDR_GET_SIZE] {
    let mut request = [0; PS_MOVE_BT_ADDR_GET_SIZE];

    request[0] = MoveRequestType::GetBluetoothAddr as u8;

    request
}

pub fn build_hsv(h: f64, s: f64, v: f64) -> Hsv {
    Hsv::from_components((h as f32, s as f32, v as f32))
}
