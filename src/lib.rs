#![allow(unused_variables)]
use bass_error::BassResult;

// mods
mod macros;
pub mod traits;
pub mod prelude;
pub mod channel;
pub mod bass_error;
pub mod bass_flags;


pub fn init_default() -> BassResult<()> {
    check_bass_err!(bass_sys::BASS_Init(-1, 44100, bass_sys::BASS_DEVICE_STEREO, 0 as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void));
    Ok(())
}

pub fn init_default_with_ptr<P>(window_ptr:*mut P) -> BassResult<()> {
    check_bass_err!(bass_sys::BASS_Init(-1, 44100, bass_sys::BASS_DEVICE_STEREO, window_ptr as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void));
    Ok(())
}