use std::ffi::{CStr, CString};
use std::str;

use hidapi::{DeviceInfo, HidApi, HidDevice};
use log::{error, info, trace};
use palette::Hsv;

use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::{PsMoveConnectionType, PsMoveRequestType};

const MAGIC_PATH: &str = "&Col01#";
const WINDOWS_BLUETOOTH_MAGIC_PATH: &str = "&Col02#";
const PS_MOVE_VENDOR_ID: u16 = 0x054c;
const PS_MOVE_PRODUCT_ID: u16 = 0x03d5;

const PS_MOVE_BT_ADDR_GET_SIZE: usize = 16;

pub struct PsMoveApi {
    hid: HidApi,
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

    /// Returns the current devices, found in the last [`Self::refresh()`] call.
    /// (the refresh is expensive)
    pub fn list(
        &mut self,
        current_controllers: &mut Vec<Box<PsMoveController>>,
    ) -> Vec<Box<PsMoveController>> {
        let mut controllers: Vec<&DeviceInfo> = self
            .hid
            .device_list()
            .filter(|dev_info| Self::is_move_controller(dev_info))
            .collect();

        Self::remove_disconnected(current_controllers, &mut controllers);

        let controllers = self.connect_new_controllers(current_controllers, &mut controllers);

        controllers
    }

    fn remove_disconnected<'a>(
        current_controllers: &mut Vec<Box<PsMoveController>>,
        controllers: &mut Vec<&DeviceInfo>,
    ) {
        current_controllers.retain(|controller| {
            let is_connected = controllers.iter().any(|dev_info| {
                let path = dev_info.path().to_str().unwrap();

                controller.has_same_path(path)
            });

            if !is_connected {
                info!(
                    "Controller disconnected. ('{}' by {})",
                    controller.bt_address, controller.connection_type
                )
            }

            is_connected
        });
    }

    fn connect_new_controllers(
        &self,
        current_controllers: &mut Vec<Box<PsMoveController>>,
        controllers: &mut Vec<&DeviceInfo>,
    ) -> Vec<Box<PsMoveController>> {
        controllers
            .iter()
            .map(|dev_info| {
                let serial_number = CString::new(dev_info.serial_number().unwrap_or("")).unwrap();

                self.connect_controller(&serial_number, dev_info.path())
            })
            .flatten()
            .fold(Vec::<Box<PsMoveController>>::new(), |res, curr| {
                self.merge_usb_with_bt_device(res, curr)
            })
    }

    fn connect_controller(
        &self,
        serial_number: &CStr,
        path: &CStr,
    ) -> Option<Box<PsMoveController>> {
        let path_str = String::from(path.to_str().unwrap());
        let mut bt_path = String::new();
        let mut usb_path = String::new();
        let mut address = String::from(serial_number.to_str().unwrap_or(""));

        let device = if address.is_empty() && !path_str.is_empty() {
            self.hid.open_path(path)
        } else {
            self.hid
                .open_serial(PS_MOVE_VENDOR_ID, PS_MOVE_PRODUCT_ID, &*address)
        };

        match device {
            Ok(device) => {
                match device.set_blocking_mode(false) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Unable to set {} to nonblocking {}", address, err);
                        return None;
                    }
                }

                let connection_type;

                if address.is_empty() {
                    connection_type = PsMoveConnectionType::USB;
                    usb_path = path_str.clone();

                    if cfg!(windows) {
                        trace!("Getting bluetooth address by special device, due to Windows.");

                        let magic_bt_path = path_str
                            .clone()
                            .replace(MAGIC_PATH, WINDOWS_BLUETOOTH_MAGIC_PATH)
                            .replace("&0000#", "&0001#");

                        match self
                            .hid
                            .open_path(&*CString::new(magic_bt_path.clone()).unwrap())
                        {
                            Ok(special_bt_device) => {
                                trace!("Got special device for bluetooth.");
                                address = Self::get_bt_address(&special_bt_device)
                                    .unwrap_or(String::from(""));
                            }
                            Err(err) => error!("Couldn't open device. Caused by: {}", err),
                        }
                    } else {
                        address = Self::get_bt_address(&device).unwrap_or(String::from(""));
                    }
                } else {
                    connection_type = PsMoveConnectionType::Bluetooth;
                    bt_path = path_str.clone();
                }

                Some(Box::new(PsMoveController::new(
                    device,
                    bt_path,
                    usb_path,
                    address,
                    connection_type,
                )))
            }
            Err(err) => {
                error!("Couldn't open '{}'. Caused by {}", path_str, err);
                None
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

    fn merge_usb_with_bt_device(
        &self,
        mut res: Vec<Box<PsMoveController>>,
        curr: Box<PsMoveController>,
    ) -> Vec<Box<PsMoveController>> {
        let dupe = res.iter_mut().find(|controller| {
            controller.bt_address == curr.bt_address
        });

        match dupe {
            None => res.push(curr),
            Some(dupe) => {
                if curr.connection_type == PsMoveConnectionType::USB {
                    dupe.usb_path = curr.usb_path;
                } else {
                    dupe.bt_path = curr.bt_path;
                }
                dupe.connection_type = PsMoveConnectionType::USBAndBluetooth;
            }
        }
        res
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
                error!("Failed to get bt address {}", err);
                None
            }
        }
    }
}

fn build_get_bt_addr_request() -> [u8; PS_MOVE_BT_ADDR_GET_SIZE] {
    let mut request = [0; PS_MOVE_BT_ADDR_GET_SIZE];

    request[0] = PsMoveRequestType::GetBluetoothAddr as u8;

    return request;
}

pub fn build_hsv(h: f64, s: f64, v: f64) -> Hsv {
    Hsv::from_components((h as f32, s as f32, v as f32))
}
