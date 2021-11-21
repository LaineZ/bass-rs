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


pub struct Bass;
impl Bass {
    fn new() -> Self {
        Bass {}
    }

    pub fn init<W>(device: BassDevice, frequency: u32, flags:Vec<InitFlags>, window_ptr:*mut W) -> BassResult<Self> {
        let flags = flags.to_num();

        check_bass_err!(bass_sys::BASS_Init(
            device.id as i32, 
            44100, 
            flags, 
            window_ptr as *mut c_void, 
            std::ptr::null::<c_void>() as *mut c_void
        ));
        Ok(Self::new())
    }

    pub fn init_default() -> BassResult<Self> {
        Self::init_default_with_ptr(0 as *mut c_void)
    }

    pub fn init_default_with_ptr<W>(window_ptr:*mut W) -> BassResult<Self> {
        check_bass_err!(bass_sys::BASS_Init(-1, 44100, BASS_DEVICE_LATENCY, window_ptr as *mut c_void, std::ptr::null::<c_void>() as *mut c_void));
        Ok(Self::new())
    }
}
impl Drop for Bass {
    fn drop(&mut self) {
        #[cfg(drop_debug)]
        println!("dropping bass!!");

        if BASS_Free() == 0 {
            panic!("Bass failed to free: {:?}", BassError::get_last_error())
        }
    }
}