#![allow(unused_variables)]
use prelude::*;

// mods
mod macros;
pub mod traits;
pub mod prelude;
pub mod channel;
pub mod bass_error;
pub mod bass_flags;
pub mod devices;



pub struct Bass {

}
impl Bass {
    fn new() -> Self {
        Bass {}
    }

    pub fn init_default() -> BassResult<Self> {
        check_bass_err!(bass_sys::BASS_Init(-1, 44100, BASS_DEVICE_LATENCY, 0 as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void));
        Ok(Self::new())
    }

    pub fn init_default_with_ptr<P>(window_ptr:*mut P) -> BassResult<Self> {
        check_bass_err!(bass_sys::BASS_Init(-1, 44100, BASS_DEVICE_LATENCY, window_ptr as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void));
        Ok(Self::new())
    }
}
impl Drop for Bass {
    fn drop(&mut self) {
        if BASS_Free() == 0 {
            panic!("Bass failed to free: {:?}", BassError::get_last_error())
        }
    }
}