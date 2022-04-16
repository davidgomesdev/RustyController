use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::str;

use actix_web::connect;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidResult};
use log::{debug, error, info};
use palette::{FromColor, Hsv, Hue, Srgb};
use strum_macros::Display;

use crate::ps_move_api::PsMoveBatteryLevel::{
    Charged, Charging, EightyPercent, Empty, FortyPercent, Full, SixtyPercent, TwentyPercent,
    Unknown,
};

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
            error!("Failed to refresh devices {}", result.unwrap_err());
            return Vec::new();
        }

        let controllers = self.hid.device_list();

        let controllers = controllers
            .filter(|dev_info| self.is_move_controller(dev_info))
            .map(|dev_info| {
                let serial_number = CString::new(dev_info.serial_number().unwrap_or("")).unwrap();

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
            Err(_) => return false,
        };
        let vendor_id = dev_info.vendor_id();
        let product_id = dev_info.product_id();

        if cfg!(windows) && !path.to_lowercase().contains(MAGIC_PATH) {
            return false;
        }

        vendor_id == PS_MOVE_VENDOR_ID && product_id == PS_MOVE_PRODUCT_ID
    }

    fn merge_usb_with_bt_device(
        &self,
        mut res: Vec<Box<PsMoveController>>,
        curr: Box<PsMoveController>,
    ) -> Vec<Box<PsMoveController>> {
        let dupe = res
            .iter_mut()
            .find(|controller| controller.bt_address == curr.bt_address);

        match dupe {
            None => res.push(curr),
            Some(dupe) => dupe.connection_type = PsMoveConnectionType::USBAndBluetooth,
        }
        res
    }

    fn connect_controller(
        &self,
        serial_number: &CStr,
        path: &CStr,
    ) -> Option<Box<PsMoveController>> {
        let device = self.hid.open_path(path);
        let mut address = String::from(serial_number.to_str().unwrap_or(""));

        match device {
            Ok(device) => {
                match device.set_blocking_mode(false) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Unable to set {} to nonblocking {}", address, err);
                        return None;
                    }
                }

                let mut connection_type = PsMoveConnectionType::Unknown;

                if address.is_empty() {
                    connection_type = PsMoveConnectionType::USB;
                    address = self.get_usb_address(path).unwrap_or(String::from(""))
                } else {
                    connection_type = PsMoveConnectionType::Bluetooth;
                }

                Some(Box::new(PsMoveController::new(
                    device,
                    address,
                    connection_type,
                )))
            }
            Err(err) => {
                error!("Couldn't open {} {}", address, err);
                None
            }
        }
    }

    fn get_usb_address(&self, path: &CStr) -> Option<String> {
        let mut usb_path = String::from(path.to_str().unwrap())
            .replace("Col01", "Col02")
            .replace("&0000#", "&0001#");

        let usb_path = CString::new(usb_path).unwrap();
        let addr_device = self.hid.open_path(usb_path.as_c_str());

        match addr_device {
            Ok(addr_device) => {
                let addr = self.get_bt_address(addr_device);

                Some(addr.unwrap_or(String::from("")))
            }
            Err(err) => {
                error!("Failed to open addr device {}", err);
                None
            }
        }
    }

    fn get_bt_address(&self, device: HidDevice) -> Option<String> {
        let mut bt_addr_report = build_get_bt_addr_request();

        let report_status = device.get_feature_report(&mut bt_addr_report);

        match report_status {
            Ok(_) => {
                let addr = &bt_addr_report[1..7];
                let addr = format!(
                    "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    addr[5], addr[4], addr[3], addr[2], addr[1], addr[0]
                );

                Some(addr)
            }
            Err(err) => {
                error!("Failed to get bt address {}", err);
                None
            }
        }
    }
}

pub struct PsMoveController {
    device: HidDevice,
    pub bt_address: String,
    pub effect: LedEffect,
    pub setting: PsMoveSetting,
    pub connection_type: PsMoveConnectionType,
    pub battery: PsMoveBatteryLevel,
}

#[derive(Clone)]
pub struct PsMoveSetting {
    led: Hsv,
    rumble: f32,
}

#[derive(strum_macros::Display, PartialEq)]
pub enum PsMoveConnectionType {
    Unknown,
    USB,
    Bluetooth,
    USBAndBluetooth,
}

#[derive(strum_macros::Display, PartialEq)]
pub enum PsMoveBatteryLevel {
    Unknown,
    Empty,
    TwentyPercent,
    FortyPercent,
    SixtyPercent,
    EightyPercent,
    Full,
    Charging,
    Charged,
}

impl PsMoveBatteryLevel {
    fn from_code(code: u8) -> PsMoveBatteryLevel {
        match code {
            0x00 => Empty,
            0x01 => TwentyPercent,
            0x02 => FortyPercent,
            0x03 => SixtyPercent,
            0x04 => EightyPercent,
            0x05 => Full,
            0xEE => Charging,
            0xEF => Charged,
            _ => Unknown,
        }
    }
}

impl PsMoveController {
    fn new(
        device: HidDevice,
        serial_number: String,
        connection_type: PsMoveConnectionType,
    ) -> PsMoveController {
        PsMoveController {
            device,
            bt_address: serial_number,
            effect: LedEffect::Off,
            setting: PsMoveSetting {
                led: Hsv::from_components((0.0, 0.0, 0.0)),
                rumble: 0.0,
            },
            connection_type,
            battery: Unknown,
        }
    }

    pub fn set_led_pwm_frequency(&self, frequency: u64) -> bool {
        let request = build_set_led_pwm_request(frequency);
        let is_ok = self.device.write(&request).is_ok();

        return is_ok;
    }

    pub fn set_led_effect(&mut self, effect: LedEffect) -> () {
        self.setting.led = match effect {
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

    pub fn set_rumble(&mut self, rumble: f32) -> bool {
        let setting = &mut self.setting;
        let request = build_set_led_and_rumble_request(setting.led, rumble);

        let res = self.device.write(&request);

        return match res {
            Ok(_) => {
                setting.rumble = rumble;
                true
            }
            Err(err) => {
                error!("Failed to set rumble {}", err);
                false
            }
        };
    }

    pub fn update(&mut self) -> bool {
        let new_hsv = self.transform_led();

        if !self.set_hsv(new_hsv) {
            return false;
        }

        if !self.set_rumble(self.setting.rumble) {
            return false;
        }

        let mut data = [0 as u8; 44];

        if self.device.read(&mut data).is_ok() {
            if data[0] == PsMoveRequestType::GetInput as u8 {
                let data = PsMoveDataInput::new(data);

                self.update_data(data);
            }
        }

        return true;
    }

    fn transform_led(&mut self) -> Hsv {
        let effect = &mut self.effect;
        let current_hsv = self.setting.led;

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
        let setting = &mut self.setting;
        let request = build_set_led_and_rumble_request(hsv, setting.rumble);

        let res = self.device.write(&request);

        match res {
            Ok(_) => {
                setting.led = hsv;
                true
            }
            Err(err) => {
                error!("Failed to set HSV {}", err);
                false
            }
        }
    }

    fn update_data(&mut self, data: PsMoveDataInput) {
        let curr_battery = PsMoveBatteryLevel::from_code(data.battery);

        if curr_battery != self.battery {
            if self.battery == Unknown {
                info!("Battery status of {} is {}", self.bt_address, curr_battery);
            } else {
                info!(
                    "Battery status of {} changed to {}",
                    self.bt_address, curr_battery
                );
            }
            self.battery = curr_battery;
        }
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

/// Adapted from [psmoveapi's source](https://github.com/thp/psmoveapi/blob/master/src/psmove.c)
struct PsMoveDataInput {
    // message type, must be PSMove_Req_GetInput
    msg_type: u8,
    buttons1: u8,
    buttons2: u8,
    buttons3: u8,
    buttons4: u8,
    // trigger value: u8, 0..255
    trigger: u8,
    // trigger value, 2nd frame
    trigger2: u8,
    _unk7: u8,
    _unk8: u8,
    _unk9: u8,
    _unk10: u8,
    // high byte of timestamp
    time_high: u8,
    // battery level: u8, 0x05 = max, 0xEE = USB charging
    battery: u8,
    // low byte of accelerometer X value
    accel_x_low: u8,
    // high byte of accelerometer X value
    accel_x_high: u8,
    accel_y_low: u8,
    accel_y_high: u8,
    accel_z_low: u8,
    accel_z_high: u8,
    // low byte of accelerometer X value, 2nd frame
    accel_x_low2: u8,
    // high byte of accelerometer X value, 2nd frame
    accel_x_high2: u8,
    accel_y_low2: u8,
    accel_y_high2: u8,
    accel_z_low2: u8,
    accel_z_high2: u8,
    // low byte of gyro X value
    gyro_x_low: u8,
    // high byte of gyro X value
    gyro_x_high: u8,
    gyro_y_low: u8,
    gyro_y_high: u8,
    gyro_z_low: u8,
    gyro_z_high: u8,
    // low byte of gyro X value, 2nd frame
    gyro_x_low2: u8,
    // high byte of gyro X value, 2nd frame
    gyro_x_high2: u8,
    gyro_y_low2: u8,
    gyro_y_high2: u8,
    gyro_z_low2: u8,
    gyro_z_high2: u8,
    // temperature (bits 12-5)
    temp_high: u8,
    // temp (bits 4-1): u8, magneto X (bits 12-9)
    temp_low_magneto_x_high: u8,
    // magnetometer X (bits 8-1)
    magneto_x_low: u8,
    // magnetometer Y (bits 12-5)
    magneto_y_high: u8,
    // magnetometer: Y (bits 4-1), Z (bits 12-9)
    magneto_y_low_magneto_z_high: u8,
    // magnetometer Z (bits 8-1)
    magneto_z_low: u8,
    // low byte of timestamp
    time_low: u8,
}

impl PsMoveDataInput {
    fn new(req: [u8; 44]) -> PsMoveDataInput {
        PsMoveDataInput {
            msg_type: req[0],
            buttons1: req[1],
            buttons2: req[2],
            buttons3: req[3],
            buttons4: req[4],
            trigger: req[5],
            trigger2: req[6],
            _unk7: req[7],
            _unk8: req[8],
            _unk9: req[9],
            _unk10: req[10],
            time_high: req[11],
            battery: req[12],
            accel_x_low: req[13],
            accel_x_high: req[14],
            accel_y_low: req[15],
            accel_y_high: req[16],
            accel_z_low: req[17],
            accel_z_high: req[18],
            accel_x_low2: req[19],
            accel_x_high2: req[20],
            accel_y_low2: req[21],
            accel_y_high2: req[22],
            accel_z_low2: req[23],
            accel_z_high2: req[24],
            gyro_x_low: req[25],
            gyro_x_high: req[26],
            gyro_y_low: req[27],
            gyro_y_high: req[28],
            gyro_z_low: req[29],
            gyro_z_high: req[30],
            gyro_x_low2: req[31],
            gyro_x_high2: req[32],
            gyro_y_low2: req[33],
            gyro_y_high2: req[34],
            gyro_z_low2: req[35],
            gyro_z_high2: req[36],
            temp_high: req[37],
            temp_low_magneto_x_high: req[38],
            magneto_x_low: req[39],
            magneto_y_high: req[40],
            magneto_y_low_magneto_z_high: req[41],
            magneto_z_low: req[42],
            time_low: req[43],
        }
    }
}
