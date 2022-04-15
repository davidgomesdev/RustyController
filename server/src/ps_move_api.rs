use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::str;

use actix_web::connect;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidResult};
use log::{debug, error, info};
use palette::{FromColor, Hsv, Hue, Srgb};
use strum_macros::Display;

const MAGIC_PATH: &str = "&col01#";
const PS_MOVE_VENDOR_ID: u16 = 0x054c;
const PS_MOVE_PRODUCT_ID: u16 = 0x03d5;

const PS_MOVE_BT_ADDR_GET_SIZE: usize = 16;

pub const MIN_LED_PWM_FREQUENCY: u64 = 733;
pub const MAX_LED_PWM_FREQUENCY: u64 = 0x24e6;

enum PsMoveRequestType {
    GetInput = 0x01,
    SetLED = 0x06,
    SetLEDPWMFrequency = 0x03,
    GetBluetoothAddr = 0x04,
    BluetoothAddr = 0x05,
    GetCalibration = 0x10,
    SetAuthChallenge = 0xA0,
    GetAuthResponse = 0xA1,
    GetExtDeviceInfo = 0xE0,
    SetDFUMode = 0xF2,
    GetFirmwareInfo = 0xF9,
}

#[derive(Clone, Copy, strum_macros::Display)]
pub enum LedEffect {
    Off,
    Static {
        hsv: Hsv,
    },
    Breathing {
        initial_hsv: Hsv,
        step: f32,
        peak: f32,
        inhaling: bool,
    },
    Rainbow {
        saturation: f32,
        value: f32,
        step: f32,
    },
}

pub struct PsMoveApi {
    hid: HidApi,
}

impl PsMoveApi {
    pub fn new() -> PsMoveApi {
        PsMoveApi {
            hid: HidApi::new().unwrap_or_else(|_| panic!("Couldn't init hidapi")),
        }
    }

    pub fn list(&mut self) -> Vec<Box<PsMoveController>> {
        let result = self.hid.refresh_devices();
        if result.is_err() {
            error!("{}", result.unwrap_err());
            return Vec::new();
        }

        let controllers = self
            .hid
            .device_list();

        let controllers = controllers
            .filter(|dev_info| self.is_move_controller(dev_info))
            .map(|dev_info| {
                let serial_number =
                    CString::new(dev_info.serial_number().unwrap_or("")).unwrap();

                self.connect_controller(&serial_number, dev_info.path())
            })
            .flatten()
            .fold(Vec::<Box<PsMoveController>>::new(), |mut res, curr| {
                self.merge_usb_with_bt_device(res, curr)
            });

        return controllers;
    }

    fn is_move_controller(&self, dev_info: &DeviceInfo) -> bool {
        let path = dev_info.path().to_str();

        let path = match path {
            Ok(path) => path,
            Err(_) => return false
        };
        let vendor_id = dev_info.vendor_id();
        let product_id = dev_info.product_id();

        if cfg!(windows) && !path.to_lowercase().contains(MAGIC_PATH) {
            return false;
        }

        vendor_id == PS_MOVE_VENDOR_ID && product_id == PS_MOVE_PRODUCT_ID
    }

    fn merge_usb_with_bt_device(&self, mut res: Vec<Box<PsMoveController>>, curr: Box<PsMoveController>) -> Vec<Box<PsMoveController>> {
        let dupe = res.iter_mut().find(|controller| {
            controller.bt_address == curr.bt_address
        });

        if dupe.is_some() {
            dupe.unwrap().connection_type = PsMoveConnectionType::USBAndBluetooth
        } else {
            res.push(curr);
        }
        res
    }

    fn connect_controller(&self, serial_number: &CStr, path: &CStr) -> Option<Box<PsMoveController>> {
        let device = self.hid.open_path(path);
        let mut address = String::from(serial_number.to_str().unwrap_or(""));

        if device.is_err() {
            return None;
        }

        let device = device.unwrap();

        let mut connection_type = PsMoveConnectionType::Unknown;

        if address.is_empty() {
            connection_type = PsMoveConnectionType::USB;
            address = self.get_usb_address(path).unwrap_or(String::from(""))
        } else {
            connection_type = PsMoveConnectionType::Bluetooth;
        }

        return Some(Box::new(PsMoveController::new(
            device,
            address,
            connection_type,
        )))
    }

    fn get_usb_address(&self, path: &CStr) -> Option<String> {
        let mut usb_path = String::from(path.to_str().unwrap())
            .replace("Col01", "Col02")
            .replace("&0000#", "&0001#");

        let usb_path = CString::new(usb_path).unwrap();
        let addr_device = self.hid.open_path(usb_path.as_c_str());

        if addr_device.is_ok() {
            let addr = self.get_bt_address(addr_device.unwrap());

            Some(addr.unwrap_or(String::from("")))
        } else {
            error!("Failed to open addr device {}", addr_device.err().unwrap());
            None
        }
    }

    fn get_bt_address(&self, device: HidDevice) -> Option<String> {
        let mut bt_addr_report = build_get_bt_addr_request();

        let report_status = device.get_feature_report(&mut bt_addr_report);

        if report_status.is_ok() {
            let addr = &bt_addr_report[1..7];
            let addr = format!(
                "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                addr[5], addr[4], addr[3], addr[2], addr[1], addr[0]);

            Some(addr)
        } else {
            error!("{}", report_status.unwrap_err());
            None
        }
    }
}

pub struct PsMoveController {
    device: HidDevice,
    pub bt_address: String,
    effect: LedEffect,
    current_setting: PsMoveSetting,
    connection_type: PsMoveConnectionType,
}

pub enum PsMoveConnectionType {
    Unknown,
    USB,
    Bluetooth,
    USBAndBluetooth,
}

#[derive(Clone)]
pub struct PsMoveSetting {
    led: Hsv,
    rumble: f32,
}

impl PsMoveController {
    fn new(device: HidDevice, serial_number: String, connection_type: PsMoveConnectionType) -> PsMoveController {
        PsMoveController {
            device,
            bt_address: serial_number,
            effect: LedEffect::Off,
            current_setting: PsMoveSetting {
                led: Hsv::from_components((0.0, 0.0, 0.0)),
                rumble: 0.0,
            },
            connection_type,
        }
    }

    pub fn set_led_pwm_frequency(&self, frequency: u64) -> bool {
        let request = build_set_led_pwm_request(frequency);
        let is_ok = self.device.write(&request).is_ok();

        return is_ok;
    }

    pub fn set_led_effect(&mut self, effect: LedEffect) -> () {
        self.current_setting.led = match effect {
            LedEffect::Off => Hsv::from_components((0.0, 0.0, 0.0)),
            LedEffect::Static { hsv } => hsv,
            LedEffect::Breathing {
                initial_hsv,
                step,
                peak,
                inhaling: _,
            } => {
                if peak <= initial_hsv.value {
                    error!("Peak must be higher than initial value")
                }
                if step >= peak {
                    error!("Step must be lower than peak")
                }

                initial_hsv
            }
            LedEffect::Rainbow {
                saturation,
                value,
                step: _,
            } => Hsv::from_components((0.0, saturation, value)),
        };
        self.effect = effect
    }

    pub fn update(&mut self) -> bool {
        let new_hsv = self.transform_led();

        if !self.set_hsv(new_hsv) {
            return false;
        }

        if !self.set_rumble(self.current_setting.rumble) {
            return false;
        }

        return true;
    }

    pub fn copy_settings(&mut self, controller: &Self) {
        self.current_setting = controller.current_setting.clone();
        self.effect = controller.effect;
    }

    fn transform_led(&mut self) -> Hsv {
        let effect = &mut self.effect;
        let current_hsv = self.current_setting.led;

        match *effect {
            LedEffect::Off => Hsv::from_components((0.0, 0.0, 0.0)),
            LedEffect::Static { hsv } => hsv,
            LedEffect::Breathing {
                initial_hsv,
                step,
                peak,
                ref mut inhaling,
            } => {
                let initial_value = initial_hsv.value;

                let mut new_hsv = current_hsv.clone();
                let mut new_value = new_hsv.value;

                if *inhaling {
                    new_value += step
                } else {
                    new_value -= step
                }

                if new_value >= peak {
                    new_value = peak;
                    *inhaling = false
                } else if new_value <= initial_value {
                    new_value = initial_value;
                    *inhaling = true
                }

                new_hsv.value = new_value;
                new_hsv
            }
            LedEffect::Rainbow {
                saturation: _,
                value: _,
                step,
            } => {
                // no need to use [saturation] and [value], since it's already when setting effect
                current_hsv.shift_hue(step)
            }
        }
    }

    fn set_hsv(&mut self, hsv: Hsv) -> bool {
        let setting = &mut self.current_setting;
        let request = build_set_led_and_rumble_request(hsv, setting.rumble);

        let res = self.device.write(&request);

        if res.is_err() {
            error!("Error setting HSV {}", res.unwrap_err());
            return false;
        }

        setting.led = hsv;
        return true;
    }

    pub fn set_rumble(&mut self, rumble: f32) -> bool {
        let setting = &mut self.current_setting;
        let request = build_set_led_and_rumble_request(setting.led, rumble);

        let res = self.device.write(&request);

        if res.is_err() {
            error!("Error setting HSV {}", res.unwrap_err());
            return false;
        }

        setting.rumble = rumble;
        return true;
    }
}

fn build_set_led_pwm_request(frequency: u64) -> [u8; 7] {
    if frequency < MIN_LED_PWM_FREQUENCY || frequency > MAX_LED_PWM_FREQUENCY {
        panic!("Frequency must be between 733 and 24e6!")
    }

    return [
        PsMoveRequestType::SetLEDPWMFrequency as u8,
        0x41,
        0,
        (frequency & 0xFF) as u8,
        ((frequency >> 8) & 0xFF) as u8,
        ((frequency >> 16) & 0xFF) as u8,
        ((frequency >> 24) & 0xFF) as u8,
    ];
}

fn build_set_led_and_rumble_request(hsv: Hsv, rumble: f32) -> [u8; 8] {
    let rgb = Srgb::from_color(hsv);
    let f32_to_u8 = |f| (f * 255.0) as u8;
    let rgb = [rgb.red, rgb.green, rgb.blue].map(f32_to_u8);

    return [
        PsMoveRequestType::SetLED as u8,
        0,
        rgb[0],
        rgb[1],
        rgb[2],
        0,
        f32_to_u8(rumble),
        0,
    ];
}

fn build_get_bt_addr_request() -> [u8; PS_MOVE_BT_ADDR_GET_SIZE] {
    let mut request = [0; PS_MOVE_BT_ADDR_GET_SIZE];

    request[0] = PsMoveRequestType::GetBluetoothAddr as u8;

    return request;
}

pub fn build_hsv(h: f64, s: f64, v: f64) -> Hsv {
    Hsv::from_components((h as f32, s as f32, v as f32))
}
