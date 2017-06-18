use std::os::raw::{c_char, c_void};

pub enum DeviceHandle {}

#[derive(Debug)]
#[repr(C)]
pub enum Error {
    Success = 0,
    EmptyName = -1,
    NoDevices = -2,
    NoSuchDevice = -3,
}

#[derive(Debug)]
#[repr(C)]
pub enum TunerType {
    Unknown = 0,
    E4000,
    FC0012,
    FC0013,
    FC2580,
    R820T,
    R828D
}

#[derive(Debug)]
#[repr(C)]
pub enum DirectSampling {
    Disabled = 0,
    I,
    Q
}

impl DirectSampling {
    pub fn from(val: i32) -> Option<DirectSampling> {
        match val {
            0 => Some(DirectSampling::Disabled),
            1 => Some(DirectSampling::I),
            2 => Some(DirectSampling::Q),
            _ => None
        }
    }
}

pub type RxCallback = unsafe extern "C" fn(buf: *mut c_char, len: u32, ctx: *const c_void);

#[link(name = "rtlsdr")]
extern "C" {
    pub fn rtlsdr_get_device_count() -> u32;
    pub fn rtlsdr_get_device_name(index: u32) -> *const c_char;
    pub fn rtlsdr_get_device_usb_strings(index: u32, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32;
    pub fn rtlsdr_get_index_by_serial(serial: *const c_char) -> i32;

    pub fn rtlsdr_open(dev: *mut *mut DeviceHandle, index: u32) -> i32;
    pub fn rtlsdr_close(dev: *mut DeviceHandle) -> i32;

    pub fn rtlsdr_set_xtal_freq(dev: *mut DeviceHandle, rtl_freq: u32, tuner_freq: u32) -> i32;
    pub fn rtlsdr_get_xtal_freq(dev: *mut DeviceHandle, rtl_freq: *mut u32, tuner_freq: *mut u32) -> i32;

    pub fn rtlsdr_get_usb_strings(dev: *mut DeviceHandle, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32;

//    pub fn rtlsdr_write_eeprom(dev: *mut DeviceHandle, data: *mut u8, offset: *mut u8, len: u16) -> i32;
//    pub fn rtlsdr_read_eeprom(dev: *mut DeviceHandle, data: *mut u8, offset: *mut u8, len: u16) -> i32;

    pub fn rtlsdr_set_center_freq(dev: *mut DeviceHandle, freq: u32) -> i32;
    pub fn rtlsdr_get_center_freq(dev: *mut DeviceHandle) -> u32;

    pub fn rtlsdr_set_freq_correction(dev: *mut DeviceHandle, ppm: i32) -> i32;
    pub fn rtlsdr_get_freq_correction(dev: *mut DeviceHandle) -> i32;

    pub fn rtlsdr_get_tuner_type(dev: *mut DeviceHandle) -> TunerType;

    pub fn rtlsdr_get_tuner_gains(dev: *mut DeviceHandle, gains: *mut i32) -> i32;
    pub fn rtlsdr_set_tuner_gain(dev: *mut DeviceHandle, gain: i32) -> i32;

    pub fn rtlsdr_get_tuner_gain(dev: *mut DeviceHandle) -> i32;
    pub fn rtlsdr_set_tuner_if_gain(dev: *mut DeviceHandle, stage: i32, gain: i32) -> i32;
    pub fn rtlsdr_set_tuner_gain_mode(dev: *mut DeviceHandle, manual: i32) -> i32;

    pub fn rtlsdr_set_sample_rate(dev: *mut DeviceHandle, rate: u32) -> i32;
    pub fn rtlsdr_get_sample_rate(dev: *mut DeviceHandle) -> u32;

    pub fn rtlsdr_set_testmode(dev: *mut DeviceHandle, on: i32) -> i32;
    pub fn rtlsdr_set_agc_mode(dev: *mut DeviceHandle, on: i32) -> i32;
    pub fn rtlsdr_set_direct_sampling(dev: *mut DeviceHandle, on: i32) -> i32;
    pub fn rtlsdr_get_direct_sampling(dev: *mut DeviceHandle) -> i32;
    pub fn rtlsdr_set_offset_tuning(dev: *mut DeviceHandle, on: i32) -> i32;
    pub fn rtlsdr_get_offset_tuning(dev: *mut DeviceHandle) -> i32;

    pub fn rtlsdr_reset_buffer(dev: *mut DeviceHandle) -> i32;
    pub fn rtlsdr_read_sync(dev: *mut DeviceHandle, buf: *mut c_char, len: i32, n_read: *mut i32) -> i32;

    pub fn rtlsdr_read_async(dev: *mut DeviceHandle, cb: RxCallback, ctx: *const c_void, buf_num: u32, buf_len: u32) -> i32;

    pub fn rtlsdr_cancel_async(dev: *mut DeviceHandle) -> i32;
}
