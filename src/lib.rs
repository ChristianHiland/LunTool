use types::{*};
use structs::{*};

mod types;
mod structs;

#[cfg(test)]
mod tests {
    use std::os::raw::{c_char};
    use super::*;

    #[test]
    fn convert_const_cchar() {
        let result = convert_c_char_to_string("Hello Testing\0".as_ptr() as *const c_char);
        assert_eq!(result, "Hello Testing".to_string());
    }

    #[test]
    fn testing_Bits() {
        let u8V: i8 = 8;
        let u16V: i16 = 16;
        let u32V: i32 = 32;
        let u64V: u64 = 64;
        
        println!("u8: {:#010b}\nu16: {:#010b}\nu32: {:#010b}\nu64: {:#010b}", u8V, u16V,  u32V, u64V);
        assert_eq!(0b00001000, u8::BITS);
    }
}
