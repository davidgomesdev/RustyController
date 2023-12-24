use std::collections::HashMap;

use hidapi::{HidDevice, HidError};
use palette::{FromColor, Hsv, Srgb};
use tracing::info;

use crate::ps_move::effects::{LedEffect, RumbleEffect, RumbleEffectKind};
use crate::ps_move::models::{BatteryLevel, ButtonState, ConnectionType, ControllerInfo, DataInput, fill_state_from_byte_slice, MoveRequestType, MoveSetting};
use crate::ps_move::models::BatteryLevel::Unknown;
use crate::tasks::models::Button;

pub const MIN_LED_PWM_FREQUENCY: u64 = 0x02dd;
pub const MAX_LED_PWM_FREQUENCY: u64 = 0x24e6;

pub struct PsMoveController {
    device: HidDevice,
    pub(super) info: ControllerInfo,
    pub bt_address: String,
    pub last_led_effect: LedEffect,
    pub led_effect: LedEffect,
    pub rumble_effect: RumbleEffect,
    pub setting: MoveSetting,
    pub last_battery: BatteryLevel,
    pub battery: BatteryLevel,
    last_button_state: HashMap<Button, ButtonState>,
    button_state: HashMap<Button, ButtonState>,
    pub trigger: f32,
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
            last_led_effect: LedEffect::off(),
            led_effect: LedEffect::off(),
            rumble_effect: RumbleEffect::off(),
            setting: MoveSetting {
                led: Hsv::from_components((0.0, 0.0, 0.0)),
                last_led: Hsv::from_components((0.0, 0.0, 0.0)),
                rumble: 0.0,
            },
            connection_type,
            last_battery: Unknown,
            battery: Unknown,
            last_button_state: HashMap::new(),
            button_state: HashMap::new(),
            trigger: 0.0,
        }
    }

    pub fn is_same_device(&self, info: &ControllerInfo) -> bool {
        match self.connection_type {
            ConnectionType::Usb => self.info.usb_path == info.usb_path,
            ConnectionType::Bluetooth => self.info.bt_path == info.bt_path,
            ConnectionType::UsbAndBluetooth => {
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

        if self.connection_type == ConnectionType::Usb {
            self.info.bt_path = other.info.bt_path.clone();
        } else if self.connection_type == ConnectionType::Bluetooth {
            self.info.usb_path = other.info.usb_path.clone();
        }
        self.connection_type = ConnectionType::UsbAndBluetooth;
    }

    #[allow(dead_code)]
    pub fn set_led_pwm_frequency(&self, frequency: u64) -> bool {
        let request = build_set_led_pwm_request(frequency);

        self.device.write(&request).is_ok()
    }

    pub fn revert_led_effect(&mut self) {
        let current_effect = self.led_effect;
        let current_led = self.setting.led;

        self.led_effect = self.last_led_effect;
        self.setting.led = self.setting.last_led;

        self.last_led_effect = current_effect;
        self.setting.last_led = current_led;

        let last_led_effect = &mut self.last_led_effect;

        if let Some(duration) = last_led_effect.duration {
            if last_led_effect.start.elapsed() >= duration {
                info!("Last led effect '{}' of '{}' has already expired, setting to off", last_led_effect, self.bt_address);
                let off_effect = LedEffect::off();

                self.led_effect = off_effect;
                self.setting.led = off_effect.kind.get_initial_hsv()
            }
        }

        info!("Reverted '{:?}' to '{}'", self.bt_address, self.led_effect);
    }

    pub fn set_led_effect(&mut self, effect: LedEffect) {
        self.last_led_effect = self.led_effect;
        self.setting.last_led = self.setting.led;

        let mut kind = effect.kind;

        self.led_effect = effect;
        self.setting.led = kind.get_updated_hsv(self.setting.led);
    }

    pub fn set_led_effect_with_hsv(&mut self, effect: LedEffect, hsv: Hsv) {
        self.setting.led = hsv;
        self.led_effect = effect;
    }

    pub fn set_rumble_effect(&mut self, effect: RumbleEffect) {
        match effect.kind {
            RumbleEffectKind::Off => {}
            RumbleEffectKind::Static { strength } => {
                if !(0.0..=1.0).contains(&strength) {
                    tracing::error!("Strength must be between 0.0 and 1.0")
                }
            }
            RumbleEffectKind::Breathing {
                initial_strength,
                step,
                peak,
                ..
            } => {
                if !(0.0..=1.0).contains(&initial_strength) {
                    tracing::error!("Initial strength must be between 0.0 and 1.0")
                }

                if !(0.0..=1.0).contains(&step) {
                    tracing::error!("Step must be between 0.0 and 1.0")
                }

                if peak < initial_strength {
                    tracing::error!("Peak must be higher than initial strength")
                }
            }
            RumbleEffectKind::Blink { strength, .. } => {
                if !(0.0..=1.0).contains(&strength) {
                    tracing::error!("Strength must be between 0.0 and 1.0")
                }
            }
        };

        self.rumble_effect = effect;
    }

    pub fn update(&mut self) -> Result<(), ()> {
        if self.update_hsv_and_rumble().is_err() {
            return Err(());
        }

        let mut data = [0_u8; 44];

        if self.device.read(&mut data).is_ok() && data[0] == MoveRequestType::GetInput as u8 {
            let data = DataInput::new(data);

            self.update_battery(data.battery);
            self.update_button_state(data.get_button_slice());
            self.trigger = data.get_trigger();
        }

        Ok(())
    }

    pub fn get_changed_buttons(&self) -> HashMap<Button, ButtonState> {
        if self.last_button_state.is_empty() {
            return HashMap::new()
        }

        HashMap::<Button, ButtonState>::from_iter(self.button_state
            .iter()
            .filter(|(btn, state)| self.last_button_state.get(btn).unwrap() != *state)
            .map(|a| (*a.0, *a.1))
            .collect::<Vec<(Button, ButtonState)>>())
    }

    pub fn transform_led(&mut self) {
        let led_effect = &mut self.led_effect;
        let current_hsv = self.setting.led;

        if led_effect.duration.is_some() {
            let duration = led_effect.duration.unwrap();

            if led_effect.start.elapsed() >= duration {
                info!("Led effect '{}' of '{}' expired", led_effect, self.bt_address);
                self.set_led_effect(LedEffect::off());
                return;
            }
        };

        self.setting.led = led_effect.kind.get_updated_hsv(current_hsv);
    }

    pub fn transform_rumble(&mut self) {
        let rumble_effect = &mut self.rumble_effect;
        let current_rumble = self.setting.rumble;

        if rumble_effect.duration.is_some() {
            let duration = rumble_effect.duration.unwrap();

            if rumble_effect.start.elapsed() >= duration {
                self.set_rumble_effect(RumbleEffect::off());
                return;
            }
        };

        self.setting.rumble = rumble_effect.kind.get_updated_rumble(current_rumble);
    }

    fn update_hsv_and_rumble(&self) -> Result<(), ()> {
        let request = build_set_led_and_rumble_request(self.setting.led, self.setting.rumble);

        let res = self.device.write(&request);

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = &err;

                if let HidError::HidApiError { message } = err {
                    // This is an error that sometimes occurs when there's a connection drop
                    if message == "Overlapped I/O operation is in progress." {
                        tracing::trace!("Couldn't set HSV due to {err}");
                        return Ok(());
                    }
                }
                tracing::error!("Failed to set HSV {err}");
                Err(())
            }
        }
    }

    fn update_battery(&mut self, byte: u8) {
        let curr_battery = BatteryLevel::from_byte(byte);
        let battery = &self.battery;

        if curr_battery != *battery {
            self.last_battery = *battery;

            if *battery == Unknown {
                info!(
                    "Controller battery status known. ('{}' at {curr_battery})",
                    self.bt_address
                );
            } else {
                info!(
                    "Controller battery status changed. ('{}' to {curr_battery})",
                    self.bt_address
                );
            }
            self.battery = curr_battery;
        }
    }

    fn update_button_state(&mut self, bytes: [u8; 4]) {
        self.last_button_state.clone_from(&self.button_state);
        fill_state_from_byte_slice(&mut self.button_state, bytes);
    }
}

#[allow(dead_code)]
fn build_set_led_pwm_request(frequency: u64) -> [u8; 7] {
    if !(MIN_LED_PWM_FREQUENCY..=MAX_LED_PWM_FREQUENCY).contains(&frequency) {
        panic!("Frequency must be between {MIN_LED_PWM_FREQUENCY} and {MAX_LED_PWM_FREQUENCY}!")
    }

    [
        MoveRequestType::SetLEDPWMFrequency as u8,
        0x41,
        0,
        (frequency & 0xFF) as u8,
        ((frequency >> 8) & 0xFF) as u8,
        ((frequency >> 16) & 0xFF) as u8,
        ((frequency >> 24) & 0xFF) as u8,
    ]
}

fn build_set_led_and_rumble_request(hsv: Hsv, rumble: f32) -> [u8; 8] {
    let f32_to_u8 = |f: f32| (f * 255.0) as u8;
    let rgb = hsv_to_rgb(hsv, f32_to_u8);

    [
        MoveRequestType::SetLED as u8,
        0,
        rgb[0],
        rgb[1],
        rgb[2],
        0,
        f32_to_u8(rumble),
        0,
    ]
}

fn hsv_to_rgb(hsv: Hsv, f32_to_u8: fn(f: f32) -> u8) -> [u8; 3] {
    let rgb = Srgb::from_color(hsv);

    [rgb.red, rgb.green, rgb.blue].map(f32_to_u8)
}
