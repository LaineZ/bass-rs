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

/// The main entrypoint for this lib.
/// 
/// # Usage
/// 
/// ## Initialization
/// 
/// There are three init functions you can use (right now)
/// 
/// `init_default`, `init_default_with_ptr`, and `init` (View these functions to see examples).
/// 
/// The init functions will return a Bass object, which should not be dropped as long as you want to use bass.
/// 
/// ## Playing Audio
/// 
/// To play audio, you must first make a channel. 
/// There are three kinds of channel you can create:
/// 
/// [`StreamChannel`], [`SampleChannel`], and [`MusicChannel`]. (View these structs for usage and examples).
/// 
/// Once a channel has been created, you can start playing audio by running .play() on it.
/// 
/// Each channel is de-ref into a [`Channel`], so see it for more usage
/// 
/// # Dropping
/// When this object is dropped, it will un-initialize bass (Bass_Free).
/// 
/// Bass will need to be re-initialized before use, and any channels will have to be remade
pub struct Bass;
impl Bass {
    fn new() -> Self {
        Bass {}
    }

    pub fn init<W>(device: BassDevice, frequency: u32, flags:Vec<InitFlags>, window_ptr:*mut W) -> BassResult<Self> {
        let flags = flags.to_num();

        check_bass_err!(bass_sys::BASS_Init(
            device.id as i32, 
            frequency, 
            flags, 
            window_ptr as *mut c_void, 
            std::ptr::null::<c_void>() as *mut c_void
        ));
        Ok(Self::new())
    }

    /// Init Bass with default flags and no window pointer
    /// 
    /// Device is default (-1), flags are BASS_DEVICE_LATENCY, window pointer is 0
    /// ```ignore
    /// let bass = Bass::init_default().expect("Error initializing Bass");
    /// ```
    pub fn init_default() -> BassResult<Self> {
        Self::init_default_with_ptr(0 as *mut c_void)
    }

    /// Init bass with default flags with specified device
    pub fn init_with_device(device: BassDevice) -> BassResult<Self> {
        check_bass_err!(bass_sys::BASS_Init(device.id as i32, 44100, BASS_DEVICE_LATENCY, 0 as *mut c_void, std::ptr::null::<c_void>() as *mut c_void));
        Ok(Self::new())
    }

    /// Init Bass with default flags and a window pointer
    /// 
    /// Device is default (-1), flags are BASS_DEVICE_LATENCY
    /// 
    /// ```ignore
    /// // create a glfw window
    /// use glfw::{Action, Context, Key, WindowMode::Windowed};
    /// let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    /// let (window, _events) = glfw.create_window(300, 300, "Bass!", Windowed).unwrap();
    /// 
    /// // get the window pointer
    /// #[cfg(target_os = "windows")]
    /// let window_ptr = window.get_win32_window();
    /// #[cfg(target_os = "linux")]
    /// let window_ptr = window.get_x11_window();
    /// #[cfg(target_os = "macos")]
    /// let window_ptr = window.get_cocoa_window();
    /// 
    /// // initialize bass
    /// let bass_instance = bass_rs::Bass::init_default_with_ptr(window_ptr).expect("Error initializing bass");
    /// ```
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