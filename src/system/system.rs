use openal_sys;

use std::ptr;

use libc::strlen;
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

pub enum DeviceError{
    EnumerationExtensionNotSupported
}

pub type DeviceResult<T> = Result<T,DeviceError>;

pub struct System {}

impl System {
    pub fn get_default_device() -> DeviceResult<String> {
        unsafe {
            if System::is_supported("ALC_ENUMERATION_EXT") {
                let ptr = openal_sys::alcGetString(ptr::null(), openal_sys::ALC_DEFAULT_DEVICE_SPECIFIER);
                return Ok(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).to_string());
            }
            else{
                return Err(DeviceError::EnumerationExtensionNotSupported)
            }
        }
    }

    pub fn get_all_device() -> DeviceResult<Vec<String>> {
        let mut result = Vec::new();

        unsafe {
            if System::is_supported("ALC_ENUMERATION_EXT") {
                let mut ptr = openal_sys::alcGetString(ptr::null(), openal_sys::ALC_DEVICE_SPECIFIER);

                while *ptr != 0 {
                    result.push(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).to_string());

                    ptr = ptr.offset(strlen(ptr) as isize + 1);
                }

                return Ok(result);
            }
            else{
                return Err(DeviceError::EnumerationExtensionNotSupported)
            }
        }
    }

    pub fn is_supported(name: &str) -> bool {
        use std::ffi::CString;

		unsafe {
			openal_sys::alcIsExtensionPresent(ptr::null(), CString::new(name).unwrap().as_ptr()) == openal_sys::AL_TRUE
		}
	}
}