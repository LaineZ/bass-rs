use crate::prelude::*;


pub struct BassDevice {
    pub id: u32,
    device_info: BassDeviceInfo
}
impl BassDevice {
    pub fn get_all_devices() -> BassResult<Vec<BassDevice>> {
        let mut list = Vec::new();

        let mut i = 1;
        let mut last_ok = 1;

        while last_ok == 1 {

            let mut info = empty_device_info();
            last_ok = BASS_GetDeviceInfo(i, &mut info);

            let device = Self::new(i, info);

            list.push(device);
            i += 1;
        }

        Ok(list)
    }

    fn new(id:u32, device_info:BassDeviceInfo) -> Self {
        Self {
            id,
            device_info
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

    /// get the name of the device
    pub fn name(&self) -> String {
        let name = unsafe {
            std::ffi::CStr::from_ptr(self.device_info.name as *const i8)
        };

        name.to_string_lossy().to_string()
    }
}


fn empty_device_info() -> BassDeviceInfo {
    BassDeviceInfo::new(
        "".as_ptr() as *mut std::ffi::c_void,
        "".as_ptr() as *mut std::ffi::c_void,
        0
    )
}