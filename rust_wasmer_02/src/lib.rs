#![allow(clippy::missing_safety_doc)]

#[no_mangle]
pub unsafe extern "C" fn sum(values: *const u8, len: u8) -> i64 {
    let slice = unsafe { std::slice::from_raw_parts(values, usize::from(len)) };
    i64::from(slice.iter().sum::<u8>())
}

#[no_mangle]
pub unsafe extern "C" fn uppercase(message: *const u8, len: u8) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(message, usize::from(len)) };
    // TODO: no unwrap()
    let message_ = String::from_utf8(slice.to_vec()).unwrap();
    let message_up = message_.to_uppercase();
    let ptr = message_up.as_ptr();
    std::mem::forget(message_up);
    ptr
}
