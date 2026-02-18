use web_time::Instant;

use wasm_bindgen::prelude::*;

/// Wasm
#[wasm_bindgen]
pub fn ydx_wasm_performance_test() -> u128 {
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
    start.elapsed().as_millis()
}
