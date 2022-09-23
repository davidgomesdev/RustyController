use hidapi::HidDevice;
use log::{error, info};
use palette::{FromColor, Hsv, Hue, Srgb};

use crate::LedEffect;
use crate::ps_move::models::{PsMoveBatteryLevel, PsMoveConnectionType, PsMoveDataInput, PsMoveRequestType, PsMoveSetting};
use crate::ps_move::models::PsMoveBatteryLevel::Unknown;

pub const MIN_LED_PWM_FREQUENCY: u64 = 0x02dd;
pub const MAX_LED_PWM_FREQUENCY: u64 = 0x24e6;

pub struct PsMoveController {
    device: HidDevice,
    pub(super) bt_path: String,
    pub(super) usb_path: String,
    pub bt_address: String,
    pub effect: LedEffect,
    pub setting: PsMoveSetting,
    pub connection_type: PsMoveConnectionType,
    pub battery: PsMoveBatteryLevel,
}

impl PsMoveController {
    pub(super) fn new(
        device: HidDevice,
        bt_path: String,
        usb_path: String,
        serial_number: String,
        connection_type: PsMoveConnectionType,
    ) -> PsMoveController {
        PsMoveController {
            device,
            bt_path,
            usb_path,
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
                if step < 0.0 || step > 1.0 {
                    error!("Step must be between 0.0 and 1.0")
                }

                if peak < initial_hsv.value {
                    error!("Peak must be higher than initial value")
                }

                initial_hsv
            }
            LedEffect::Rainbow {
                saturation,
                value,
                step,
            } => {
                if step > 360.0 {
                    error!("Step can't be higher than 360 (max hue)")
                }

                Hsv::from_components((0.0, saturation, value))
            }
        };
        self.effect = effect
    }

    #[allow(dead_code)]
    pub fn set_rumble(&mut self, rumble: f32) -> bool {
        if rumble < 0.0 || rumble > 1.0 {
            false
        } else {
            self.setting.rumble = rumble;
            true
        }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        if self.update_hsv_and_rumble().is_err() {
            return Err(());
        }

        let mut data = [0 as u8; 44];

        if self.device.read(&mut data).is_ok() {
            if data[0] == PsMoveRequestType::GetInput as u8 {
                let data = PsMoveDataInput::new(data);

                self.update_battery(data.battery);
            }
        }

        return Ok(());
    }

    pub fn transform_led(&mut self) {
        let effect = &mut self.effect;
        let current_hsv = self.setting.led;

        self.setting.led = match *effect {
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
                    new_value += step * peak
                } else {
                    new_value -= step * peak
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
                // no need to use [saturation] and [value], since it was already set in the beginning
                // similar to breathing, the step is relative to the max possible value
                current_hsv.shift_hue(step * 360.0)
            }
        }
    }

    fn update_hsv_and_rumble(&self) -> Result<(), ()> {
        let request = build_set_led_and_rumble_request(self.setting.led, self.setting.rumble);

        let res = self.device.write(&request);

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to set HSV {}", err);
                Err(())
            }
        }
    }

    fn update_battery(&mut self, battery: u8) {
        let curr_battery = PsMoveBatteryLevel::from_byte(battery);
        let last_battery = &self.battery;

        if curr_battery != *last_battery {
            if *last_battery == Unknown {
                info!("Controller battery status known. ('{}' at {})", self.bt_address, curr_battery);
            } else {
                info!(
                    "Controller battery status changed. ('{}' to {})",
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
    let f32_to_u8 = |f: f32| (f * 255.0) as u8;
    let rgb = hsv_to_rgb(hsv, f32_to_u8);

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

fn hsv_to_rgb(hsv: Hsv, f32_to_u8: fn(f: f32) -> u8) -> [u8; 3] {
    let rgb = Srgb::from_color(hsv);
    [rgb.red, rgb.green, rgb.blue].map(f32_to_u8)
}