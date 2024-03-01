#![no_std]

// One of the few occasions where we have to use `extern crate`
// even in Rust Edition 2021.
extern crate alloc;
use alloc::vec::Vec;

use cfg_if::cfg_if;

#[no_mangle]
extern "C" fn nth_prime(n: usize) -> usize {
    // Please enjoy this horrible implementation of
    // The Sieve of Eratosthenes.
    let mut primes: Vec<usize> = Vec::new();
    let mut current = 2;
    while primes.len() < n {
        if !primes.iter().any(|prime| current % prime == 0) {
            primes.push(current);
        }
        current += 1;
    }
    primes.into_iter().last().unwrap_or(0)
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}

cfg_if! {
    if #[cfg(feature = "lol_alloc")] {
        use lol_alloc::LeakingPageAllocator;
        #[global_allocator]
        static ALLOCATOR: LeakingPageAllocator = LeakingPageAllocator;
    } else if #[cfg(feature = "custom_alloc")] {
        mod alloc_custom;
        #[global_allocator]
        static ALLOCATOR: SimpleAllocator = SimpleAllocator::new();
    }
}