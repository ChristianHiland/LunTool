use std::os::raw::{c_char};
use std::ffi::CStr;

#[warn(dead_code)]
pub fn convert_c_char_to_string(c_ptr: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(c_ptr) };
    let result = format!("{}", c_str.to_str().unwrap().to_string());
    result
}

#[warn(dead_code)]
pub fn convert_c_char_to_string_lossy(c_ptr: *const c_char) -> String {
    if c_ptr.is_null() {
        // Handle null pointer appropriately, perhaps return an empty string or panic
        return String::new(); // Or consider panicking: panic!("Null pointer received");
    }
    // SAFETY: The caller must ensure that c_ptr is a valid, null-terminated
    // C string with a valid lifetime.
    let c_str = unsafe { CStr::from_ptr(c_ptr) };
    c_str.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_convert_const_cchar() {
        let result = convert_c_char_to_string("Hello Testing\0".as_ptr() as *const c_char);
        assert_eq!(result, "Hello Testing".to_string());
    }

    #[test]
    fn types_other_test() {
        let c_char_ptr = "Hello Testing\0".as_ptr() as *const c_char;
        if c_char_ptr.is_null() {
            panic!("C char pointer is null!");
        }

        let c_str = unsafe { std::ffi::CStr::from_ptr(c_char_ptr) };

        // Debugging
        println!("CStr as seen by Rust: {:?}", c_str); // Shows content + checks for internal nulls
        println!("CStr bytes: {:?}", c_str.to_bytes());
        println!("CStr length (excluding null): {}", c_str.to_bytes().len());

        let result = format!("{}", c_str.to_str().unwrap().to_string());
        assert_eq!(result, "Hello Testing".to_string());
    }
}