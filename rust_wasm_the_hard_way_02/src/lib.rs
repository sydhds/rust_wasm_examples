
// Enable this and compile, the resulting size is ~ 16kb
/*
static PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
#[no_mangle]
extern "C" fn nth_prime(n: usize) -> i32 {
    PRIMES[n]
}
*/

// In order to reduce the wasm file size, we can do the bounds checking
/*
static PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
#[no_mangle]
fn nth_prime(n: usize) -> i32 {
    PRIMES.get(n).copied().unwrap_or(-1)
}
*/

// Another way to do it (but using unsafe)
/*
static PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
#[no_mangle]
fn nth_prime(n: usize) -> i32 {
    unsafe { *PRIMES.get_unchecked(n) }
}
*/

#![no_std]
static PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];

#[no_mangle]
extern "C" fn nth_prime(n: usize) -> i32 {
    // PRIMES[n]
    PRIMES.get(n).copied().unwrap_or(-1)
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}
