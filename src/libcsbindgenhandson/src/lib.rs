#[allow(non_snake_case)]
mod myMath;

#[allow(non_snake_case)]
mod myMath_ffi;

#[no_mangle]
pub extern "C" fn rust_add(x: i32, y: i32) -> i32 {
    x + y
}