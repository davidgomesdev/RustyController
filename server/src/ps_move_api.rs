use hidapi::{HidApi, HidDevice};
use palette::{rgb::RgbStandard, FromColor, Hsv, Hue, Srgb};

const MAGIC_PATH: &str = "&col01#";
const PSMOVE_VENDOR_ID: u16 = 0x054c;
const PSMOVE_PRODUCT_ID: u16 = 0x03d5;

pub struct PsMoveApi {
    hid: HidApi,
}

impl PsMoveApi {
    pub fn new() -> PsMoveApi {
        PsMoveApi {
            hid: HidApi::new().unwrap(),
        }
    }

    pub fn list(&self) -> Vec<PsMoveController> {
        let controllers = self
            .hid
            .device_list()
            .filter(|device| -> bool {
                let path = device.path().to_str().unwrap();
                let vendor_id = device.vendor_id();
                let product_id = device.product_id();

                path.contains(MAGIC_PATH)
                    && vendor_id == PSMOVE_VENDOR_ID
                    && product_id == PSMOVE_PRODUCT_ID
            })
            .map(|dev_info| self.hid.open_path(dev_info.path()).unwrap())
            .map(|dev| PsMoveController::new(dev));

        return controllers.collect();
    }
}

pub struct PsMoveController {
    hid_device: HidDevice,
    effect: LedEffect,
    current_setting: PsMoveSetting,
}

pub struct PsMoveSetting {
    led: Hsv,
    rumble: f32,
}

impl PsMoveController {
    fn new(hid_device: HidDevice) -> PsMoveController {
        PsMoveController {
            hid_device,
            effect: LedEffect::Off,
            current_setting: PsMoveSetting {
                led: Hsv::from_components((0.0, 0.0, 0.0)),
                rumble: 0.0,
            },
        }
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
                    panic!("Peak must be higher than initial value");
                }
                if step >= peak {
                    panic!("Step must be lower than peak")
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
        let request = build_set_leds_rumble_request(hsv, setting.rumble);

        let is_ok = self.hid_device.write(&request).is_ok();

        if is_ok {
            setting.led = hsv;
        }

        return is_ok;
    }

    pub fn set_rumble(&mut self, rumble: f32) -> bool {
        let setting = &mut self.current_setting;
        let request = build_set_leds_rumble_request(setting.led, rumble);

        let is_ok = self.hid_device.write(&request).is_ok();

        if is_ok {
            setting.rumble = rumble;
        }

        return is_ok;
    }
}

fn build_set_leds_rumble_request(hsv: Hsv, rumble: f32) -> [u8; 8] {
    let rgb = Srgb::from_color(hsv);
    let f32_to_u8 = |f| (f * 255.0) as u8;
    let rgb = [rgb.red, rgb.green, rgb.blue].map(f32_to_u8);

    return [
        PSMOVE_REQ_SET_LED,
        0,
        rgb[0],
        rgb[1],
        rgb[2],
        0,
        f32_to_u8(rumble),
        0,
    ];
}

const PSMOVE_REQ_SET_LED: u8 = 0x06;

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

// pub trait LedEffect {
//     fn transformer(&mut self, led: Hsv) -> Hsv;
// }

// pub struct Off {}
// impl LedEffect for Off {
//     fn transformer(&mut self, _led: Hsv) -> Hsv {
//         return Hsv::from_components((0.0, 0.0, 0.0));
//     }
// }
// impl Off {
//     pub fn new() -> Off {
//         Off {}
//     }
// }

// pub struct Static {}
// impl LedEffect for Static {
//     fn transformer(&mut self, led: Hsv) -> Hsv {
//         return led;
//     }
// }
// impl Static {
//     pub fn new() -> Static {
//         Static {}
//     }
// }

// pub struct Rainbow {
//     step: f32
// }
// impl LedEffect for Rainbow {
//     fn transformer(&mut self, mut led: Hsv) -> Hsv {
//         led.hue += self.step;
//         return led;
//     }
// }
// impl Rainbow {
//     pub fn new(step: f32) -> Rainbow {
//         Rainbow { step }
//     }
// }

// pub struct Breathing {
//     peak: f32,
//     step: f32,
//     is_inhaling: bool,
// }
// impl LedEffect for Breathing {
//     fn transformer(&mut self, mut led: Hsv) -> Hsv {
//         if self.is_inhaling {
//             led.value += self.step;

//             if led.value >= self.peak {
//                 self.is_inhaling = false;
//                 led.value = self.peak;
//             }
//         } else {
//             led.value -= self.step;

//             if led.value <= 0.0 {
//                 self.is_inhaling = true;
//                 led.value = 0.0;
//             }
//         }

//         return led;
//     }
// }
// impl Breathing {
//     pub fn new(step: f32, peak: f32) -> Breathing {
//         Breathing { step, peak, is_inhaling: true }
//     }
// }
