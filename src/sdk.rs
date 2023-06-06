use crate::sys;
use crate::ColorPercent;
use crate::DeviceType;
use crate::KeyName;
pub use crate::TargetDevice;
use crate::SDK_LOCK;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::MutexGuard;
use std::sync::TryLockError;
use std::time::Duration;

/// Entry to Api.
///
/// This serves as proof of initalization and prevents the API from being used by other threads.
pub struct Sdk(MutexGuard<'static, ()>);

impl Sdk {
    /// Create a new sdk instance with no name.
    ///
    /// # Returns
    /// Returns None if the sdk could not be initialized.
    pub fn new() -> Option<Self> {
        let guard = match SDK_LOCK.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::WouldBlock) => return None,
            Err(TryLockError::Poisoned(e)) => e.into_inner(),
        };

        let init = unsafe { sys::LogiLedInit() };
        if !init {
            return None;
        }

        Some(Sdk(guard))
    }

    /// Create a new sdk instance with a name, where the name is the name of the application using the sdk.
    ///
    /// # Panics
    /// Panics if the name contains interior NULs.
    ///
    /// # Returns
    /// Returns None if the sdk could not be initialized.
    pub fn new_with_name(name: &str) -> Option<Self> {
        let guard = match SDK_LOCK.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::WouldBlock) => return None,
            Err(TryLockError::Poisoned(e)) => e.into_inner(),
        };

        let name = CString::new(name).expect("name contains interior NUL");
        let init = unsafe { sys::LogiLedInitWithName(name.as_ptr()) };
        if !init {
            return None;
        }

        Some(Sdk(guard))
    }

    /// Returns the sdk version.
    ///
    /// # Returns
    /// Returns a tuple of the major, minor and build numbers if successful.
    /// Returns None if the version could not be found.
    pub fn get_version(&self) -> Option<(u32, u32, u32)> {
        let mut major = 0;
        let mut minor = 0;
        let mut build = 0;

        let valid = unsafe { sys::LogiLedGetSdkVersion(&mut major, &mut minor, &mut build) };
        if !valid {
            return None;
        }

        // i32 -> u32, transmute
        Some((major as u32, minor as u32, build as u32))
    }

    /// Selects the target devices.
    ///
    /// # Returns
    /// Returns true if the target devices were selected.
    pub fn set_target(&self, target_device: TargetDevice) -> bool {
        // u32 -> i32, transmute
        unsafe { sys::LogiLedSetTargetDevice(target_device.bits() as c_int) }
    }

    /// Sets the lighting.
    ///
    /// # Returns
    /// Returns true if successful.
    pub fn set_lighting(&self, color: ColorPercent) -> bool {
        unsafe { sys::LogiLedSetLighting(color.r.into(), color.g.into(), color.b.into()) }
    }

    /// Set the lighting for a keyboard key by key name.
    ///
    /// # Returns
    /// Returns true if successful.
    pub fn set_lighting_for_key_with_name(&self, key: KeyName, color: ColorPercent) -> bool {
        unsafe {
            sys::LogiLedSetLightingForKeyWithKeyName(
                key,
                color.r.into(),
                color.g.into(),
                color.b.into(),
            )
        }
    }

    /// Sets the lighting for a keyboard key by scan code.
    ///
    /// # Returns
    /// Returns true if successful.
    pub fn set_lighting_for_key_with_scan_code(&self, scan_code: u32, color: ColorPercent) -> bool {
        unsafe {
            sys::LogiLedSetLightingForKeyWithScanCode(
                scan_code as c_int,
                color.r.into(),
                color.g.into(),
                color.b.into(),
            )
        }
    }

    /// Sets the lighting for a keyboard key by HID code.
    ///
    /// # Returns
    /// Returns true if successful.
    pub fn set_lighting_for_key_with_hid_code(&self, hid_code: u32, color: ColorPercent) -> bool {
        unsafe {
            sys::LogiLedSetLightingForKeyWithHidCode(
                hid_code as c_int,
                color.r.into(),
                color.g.into(),
                color.b.into(),
            )
        }
    }

    /// Sets the lighting for a specific device's target zone.
    ///
    /// A zone number is generally different per device, read the offical SDK docs for more info.
    ///
    /// # Returns
    /// Returns true if successful.
    pub fn set_lighting_for_target_zone(
        &self,
        device: DeviceType,
        zone: u32,
        color: ColorPercent,
    ) -> bool {
        unsafe {
            sys::LogiLedSetLightingForTargetZone(
                device,
                zone as c_int,
                color.r.into(),
                color.g.into(),
                color.b.into(),
            )
        }
    }

    /// Save the current lighting, play the effect, and restore the lighting.
    ///
    /// duration controls how long the flashes occur overall.
    /// If omitted, it will run until manually stopped.
    /// The interval is the time between flashes.
    ///
    /// # Returns
    /// Returns false if the call fails or any of the time values are too large.
    pub fn flash_lighting(
        &self,
        color: ColorPercent,
        duration: Option<Duration>,
        interval: Duration,
    ) -> bool {
        let duration = match duration.map(|duration| c_int::try_from(duration.as_millis())) {
            Some(Ok(duration)) if duration != sys::LOGI_LED_DURATION_INFINITE as c_int => duration,
            Some(Err(_)) | Some(Ok(_)) => return false,
            None => sys::LOGI_LED_DURATION_INFINITE as c_int,
        };
        let interval = match c_int::try_from(interval.as_millis()) {
            Ok(interval) => interval,
            Err(_) => return false,
        };

        unsafe {
            sys::LogiLedFlashLighting(
                color.r.into(),
                color.g.into(),
                color.b.into(),
                duration,
                interval,
            )
        }
    }

    /// Start a flashing effect on the given key.
    ///
    /// duration controls how long the flashes occur overall.
    /// If omitted, it will run until manually stopped.
    /// The interval is the time between flashes.
    ///
    /// # Returns
    /// Returns false if the call fails or any of the time values are too large.
    pub fn flash_single_key(
        &self,
        key: KeyName,
        color: ColorPercent,
        duration: Option<Duration>,
        interval: Duration,
    ) -> bool {
        let duration = match duration.map(|duration| c_int::try_from(duration.as_millis())) {
            Some(Ok(duration)) if duration != sys::LOGI_LED_DURATION_INFINITE as c_int => duration,
            Some(Err(_)) | Some(Ok(_)) => return false,
            None => sys::LOGI_LED_DURATION_INFINITE as c_int,
        };
        let interval = match c_int::try_from(interval.as_millis()) {
            Ok(interval) => interval,
            Err(_) => return false,
        };

        unsafe {
            sys::LogiLedFlashSingleKey(
                key,
                color.r.into(),
                color.g.into(),
                color.b.into(),
                duration,
                interval,
            )
        }
    }

    /// Stops all current LED effects.
    ///
    /// # Returns
    /// Returns false if the call fails.
    pub fn stop_effects(&self) -> bool {
        unsafe { sys::LogiLedStopEffects() }
    }

    /// Stops all LED effects on one key.
    ///
    /// # Returns
    /// Returns false if the call fails.
    pub fn stop_effects_on_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedStopEffectsOnKey(key) }
    }

    /// Save the current lighting, pulse the lighting, then restore the lighting.
    ///
    /// duration controls how long the pulses occur overall.
    /// If omitted, it will run until manually stopped.
    /// The interval is the time between pulses.
    ///
    /// # Returns
    /// Returns false if the call fails or any of the time values are too large.
    pub fn pulse_lighting(
        &self,
        color: ColorPercent,
        duration: Option<Duration>,
        interval: Duration,
    ) -> bool {
        let duration = match duration.map(|duration| c_int::try_from(duration.as_millis())) {
            Some(Ok(duration)) if duration != sys::LOGI_LED_DURATION_INFINITE as c_int => duration,
            Some(Err(_)) | Some(Ok(_)) => return false,
            None => sys::LOGI_LED_DURATION_INFINITE as c_int,
        };
        let interval = match c_int::try_from(interval.as_millis()) {
            Ok(interval) => interval,
            Err(_) => return false,
        };

        unsafe {
            sys::LogiLedPulseLighting(
                color.r.into(),
                color.g.into(),
                color.b.into(),
                duration,
                interval,
            )
        }
    }

    /// Start a pulsing effect on the given key.
    ///
    /// duration controls how long the pulses occur overall.
    ///
    /// # Returns
    /// Returns false if the call fails or any of the time values are too large.
    pub fn pulse_single_key(
        &self,
        key: KeyName,
        start_color: ColorPercent,
        end_color: ColorPercent,
        duration: Duration,
        is_infinite: bool,
    ) -> bool {
        let duration = match c_int::try_from(duration.as_millis()) {
            Ok(duration) => duration,
            Err(_) => return false,
        };

        unsafe {
            sys::LogiLedPulseSingleKey(
                key,
                start_color.r.into(),
                start_color.g.into(),
                start_color.b.into(),
                end_color.r.into(),
                end_color.g.into(),
                end_color.b.into(),
                duration,
                is_infinite,
            )
        }
    }

    /// Saves the current lighting config for the given key.
    ///
    /// # Returns
    /// Returns false if the call fails.
    pub fn save_lighting_for_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedSaveLightingForKey(key) }
    }

    /// Restores the current lighting config for the given key.
    ///
    /// # Returns
    /// Returns false if the call fails.
    pub fn restore_lighting_for_key(&self, key: KeyName) -> bool {
        unsafe { sys::LogiLedRestoreLightingForKey(key) }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        unsafe {
            sys::LogiLedShutdown();
        }
    }
}
