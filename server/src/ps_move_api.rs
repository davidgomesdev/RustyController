use std::borrow::Borrow;
use std::fmt;

use hidapi::{HidApi, HidDevice, HidResult};
use log::error;
use palette::{FromColor, Hsv, Hue, Srgb};
use strum_macros::Display;

const MAGIC_PATH: &str = "&col01#";
const PSMOVE_VENDOR_ID: u16 = 0x054c;
const PSMOVE_PRODUCT_ID: u16 = 0x03d5;

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

    pub fn list(&mut self) -> Vec<PsMoveController> {
        if self.hid.refresh_devices().is_err() {
            error!("ERROR: HID devices refresh");
            return Vec::new();
        }

        let controllers: Vec<PsMoveController> = self
            .hid
            .device_list()
            .filter(|device| -> bool {
                let path = device.path().to_str();

                let path = match path {
                    Ok(path) => path,
                    Err(_) => return false
                };
                let vendor_id = device.vendor_id();
                let product_id = device.product_id();

                path.to_lowercase().contains(MAGIC_PATH)
                    && vendor_id == PSMOVE_VENDOR_ID
                    && product_id == PSMOVE_PRODUCT_ID
            })
            .map(|dev_info| self.hid.open_path(dev_info.path()))
            .filter_map(|dev| dev.ok())
            .map(|dev|
                PsMoveController::new(dev))
            .collect();

        return controllers;
    }
}

pub struct PsMoveController {
    pub hid_device: HidDevice,
    pub serial_number: String,
    effect: LedEffect,
    current_setting: PsMoveSetting,
}

#[derive(Clone)]
pub struct PsMoveSetting {
    led: Hsv,
    rumble: f32,
}

impl PsMoveController {
    fn new(hid_device: HidDevice) -> PsMoveController {
        let no_serial_number = String::from("");
        let serial_number = hid_device.get_serial_number_string()
            .unwrap_or(Option::Some(no_serial_number.clone()))
            .unwrap_or(no_serial_number);

        PsMoveController {
            hid_device,
            serial_number,
            effect: LedEffect::Off,
            current_setting: PsMoveSetting {
                led: Hsv::from_components((0.0, 0.0, 0.0)),
                rumble: 0.0,
            },
        }
    }

    pub fn set_led_pwm_frequency(&self, frequency: u64) -> bool {
        let request = build_set_led_pwm_request(frequency);
        let is_ok = self.hid_device.write(&request).is_ok();

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

        let is_ok = self.hid_device.write(&request).is_ok();

        if is_ok {
            setting.led = hsv;
        }

        return is_ok;
    }

    pub fn set_rumble(&mut self, rumble: f32) -> bool {
        let setting = &mut self.current_setting;
        let request = build_set_led_and_rumble_request(setting.led, rumble);

        let is_ok = self.hid_device.write(&request).is_ok();

        if is_ok {
            setting.rumble = rumble;
        }

        return is_ok;
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

pub fn build_hsv(h: f64, s: f64, v: f64) -> Hsv {
    Hsv::from_components((h as f32, s as f32, v as f32))
}
