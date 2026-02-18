/// Test 1: Intensive Calculations in a Loop
pub fn test_loop_computations() -> u64 {
    let mut sum = 0u64;
    let n = 10_000_000;
    for i in 0..n {
        sum = sum.wrapping_add(i * 3 + i / 2);
    }
    sum
}

/// Test 2: Working with Dynamic Memory (Vector)
pub fn test_vector_operations() -> usize {
    let mut vec = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        vec.push(i * 2);
    }
    vec.iter().sum::<usize>()
}

// Test 3: Floating-Point Calculations
pub fn test_float_calculations() -> f64 {
    let mut result = 0.0;
    for i in 1..=2_000_000 {
        let x = i as f64;
        result += (x * 0.5).sin() + (x * 2.0).sqrt();
    }
    result
}
