#[allow(non_snake_case)]
mod myMath;

#[allow(non_snake_case)]
mod myMath_ffi;

#[no_mangle]
pub extern "C" fn rust_add(x: i32, y: i32) -> i32 {
    x + y
}

use ::std::os::raw::c_int;

#[no_mangle]
pub unsafe extern "C" fn rust_pow(x: c_int, y: c_int) -> c_int {
    let mut ret: c_int = 1;
    for _ in 1..=y {
        ret = myMath::myMath_mul(ret, x);
    }
    ret
}