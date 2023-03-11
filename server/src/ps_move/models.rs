use std::collections::HashMap;

use juniper::GraphQLEnum;
use palette::Hsv;
use strum_macros::Display;

use crate::ps_move::models::BatteryLevel::*;
use crate::tasks::models::Button;

#[derive(Clone)]
pub struct MoveSetting {
    pub led: Hsv,
    pub rumble: f32,
}

#[derive(Display, PartialEq, Copy, Clone, GraphQLEnum)]
pub enum ConnectionType {
    Usb,
    Bluetooth,
    UsbAndBluetooth,
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

// From https://github.com/nitsch/moveonpc/wiki/Input-report
pub fn fill_state_from_byte_slice(state: &mut HashMap<Button, ButtonState>, bytes: [u8; 4]) {
    fill_state(state, &Button::Start, ((bytes[0] >> 3) & 1) == 1);
    fill_state(state, &Button::Select, ((bytes[0]) & 1) == 1);

    fill_state(state, &Button::Square, ((bytes[1] >> 7) & 1) == 1);
    fill_state(state, &Button::Cross, ((bytes[1] >> 6) & 1) == 1);
    fill_state(state, &Button::Circle, ((bytes[1] >> 5) & 1) == 1);
    fill_state(state, &Button::Triangle, ((bytes[1] >> 4) & 1) == 1);

    fill_state(state, &Button::Move, ((bytes[3] >> 6) & 1) == 1);
    fill_state(state, &Button::Trigger, ((bytes[3] >> 7) & 1) == 1);
}

fn fill_state(states: &mut HashMap<Button, ButtonState>, button: &Button, is_down: bool) {
    states.insert(*button, ButtonState::new(is_down));
}

/// `Pressed` means that it was `Up` but now is `Down`, and vice-versa for `Released`
#[derive(GraphQLEnum, PartialEq, Copy, Clone, Display, Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl ButtonState {
    pub fn new(is_down: bool) -> ButtonState {
        if is_down {
            Self::Pressed
        } else {
            Self::Released
        }
    }
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
    // 4 Start
    // 0 Select
    pub buttons1: u8,
    // 7 Square
    // 6 Cross
    // 5 Circle
    // 4 Triangle
    pub buttons2: u8,
    // 0 Ps
    pub buttons3: u8,
    // Move, Trigger
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

    pub fn get_button_slice(&self) -> [u8; 4] {
        [self.buttons1, self.buttons2, self.buttons3, self.buttons4]
    }

    pub fn get_trigger(&self) -> f32 {
        (self.trigger as u16 + self.trigger2 as u16) as f32 / 2.0
    }
}
