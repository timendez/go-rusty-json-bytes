/**
 * Fair warning, since I don't know Rust, this is largely Chat GPT'd code with only a few minor tweaks from me.
 * If any Rustacean wants to modify it, I'll happily entertain a PR!
 */
use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct MyStringData {
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct MyByteData<'a> {
    field1: &'a [u8], // Raw bytes
    field2: i32,
}

// Function to marshal byte data to JSON and return a pointer to bytes and their length
#[no_mangle]
pub extern "C" fn marshal_bytes(field1: *const u8, field1_len: usize, field2: i32) -> (*mut c_void, usize) {
    // Use the raw byte slice directly
    let raw_field1 = unsafe { std::slice::from_raw_parts(field1, field1_len) };

    // Create the byte data structure
    let byte_data = MyByteData {
        field1: raw_field1,
        field2,
    };

    // Serialize to JSON and then to bytes
    let json_bytes = serde_json::to_vec(&byte_data).unwrap(); // Serialize to bytes directly

    // Allocate memory for the return value
    let mut boxed = json_bytes.into_boxed_slice(); // Declare as mutable
    let ptr = boxed.as_mut_ptr(); // Get a raw pointer to the allocated memory

    // Prevent Rust from freeing the memory
    std::mem::forget(boxed);

    (ptr as *mut c_void, json_bytes.len())
}

// Free the memory allocated for the bytes
#[no_mangle]
pub extern "C" fn free_bytes(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            // Free the memory allocated for the bytes
            let _ = Box::from_raw(ptr as *mut u8); // Correctly cast to u8 pointer
        }
    }
}


// Function to marshal a Rust string to JSON and return a C string
pub extern "C" fn marshal_json(field1: *const c_char, field2: i32) -> *mut c_char {
    // Convert raw C string to Rust string
    let rust_field1 = unsafe {
        assert!(!field1.is_null());
        // Create a CString from the raw pointer to read it as a Rust string
        let c_str = CString::from_raw(field1 as *mut c_char);
        c_str.into_string().unwrap() // Convert and take ownership
    };

    let string_data = MyStringData {
        field1: rust_field1,
        field2,
    };

    // Serialize the data to a JSON string
    let json_str = serde_json::to_string(&string_data).unwrap();

    // Convert the JSON string to a C string
    let c_str = CString::new(json_str).unwrap();
    c_str.into_raw()
}

// Free the memory allocated for the JSON C string
#[no_mangle]
pub extern "C" fn free_json(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s); }
    }
}
