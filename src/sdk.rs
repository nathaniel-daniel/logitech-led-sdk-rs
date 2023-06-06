use crate::sys;
use crate::ColorPercent;
use crate::KeyName;
pub use crate::TargetDevice;
use crate::SDK_LOCK;
use std::ffi::CString;
use std::sync::MutexGuard;
use std::sync::TryLockError;

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
        unsafe { sys::LogiLedSetTargetDevice(target_device.bits() as i32) }
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
                key as _,
                color.r.into(),
                color.g.into(),
                color.b.into(),
            )
        }
    }
}
