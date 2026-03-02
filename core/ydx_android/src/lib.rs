use std::time::Instant;

use jni::JNIEnv;
use jni::objects::{JClass, JString};

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_MainActivity_performanceTest<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    _input: JString<'local>,
) -> u128 {
    // Get time
    let start = Instant::now();
    // Run functions
    let mut counter = 0.0;
    for _ in 0..100 {
        counter += ydx_core::utils::tests::test_loop_computations() as f64;
        counter += ydx_core::utils::tests::test_vector_operations() as f64;
        counter += ydx_core::utils::tests::test_float_calculations() as f64;
    }
    println!("counter: {:?}", counter);
    // Result time
    start.elapsed().as_millis()
}
