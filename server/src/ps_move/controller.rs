use hidapi::{HidDevice, HidError};
use log::{debug, error, info};
use palette::{FromColor, Hsv, Srgb};

use crate::LedEffect;
use crate::ps_move::models::{
    BatteryLevel, ConnectionType, ControllerInfo, DataInput, MoveRequestType, MoveSetting,
    RumbleEffect,
};
use crate::ps_move::models::BatteryLevel::Unknown;

pub const MIN_LED_PWM_FREQUENCY: u64 = 0x02dd;
pub const MAX_LED_PWM_FREQUENCY: u64 = 0x24e6;

pub struct PsMoveController {
    device: HidDevice,
    pub(super) info: ControllerInfo,
    pub bt_address: String,
    pub led_effect: LedEffect,
    pub rumble_effect: RumbleEffect,
    pub setting: MoveSetting,
    pub battery: BatteryLevel,
    pub connection_type: ConnectionType,
}

impl PsMoveController {
    pub(super) fn new(
        device: HidDevice,
        serial_number: &str,
        bt_path: String,
        usb_path: String,
        bt_address: String,
        connection_type: ConnectionType,
    ) -> PsMoveController {
        let info = ControllerInfo::new(String::from(serial_number), bt_path, usb_path);

        PsMoveController {
            device,
            info,
            bt_address,
            led_effect: LedEffect::Off,
            rumble_effect: RumbleEffect::Off,
            setting: MoveSetting {
                led: Hsv::from_components((0.0, 0.0, 0.0)),
                rumble: 0.0,
            },
            connection_type,
            battery: Unknown,
        }
    }

    pub fn is_same_device(&self, info: &ControllerInfo) -> bool {
        match self.connection_type {
            ConnectionType::USB => self.info.usb_path == info.usb_path,
            ConnectionType::Bluetooth => self.info.bt_path == info.bt_path,
            ConnectionType::USBAndBluetooth => {
                self.info.usb_path == info.usb_path || self.info.bt_path == info.bt_path
            }
        }
    }

    /// Merges a USB device with a Bluetooth one (or vice-versa)
    ///
    /// Updating the connection type.
    pub fn merge_with(&mut self, other: &PsMoveController) {
        if self.connection_type == other.connection_type {
            panic!("Both controllers are connected the same way! Nothing to merge.")
        }

        if self.connection_type == ConnectionType::USB {
            self.info.bt_path = other.info.bt_path.clone();
        } else if self.connection_type == ConnectionType::Bluetooth {
            self.info.usb_path = other.info.usb_path.clone();
        }
        self.connection_type = ConnectionType::USBAndBluetooth;
    }

    pub fn set_led_pwm_frequency(&self, frequency: u64) -> bool {
        let request = build_set_led_pwm_request(frequency);
        let is_ok = self.device.write(&request).is_ok();

        return is_ok;
    }

    pub fn set_led_effect(&mut self, effect: LedEffect) {
        self.setting.led = match effect {
            LedEffect::Off => Hsv::from_components((0.0, 0.0, 0.0)),
            LedEffect::Static { hsv }
            | LedEffect::Blink {
                hsv,
                interval: _,
                start: _,
            } => hsv,
            LedEffect::Breathing {
                initial_hsv,
                step,
                peak,
                ..
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
        self.led_effect = effect
    }

    pub fn set_rumble_effect(&mut self, effect: RumbleEffect) {
        match effect {
            RumbleEffect::Off => {}
            RumbleEffect::Static { strength } => {
                if strength < 0.0 || strength > 1.0 {
                    error!("Strength must be between 0.0 and 1.0")
                }
            }
            RumbleEffect::Breathing {
                initial_strength,
                step,
                peak,
                ..
            } => {
                if initial_strength < 0.0 || initial_strength > 1.0 {
                    error!("Initial strength must be between 0.0 and 1.0")
                }

                if step < 0.0 || step > 1.0 {
                    error!("Step must be between 0.0 and 1.0")
                }

                if peak < initial_strength {
                    error!("Peak must be higher than initial strength")
                }
            }
        };

        self.rumble_effect = effect;
    }

    pub fn update(&mut self) -> Result<(), ()> {
        if self.update_hsv_and_rumble().is_err() {
            return Err(());
        }

        let mut data = [0 as u8; 44];

        if self.device.read(&mut data).is_ok() {
            if data[0] == MoveRequestType::GetInput as u8 {
                let data = DataInput::new(data);

                self.update_battery(data.battery);
            }
        }

        return Ok(());
    }

    pub fn transform_led(&mut self) {
        let led_effect = &mut self.led_effect;
        let current_hsv = self.setting.led;

        self.setting.led = led_effect.get_updated_hsv(current_hsv);
    }

    pub fn transform_rumble(&mut self) {
        let rumble_effect = &mut self.rumble_effect;
        let current_rumble = self.setting.rumble;

        self.setting.rumble = rumble_effect.get_updated_rumble(current_rumble);
    }

    fn update_hsv_and_rumble(&self) -> Result<(), ()> {
        let request = build_set_led_and_rumble_request(self.setting.led, self.setting.rumble);

        let res = self.device.write(&request);

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = &err;

                match err {
                    HidError::HidApiError { message } => {
                        // This is an error that sometimes occurs when there's a connection drop
                        if message == "Overlapped I/O operation is in progress." {
                            debug!("Couldn't set HSV due to {}", err);
                            return Ok(());
                        }
                    }
                    _ => {}
                }
                error!("Failed to set HSV {}", err);
                Err(())
            }
        }
    }

    fn update_battery(&mut self, battery: u8) {
        let curr_battery = BatteryLevel::from_byte(battery);
        let last_battery = &self.battery;

        if curr_battery != *last_battery {
            if *last_battery == Unknown {
                info!(
                    "Controller battery status known. ('{}' at {})",
                    self.bt_address, curr_battery
                );
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
        MoveRequestType::SetLEDPWMFrequency as u8,
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
        MoveRequestType::SetLED as u8,
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
