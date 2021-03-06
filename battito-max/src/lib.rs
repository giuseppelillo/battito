extern crate libc;

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;

#[repr(C)]
pub struct Event {
    pub value: u32,
    pub probability: u8, // [0, 100]
}

#[repr(C)]
pub struct Pattern {
    pub events: *const Event,
    pub length: u32,
}

#[no_mangle]
pub extern "C" fn transform(ptr: *const c_char, subdivision: u32) -> Pattern {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    let pattern = battito_lib::pattern::transform(cstr.to_str().unwrap(), Some(subdivision))
        .unwrap_or(battito_lib::pattern::pattern::Pattern::empty(subdivision));
    let filled = pattern.fill();

    let v: Vec<Event> = filled
        .into_iter()
        .map(|te| Event {
            value: te.value.parse().unwrap_or(0),
            probability: te.probability,
        })
        .collect();

    let pointer = v.as_ptr();
    mem::forget(v);

    Pattern {
        events: pointer,
        length: pattern.length,
    }
}
