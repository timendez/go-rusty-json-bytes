// rust-marshaller/src/lib.rs
use serde_json::json;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[derive(serde::Serialize)]
struct MyData {
    field1: String,
    field2: i32,
}

#[no_mangle]
pub extern "C" fn marshal_json() -> *mut c_char {
    let data = MyData {
        field1: "example".to_string(),
        field2: 123,
    };
    let json_str = serde_json::to_string(&data).unwrap();
    let c_str = CString::new(json_str).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn free_json(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s); }
    }
}
