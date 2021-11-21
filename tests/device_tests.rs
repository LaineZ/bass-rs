use bass::prelude::*;
use bass::__bass_check;

#[test]
pub fn device_tests() -> BassResult<()> {
    let all_devices = __bass_check!(BassDevice::get_all_devices());
    println!("Devices: ");
    for device in all_devices {
        println!("{}. {} ({})", device.id, device.name, device.driver)
    }

    Ok(())
}