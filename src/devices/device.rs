use crate::{prelude::*};
use std::ffi::CStr;
use std::alloc::{Layout, alloc_zeroed, dealloc};


pub struct BassDevice {
    pub id: u32,
    pub name: String,
    pub driver: String,
    // pub flags: Vec<BassDeviceFlags>
}
impl BassDevice {
    pub fn get_all_devices() -> BassResult<Vec<BassDevice>> {
        let mut list = Vec::new();

        let mut i = 1;
        let mut last_ok = 1;

        while last_ok == 1 {
            let mut info = BassDeviceInfo::new(
                std::ptr::null(), //name.data(),
                std::ptr::null(), //driver.data(),
                0
            );

            last_ok = BASS_GetDeviceInfo(i, &mut info);
            if last_ok == 0 {break}

            let name = pain(info.name);
            let driver = pain(info.driver);

            list.push(Self::new(i, name, driver));
            i += 1;
        }

        Ok(list)
    }

    fn new(id:u32, name:String, driver:String) -> Self {
        Self {
            id,
            name,
            driver
        }
    }
}

impl BassDevice {
    /// use this device
    pub fn set(&self) -> BassResult<()> {
        check_bass_err!(BASS_SetDevice(self.id));
        Ok(())
    }

    // /// refresh the device info
    // /// returns true if the device info has changed
    // pub fn refresh(&mut self) -> BassResult<bool> {
    //     Ok(false)
    // }

}

fn pain(ugh: *const std::ffi::c_void) -> String {
    unsafe {
        CStr::from_ptr(ugh as *const i8).to_string_lossy().to_string()
    }
}

struct Cringe {
    pub cap: usize,
    data: *mut std::ffi::c_void
}
impl Cringe {
    pub fn new(cap: usize) -> Self {
        let data = unsafe {
            alloc_zeroed(Layout::array::<std::os::raw::c_char>(cap).unwrap()) as *mut std::ffi::c_void
        };

        Self {
            cap, 
            data,
        }
    }
    pub fn data(&mut self) -> *mut std::ffi::c_void {
        self.data
    }
}
impl ToString for Cringe {
    fn to_string(&self) -> String {
        unsafe {
            CStr::from_ptr(self.data as *const i8).to_string_lossy().to_string()
        }
    }
}
impl Drop for Cringe {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data as *mut u8, Layout::array::<std::os::raw::c_char>(self.cap).unwrap())
        }
    }
}

#[test]
fn cringe_test() {
    let mut new_cringe = Cringe::new(100);
    for i in 0..10 {
        let c = (i as u8 + 65) as char;
        println!("adding: {}", c);
        unsafe {
            *(new_cringe.data().add(i) as *mut char) = c;
        }
    }

    println!("new_cringe: {}", new_cringe.to_string());
    panic!("show console output")
}