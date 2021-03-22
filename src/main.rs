#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/sample.rs"));

fn main() {
    unsafe {
        let mut sample = Sample::new(5);
        sample.print();
    }
}
