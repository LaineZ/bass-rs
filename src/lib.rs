#![allow(unused_variables)]
use bass_error::BassResult;
use bass_sys::BASS_DEVICE_LATENCY;

// mods
mod macros;
pub mod traits;
pub mod prelude;
pub mod channel;
pub mod bass_error;
pub mod bass_flags;
pub mod devices;

pub fn init_default() -> BassResult<()> {
    check_bass_err!(bass_sys::BASS_Init(-1, 44100, BASS_DEVICE_LATENCY, 0 as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void));
    Ok(())
}

pub fn init_default_with_ptr<P>(window_ptr:*mut P) -> BassResult<()> {
    check_bass_err!(bass_sys::BASS_Init(-1, 44100, BASS_DEVICE_LATENCY, window_ptr as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void));
    Ok(())
}