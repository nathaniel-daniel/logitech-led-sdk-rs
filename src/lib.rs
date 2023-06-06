mod color_percent;
mod sdk;
mod target_device;

pub use self::color_percent::ColorPercent;
pub use self::sdk::Sdk;
pub use self::target_device::TargetDevice;
pub use logitech_led_sdk_sys as sys;
use std::convert::TryInto;
use std::os::raw::c_int;
use std::sync::Mutex;
use std::time::Duration;
pub use sys::LogiLed_DeviceType as DeviceType;
pub use sys::LogiLed_KeyName as KeyName;

/// The lock that syncs accesses to the SDK.
///
/// If you use raw sdk api functions anywhere, you MUST use this lock to wrap accesses to the sdk in order to prevent data races.
/// This library does all this for you, this is exposed only for users who want to use raw sdk functions safely.
pub static SDK_LOCK: Mutex<()> = Mutex::new(());

impl Sdk {
    /// Sets the lighting for a keyboard key by scan code
    pub fn set_lighting_for_key_with_scan_code(&self, code: u32, color: ColorPercent) -> bool {
        unsafe {
            sys::LogiLedSetLightingForKeyWithScanCode(
                code as _,
                color.r as _,
                color.g as _,
                color.b as _,
            )
        }
    }

    /// Sets the lighting for a keyboard key by HID code
    pub fn set_lighting_for_key_with_hid_code(&self, code: u32, color: ColorPercent) -> bool {
        unsafe {
            sys::LogiLedSetLightingForKeyWithHidCode(
                code as _,
                color.r as _,
                color.g as _,
                color.b as _,
            )
        }
    }

    /// Sets the lighting for a specific device's target zone
    pub fn set_lighting_for_target_zone(
        &self,
        device: DeviceType,
        zone: usize,
        color: ColorPercent,
    ) -> bool {
        unsafe {
            sys::LogiLedSetLightingForTargetZone(
                device as _,
                zone as _,
                color.r as _,
                color.g as _,
                color.b as _,
            )
        }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the flashes occur overall. The interval is the time between flashes.
    pub fn flash_lighting(
        &self,
        color: ColorPercent,
        duration: Duration,
        interval: Duration,
    ) -> bool {
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        let interval: c_int = match interval.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe {
            sys::LogiLedFlashLighting(color.r as _, color.g as _, color.b as _, duration, interval)
        }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the flashes occur overall. The interval is the time between flashes.
    pub fn flash_single_key(
        &self,
        key: KeyName,
        color: ColorPercent,
        duration: Duration,
        interval: Duration,
    ) -> bool {
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        let interval: c_int = match interval.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe {
            sys::LogiLedFlashSingleKey(
                key as _,
                color.r as _,
                color.g as _,
                color.b as _,
                duration,
                interval,
            )
        }
    }

    /// Stops all current LED effects
    pub fn stop_effects(&self) -> bool {
        unsafe { sys::LogiLedStopEffects() }
    }

    /// Stops all LED effects on one key
    pub fn stop_effects_on_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedStopEffectsOnKey(key as _) }
    }

    /// Returns None if the call fails or any of the time values are too large.
    ///
    /// Duration is how long the pulses occur overall.
    /// The interval is the time between pulses.
    pub fn pulse_lighting(
        &self,
        color: ColorPercent,
        duration: Duration,
        interval: Duration,
    ) -> bool {
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        let interval: c_int = match interval.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe {
            sys::LogiLedPulseLighting(color.r as _, color.g as _, color.b as _, duration, interval)
        }
    }

    /// Returns None if the call fails or any of the time values are too large. Duration how long the pulses occur overall.
    pub fn pulse_single_key(
        &self,
        key: KeyName,
        start_color: ColorPercent,
        end_color: ColorPercent,
        duration: Duration,
        is_infinite: bool,
    ) -> bool {
        let duration: c_int = match duration.as_millis().try_into() {
            Ok(v) => v,
            Err(_e) => return false,
        };
        unsafe {
            sys::LogiLedPulseSingleKey(
                key as _,
                start_color.r as _,
                start_color.g as _,
                start_color.b as _,
                end_color.r as _,
                end_color.g as _,
                end_color.b as _,
                duration,
                is_infinite,
            )
        }
    }

    /// Saves the current lighting config for the given key
    pub fn save_lighting_for_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedSaveLightingForKey(key as _) }
    }

    /// Saves the current lighting config for the given key
    pub fn restore_lighting_for_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedRestoreLightingForKey(key as _) }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        unsafe {
            sys::LogiLedShutdown();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    // A simple lock to ensure that only one test runs as a time.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn sanity_check() {
        let _test_lock = TEST_LOCK.lock().expect("test lock poisoned");

        // 1st init
        let sdk = Sdk::new().expect("LG SDK");
        std::thread::sleep(Duration::from_secs(5));
        drop(sdk);
        std::thread::sleep(Duration::from_secs(5));

        // 2nd init
        let sdk = Sdk::new_with_name("Test").unwrap();
        std::thread::sleep(Duration::from_secs(5));
        let _version = sdk.get_version().unwrap();
        assert!(sdk.set_target(TargetDevice::All));
        assert!(sdk.set_lighting(ColorPercent::new_rgb(255, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::L, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Mouse,
            1,
            ColorPercent::new_rgb(255, 0, 0)
        ));
        assert!(sdk.flash_lighting(
            ColorPercent::new_rgb(255, 0, 0),
            Duration::from_millis(10_000),
            Duration::from_millis(100)
        ));
        assert!(sdk.stop_effects());
        assert!(sdk.pulse_lighting(
            ColorPercent::new_rgb(255, 0, 0),
            Duration::from_millis(10_000),
            Duration::from_millis(100)
        ));
        assert!(sdk.stop_effects());
        assert!(sdk.set_lighting_for_key_with_scan_code(16, ColorPercent::new_rgb(255, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_hid_code(26, ColorPercent::new_rgb(255, 255, 255)));
        assert!(sdk.save_lighting_for_key(KeyName::L));
        assert!(sdk.restore_lighting_for_key(KeyName::L));
        assert!(sdk.flash_single_key(
            KeyName::L,
            ColorPercent::new_rgb(255, 0, 0),
            Duration::from_millis(10_000),
            Duration::from_millis(100)
        ));
        assert!(sdk.pulse_single_key(
            KeyName::L,
            ColorPercent::new_rgb(255, 0, 0),
            ColorPercent::new_rgb(255, 255, 0),
            Duration::from_millis(10_000),
            true
        ));
        assert!(sdk.stop_effects_on_key(KeyName::L));
        drop(sdk);
        std::thread::sleep(Duration::from_secs(5));
    }

    #[test]
    fn logi_set_target_zone_sample() {
        let _test_lock = TEST_LOCK.lock().expect("test lock poisoned");

        let sdk = Sdk::new_with_name("Test").expect("LG SDK");
        std::thread::sleep(Duration::from_secs(5));
        assert!(sdk.set_target(TargetDevice::All));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::L, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::O, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::G, ColorPercent::new_rgb(0, 255, 255)));
        assert!(sdk.set_lighting_for_key_with_name(KeyName::I, ColorPercent::new_rgb(0, 255, 255)));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Mouse,
            1,
            ColorPercent::new_rgb(255, 0, 0)
        ));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            1,
            ColorPercent::new_rgb(255, 0, 0)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            2,
            ColorPercent::new_rgb(255, 255, 0)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            3,
            ColorPercent::new_rgb(0, 255, 0)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            4,
            ColorPercent::new_rgb(0, 255, 255)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Keyboard,
            5,
            ColorPercent::new_rgb(0, 0, 255)
        ));

        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Headset,
            0,
            ColorPercent::new_rgb(255, 255, 255)
        ));
        assert!(sdk.set_lighting_for_target_zone(
            DeviceType::Headset,
            1,
            ColorPercent::new_rgb(255, 0, 255)
        ));
        drop(sdk);
        std::thread::sleep(Duration::from_secs(5));
    }
}
