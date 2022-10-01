use juniper::GraphQLEnum;
use palette::{Hsv, Hue};
use strum_macros::Display;

use crate::ps_move::models::BatteryLevel::*;

#[derive(Clone, Copy, Display)]
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

#[derive(Clone, Copy, Display)]
pub enum RumbleEffect {
    Off,
    Static {
        strength: f32
    },
    Breathing {
        initial_strength: f32,
        step: f32,
        peak: f32,
        inhaling: bool,
    },
}

impl LedEffect {
    pub fn get_updated_hsv(&mut self, current_hsv: Hsv) -> Hsv {
        match *self {
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
}

impl RumbleEffect {
    pub fn get_updated_rumble(&mut self, mut current_rumble: f32) -> f32 {
        match *self {
            RumbleEffect::Off => 0.0,
            RumbleEffect::Static { strength: value } => value,
            RumbleEffect::Breathing {
                initial_strength: initial,
                step,
                peak,
                ref mut inhaling,
            } => {
                if *inhaling {
                    current_rumble += step * peak
                } else {
                    current_rumble -= step * peak
                }

                if current_rumble >= peak {
                    current_rumble = peak;
                    *inhaling = false
                } else if current_rumble <= initial {
                    current_rumble = initial;
                    *inhaling = true
                }

                current_rumble
            }
        }
    }
}

#[derive(Clone)]
pub struct MoveSetting {
    pub led: Hsv,
    pub rumble: f32,
}

#[derive(Display, PartialEq, Copy, Clone, GraphQLEnum)]
pub enum ConnectionType {
    USB,
    Bluetooth,
    USBAndBluetooth,
}

#[derive(Clone, PartialEq)]
pub struct ControllerInfo {
    pub serial_number: String,
    pub bt_path: String,
    pub usb_path: String,
}

impl ControllerInfo {
    pub(super) fn from(serial_number: &str, path: &str) -> ControllerInfo {
        let serial_number = String::from(serial_number);
        let path = String::from(path);

        if serial_number.is_empty() {
            ControllerInfo {
                serial_number,
                bt_path: String::new(),
                usb_path: path,
            }
        } else {
            ControllerInfo {
                serial_number,
                bt_path: path,
                usb_path: String::new(),
            }
        }
    }

    pub(super) fn new(serial_number: String, bt_path: String, usb_path: String) -> ControllerInfo {
        ControllerInfo {
            serial_number,
            bt_path,
            usb_path,
        }
    }
}

#[allow(unused)]
pub(super) enum MoveRequestType {
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

#[derive(Display, PartialEq, Copy, Clone, GraphQLEnum)]
pub enum BatteryLevel {
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

impl BatteryLevel {
    pub fn from_byte(byte: u8) -> BatteryLevel {
        match byte {
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

/// Adapted from [psmoveapi's source](https://github.com/thp/psmoveapi/blob/master/src/psmove.c)
#[allow(unused)]
pub(super) struct DataInput {
    // message type, must be PSMove_Req_GetInput
    pub msg_type: u8,
    pub buttons1: u8,
    pub buttons2: u8,
    pub buttons3: u8,
    pub buttons4: u8,
    // trigger value: u8, 0..255
    pub trigger: u8,
    // trigger value, 2nd frame
    pub trigger2: u8,
    _unk7: u8,
    _unk8: u8,
    _unk9: u8,
    _unk10: u8,
    // high byte of timestamp
    time_high: u8,
    // battery level: u8, 0x05 = max, 0xEE = USB charging
    pub battery: u8,
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

impl DataInput {
    pub fn new(req: [u8; 44]) -> DataInput {
        DataInput {
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
