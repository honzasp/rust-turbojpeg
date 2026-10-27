use std::ffi::CStr;
use crate::common::{Result, Error};
use crate::raw;

#[derive(Debug)]
pub struct Handle {
    ptr: raw::tjhandle,
}

impl Handle {
    pub fn new(init: raw::TJINIT) -> Result<Self> {
        let ptr = unsafe { raw::tj3Init(init as libc::c_int) };
        let mut this = Self { ptr };
        if this.ptr.is_null() {
            return Err(this.get_error())
        }
        Ok(this)
    }

    pub fn get_error(&mut self) -> Error {
        Error::TurboJpegError(self.get_error_message())
    }

    pub fn get_error_message(&mut self) -> String {
        let msg = unsafe { CStr::from_ptr(raw::tj3GetErrorStr(self.ptr)) };
        msg.to_string_lossy().into_owned()
    }

    /// Whether the last error was a warning. TurboJPEG also reports a fatal error as a warning when
    /// a warning preceded it.
    pub fn error_is_warning(&mut self) -> bool {
        let code = unsafe { raw::tj3GetErrorCode(self.ptr) };
        code == raw::TJERR_TJERR_WARNING as libc::c_int
    }

    pub fn get(&mut self, param: raw::TJPARAM) -> libc::c_int {
        unsafe { raw::tj3Get(self.ptr, param as libc::c_int) }
    }

    pub fn set(&mut self, param: raw::TJPARAM, value: libc::c_int) -> Result<()> {
        let res = unsafe { raw::tj3Set(self.ptr, param as libc::c_int, value) };
        if res != 0 {
            return Err(self.get_error())
        }
        Ok(())
    }

    pub fn set_scaling_factor(&mut self, scaling_factor: raw::tjscalingfactor) -> Result<()> {
        let res = unsafe { raw::tj3SetScalingFactor(self.ptr, scaling_factor) };
        if res != 0 {
            return Err(self.get_error())
        }
        Ok(())
    }

    pub unsafe fn as_ptr(&mut self) -> raw::tjhandle {
        self.ptr
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { raw::tj3Destroy(self.ptr); }
    }
}
