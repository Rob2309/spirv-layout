#![no_main]

use libfuzzer_sys::fuzz_target;

use spirv_layout::*;
use std::slice;

fuzz_target!(|data: &[u8]| {
    target(data);
});

fn target(data: &[u8]) {
    let words = unsafe{slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4)};
    let _module = Module::from_words(words);
}
