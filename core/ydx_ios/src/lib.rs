use std::time::Instant;

use ::safer_ffi::prelude::*;

#[ffi_export]
pub fn ydx_ios_performance_test() -> u64 {
    // Get time
    let start = Instant::now();
    // Run functions
    let mut counter = 0.0;
    for _ in 0..100 {
        counter += ydx_core::test_loop_computations() as f64;
        counter += ydx_core::test_vector_operations() as f64;
        counter += ydx_core::test_float_calculations() as f64;
    }
    println!("counter: {:?}", counter);
    // Result time
    start.elapsed().as_millis() as u64
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("build/headers/ydx_ios.h")?
        .generate()
}
