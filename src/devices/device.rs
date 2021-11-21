use crate::{prelude::*};
use std::ffi::CStr;
use std::fmt::Display;


pub struct BassDevice {
    pub id: u32,
    pub name: String,
    pub driver: String,
    pub flags: Vec<DeviceFlags>
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

            list.push(Self::new(i, name, driver, info.flags));
            i += 1;
        }

        Ok(list)
    }

    fn new(id:u32, name:String, driver:String, flags: u32) -> Self {
        Self {
            id,
            name,
            driver,
            flags: flags.to_flags()
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

// pretty logging c:
impl Display for BassDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {} ({}): {:?}", self.id, self.name, self.driver, self.flags)
    }
}


fn pain(ugh: *const c_void) -> String {
    unsafe {
        CStr::from_ptr(ugh as *const i8).to_string_lossy().to_string()
    }
}