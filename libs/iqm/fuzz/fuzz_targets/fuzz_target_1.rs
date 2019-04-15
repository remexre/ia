#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use byteorder::{ByteOrder, LittleEndian};

fuzz_target!(|data: &[u8]| {
    let mut data = data.to_vec();
    let l = data.len() as u32;
    if l >= 24 {
        LittleEndian::write_u32(&mut data[20..24], l);
        let _ = iqm::IQM::parse_from(&data);
    }
});
