use hidapi::{HidApi, HidDevice};
use palette::{FromColor, Hsv, Srgb};

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
    effect: Box<dyn LedEffect>,
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
            effect: Box::new(OFF {}),
            current_setting: PsMoveSetting {
                led: Hsv::new(0.0, 0.0, 0.0),
                rumble: 0.0,
            },
        }
    }

    pub fn set_effect(&mut self, effect: Box<dyn LedEffect>, initial_hsv: (f32, f32, f32)) -> () {
        self.set_hsv(Hsv::from_components(initial_hsv));
        self.effect = effect;
    }

    pub fn update(&mut self) -> bool {
        let led = self.effect.transformer(self.current_setting.led);

        if !self.set_hsv(led) {
            return false;
        }

        if !self.set_rumble(self.current_setting.rumble) {
            return false;
        }

        return true;
    }

    fn set_hsv(&mut self, hsv: Hsv) -> bool {
        let setting = &mut self.current_setting;
        let request = build_set_leds_rumble_request(hsv, setting.rumble);

        let is_ok = self.hid_device.write(&request).is_ok();

        if is_ok {
            setting.led = hsv.clone();
        }

        return is_ok;
    }

    pub fn set_rumble(&mut self, rumble: f32) -> bool {
        let setting = &mut self.current_setting;
        let request = build_set_leds_rumble_request(setting.led, rumble);

        let is_ok = self
            .hid_device
            .write(&request)
            .is_ok();

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

pub trait LedEffect {
    fn transformer(&mut self, led: Hsv) -> Hsv;
}

pub struct OFF {}
impl LedEffect for OFF {
    fn transformer(&mut self, _led: Hsv) -> Hsv {
        return Hsv::from_components((0.0, 0.0, 0.0));
    }
}
impl OFF {
    pub fn new() -> OFF {
        OFF {}
    }
}

pub struct Static {}
impl LedEffect for Static {
    fn transformer(&mut self, led: Hsv) -> Hsv {
        return led;
    }
}
impl Static {
    pub fn new() -> Static {
        Static {}
    }
}

pub struct Rainbow {}
impl LedEffect for Rainbow {
    fn transformer(&mut self, mut led: Hsv) -> Hsv {
        led.hue += 0.1;
        return led;
    }
}

pub struct Breathing {
    peak: f32,
    step: f32,
    is_inhaling: bool,
}
impl LedEffect for Breathing {
    fn transformer(&mut self, mut led: Hsv) -> Hsv {
        if self.is_inhaling {
            led.value += self.step;

            if led.value >= self.peak {
                self.is_inhaling = false;
                led.value = self.peak;
            }
        } else {
            led.value -= self.step;

            if led.value <= 0.0 {
                self.is_inhaling = true;
                led.value = 0.0;
            }
        }

        return led;
    }
}
impl Breathing {
    pub fn new(step: f32, peak: f32) -> Breathing {
        Breathing { step: step, peak: peak, is_inhaling: true }
    }
}
