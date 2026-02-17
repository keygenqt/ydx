#[cfg(not(target_family = "wasm"))]
use std::time::Instant;
#[cfg(target_family = "wasm")]
use web_time::Instant;

use wasm_bindgen::prelude::*;
use ::safer_ffi::prelude::*;

use jni::JNIEnv;
use jni::objects::{JClass, JString};

/// Linux C
#[ffi_export]
pub fn shared_performance_test() -> u64 {
    performance_test() as u64
}

/// Android JNI
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_MainActivity_performanceTest<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    _input: JString<'local>,
) -> u128 {
    performance_test()
}

/// Wasm
#[wasm_bindgen]
pub fn performance_test() -> u128 {
    // Get time
    let start = Instant::now();
    // Run functions
    let mut counter = 0.0;
    for _ in 0..100 {
        counter += test_loop_computations() as f64;
        counter += test_vector_operations() as f64;
        counter += test_float_calculations() as f64;
    }
    println!("counter: {:?}", counter);
    // Result time
    start.elapsed().as_millis()
}

/// Test 1: Intensive Calculations in a Loop
fn test_loop_computations() -> u64 {
    let mut sum = 0u64;
    let n = 10_000_000;
    for i in 0..n {
        sum = sum.wrapping_add(i * 3 + i / 2);
    }
    sum
}

/// Test 2: Working with Dynamic Memory (Vector)
fn test_vector_operations() -> usize {
    let mut vec = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        vec.push(i * 2);
    }
    vec.iter().sum::<usize>()
}

// Test 3: Floating-Point Calculations
fn test_float_calculations() -> f64 {
    let mut result = 0.0;
    for i in 1..=2_000_000 {
        let x = i as f64;
        result += (x * 0.5).sin() + (x * 2.0).sqrt();
    }
    result
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("./libs/libcore.h")?
        .generate()
}
