use super::super::*;

pub fn main() {
    let count = Device::get_device_count();
    println!("Count: {}", count);

//    println!("Device Name: {:?}", Device::get_device_name(0));
//    println!("USB Strings: {:?}", Device::get_device_usb_strings(0));
}
