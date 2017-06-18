use clap::ArgMatches;

use std::os::raw::{c_void};
use std::slice;

use super::super::*;

pub fn main(_args: &ArgMatches) {
    match Device::open(0 as u32) {
        Some(mut dev) => {
            println!("XTAL Freq:     {:?}", dev.get_xtal_freq());
            println!("Model:         {:?}", dev.get_usb_strings());
            println!("Frequency:     {:?}", dev.get_center_freq());
            println!("Correction     {:?}", dev.get_freq_correction());
            println!("Type           {:?}", dev.get_tuner_type());
            println!("Gains          {:?}", dev.get_tuner_gains());
            println!("Gain           {:?}", dev.get_tuner_gain());
            println!("Sample Rate:   {:?}", dev.get_sample_rate());
            println!("Direct:        {:?}", dev.get_direct_sampling());
            println!("Offset Tuning: {:?}", dev.get_offset_tuning());

//            dev.set_testmode(1);
//            dev.set_center_freq(92_606_000);
//            dev.set_sample_rate(2_048_000);
//            dev.reset_buffer();
//
//            unsafe extern "C" fn rx_callback(buf: *mut c_char, len: u32, _ctx: *const c_void) {
//                println!("{:?}", len);
//
//                let slice = slice::from_raw_parts(buf, len as usize);
//                println!("{:?}", slice);
//            }
//
//            let res = dev.read_async(rx_callback, ptr::null(), 0, 0);
//            println!("{:?}", res);
        },
        None => {
            println!("Unable to open device");
        }
    }
}
