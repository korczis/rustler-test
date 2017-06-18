extern crate sdr_rtl_sys;

use std::os::raw::{c_char, c_void};
use sdr_rtl_sys::DeviceHandle;

pub struct Wrapper {}

impl Wrapper {
    pub unsafe fn rtlsdr_get_device_count() -> u32 {
        debug!("rtlsdr_get_device_count()");
        sdr_rtl_sys::rtlsdr_get_device_count()
    }

    pub unsafe fn rtlsdr_get_device_name(index: u32) ->  *const c_char {
        debug!("rtlsdr_get_device_name({})", index);
        sdr_rtl_sys::rtlsdr_get_device_name(index)
    }

    pub unsafe fn rtlsdr_get_device_usb_strings(index: u32, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32 {
        debug!("rtlsdr_get_device_usb_strings({}, {:?}, {:?}, {:?})", index, manufact, product, serial);
        sdr_rtl_sys::rtlsdr_get_device_usb_strings(index, manufact, product, serial)
    }

    pub unsafe fn rtlsdr_get_index_by_serial(serial: *const c_char) -> i32 {
        debug!("rtlsdr_get_index_by_serial({:?})", serial);
        sdr_rtl_sys::rtlsdr_get_index_by_serial(serial)
    }

    pub unsafe fn rtlsdr_open(dev: *mut *mut DeviceHandle, index: u32) -> i32 {
        debug!("rtlsdr_open({:?}, {:?})", dev, index);
        sdr_rtl_sys::rtlsdr_open(dev, index)
    }

    pub unsafe fn rtlsdr_close(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_close({:?})", dev);
        sdr_rtl_sys::rtlsdr_close(dev)
    }

    pub unsafe fn rtlsdr_set_xtal_freq(dev: *mut DeviceHandle, rtl_freq: u32, tuner_freq: u32) -> i32 {
        debug!("rtlsdr_set_xtal_freq({:?}, {:?}, {:?})", dev, rtl_freq, tuner_freq);
        sdr_rtl_sys::rtlsdr_set_xtal_freq(dev, rtl_freq, tuner_freq)
    }

    pub unsafe fn rtlsdr_get_xtal_freq(dev: *mut DeviceHandle, rtl_freq: *mut u32, tuner_freq: *mut u32) -> i32 {
        debug!("rtlsdr_get_xtal_freq({:?}, {:?}, {:?})", dev, rtl_freq, tuner_freq);
        sdr_rtl_sys::rtlsdr_get_xtal_freq(dev, rtl_freq, tuner_freq)
    }

    pub unsafe fn rtlsdr_get_usb_strings(dev: *mut DeviceHandle, manufact: *mut c_char, product: *mut c_char, serial: *mut c_char) -> i32 {
        debug!("rtlsdr_get_usb_strings({:?}, {:?}, {:?}, {:?})", dev, manufact, product, serial);
        sdr_rtl_sys::rtlsdr_get_usb_strings(dev, manufact, product, serial)
    }

    pub unsafe fn rtlsdr_set_center_freq(dev: *mut DeviceHandle, freq: u32) -> i32 {
        debug!("rtlsdr_set_center_freq({:?}, {:?})", dev, freq);
        sdr_rtl_sys::rtlsdr_set_center_freq(dev, freq)
    }

    pub unsafe fn rtlsdr_get_center_freq(dev: *mut DeviceHandle) -> u32 {
        debug!("rtlsdr_get_center_freq({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_center_freq(dev)
    }

    pub unsafe fn rtlsdr_set_freq_correction(dev: *mut DeviceHandle, ppm: i32) -> i32 {
        debug!("rtlsdr_set_freq_correction({:?}, {:?})", dev, ppm);
        sdr_rtl_sys::rtlsdr_set_freq_correction(dev, ppm)
    }

    pub unsafe fn rtlsdr_get_freq_correction(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_get_freq_correction({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_freq_correction(dev)
    }

    pub unsafe fn rtlsdr_get_tuner_type(dev: *mut DeviceHandle) -> sdr_rtl_sys::TunerType {
        debug!("rtlsdr_get_tuner_type({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_tuner_type(dev)
    }

    pub unsafe fn rtlsdr_get_tuner_gains(dev: *mut DeviceHandle, gains: *mut i32) -> i32 {
        debug!("rtlsdr_get_tuner_gains({:?}, {:?})", dev, gains);
        sdr_rtl_sys::rtlsdr_get_tuner_gains(dev, gains)
    }

    pub unsafe fn rtlsdr_set_tuner_gain(dev: *mut DeviceHandle, gain: i32) -> i32 {
        debug!("rtlsdr_set_tuner_gain({:?}, {:?})", dev, gain);
        sdr_rtl_sys::rtlsdr_set_tuner_gain(dev, gain)
    }

    pub unsafe fn rtlsdr_get_tuner_gain(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_get_tuner_gain({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_tuner_gain(dev)
    }

    pub unsafe fn rtlsdr_set_tuner_if_gain(dev: *mut DeviceHandle, stage: i32, gain: i32) -> i32 {
        debug!("rtlsdr_set_tuner_if_gain({:?}, {:?}, {:?})", dev, stage, gain);
        sdr_rtl_sys::rtlsdr_set_tuner_if_gain(dev, stage, gain)
    }

    pub unsafe fn rtlsdr_set_tuner_gain_mode(dev: *mut DeviceHandle, manual: i32) -> i32 {
        debug!("rtlsdr_set_tuner_gain_mode({:?}, {:?})", dev, manual);
        sdr_rtl_sys::rtlsdr_set_tuner_gain_mode(dev, manual)
    }

    pub unsafe fn rtlsdr_set_sample_rate(dev: *mut DeviceHandle, rate: u32) -> i32 {
        debug!("rtlsdr_set_sample_rate({:?}, {:?})", dev, rate);
        sdr_rtl_sys::rtlsdr_set_sample_rate(dev, rate)
    }

    pub unsafe fn rtlsdr_get_sample_rate(dev: *mut DeviceHandle) -> u32 {
        debug!("rtlsdr_get_sample_rate({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_sample_rate(dev)
    }

    pub unsafe fn rtlsdr_set_testmode(dev: *mut DeviceHandle, on: i32) -> i32 {
        debug!("rtlsdr_set_testmode({:?}, {:?})", dev, on);
        sdr_rtl_sys::rtlsdr_set_testmode(dev, on)
    }

    pub unsafe fn rtlsdr_set_agc_mode(dev: *mut DeviceHandle, on: i32) -> i32 {
        debug!("rtlsdr_set_agc_mode({:?}, {:?})", dev, on);
        sdr_rtl_sys::rtlsdr_set_agc_mode(dev, on)
    }

    pub unsafe fn rtlsdr_set_direct_sampling(dev: *mut DeviceHandle, on: i32) -> i32 {
        debug!("rtlsdr_set_direct_sampling({:?}, {:?})", dev, on);
        sdr_rtl_sys::rtlsdr_set_direct_sampling(dev, on)
    }

    pub unsafe fn rtlsdr_get_direct_sampling(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_get_direct_sampling({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_direct_sampling(dev)
    }

    pub unsafe fn  rtlsdr_set_offset_tuning(dev: *mut DeviceHandle, on: i32) -> i32 {
        debug!("rtlsdr_set_offset_tuning({:?}, {:?})", dev, on);
        sdr_rtl_sys::rtlsdr_set_offset_tuning(dev, on)
    }

    pub unsafe fn rtlsdr_get_offset_tuning(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_get_offset_tuning({:?})", dev);
        sdr_rtl_sys::rtlsdr_get_offset_tuning(dev)
    }

    pub unsafe fn rtlsdr_reset_buffer(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_reset_buffer({:?})", dev);
        sdr_rtl_sys::rtlsdr_reset_buffer(dev)
    }

    pub unsafe fn rtlsdr_read_sync(dev: *mut DeviceHandle, buf: *mut c_char, len: i32, n_read: *mut i32) -> i32 {
        debug!("rtlsdr_read_sync({:?}, {:?}, {:?}, {:?})", dev, buf, len, n_read);
        sdr_rtl_sys::rtlsdr_read_sync(dev, buf, len, n_read)
    }

    pub unsafe fn rtlsdr_read_async(dev: *mut DeviceHandle, cb: sdr_rtl_sys::RxCallback, ctx: *const c_void, buf_num: u32, buf_len: u32) -> i32 {
        debug!("rtlsdr_read_async({:?}, {:?}, {:?}, {:?}, {:?})", dev, cb, ctx, buf_num, buf_len);
        sdr_rtl_sys::rtlsdr_read_async(dev, cb, ctx, buf_num, buf_len)
    }

    pub unsafe fn rtlsdr_cancel_async(dev: *mut DeviceHandle) -> i32 {
        debug!("rtlsdr_cancel_async({:?})", dev);
        sdr_rtl_sys::rtlsdr_cancel_async(dev)
    }
}
