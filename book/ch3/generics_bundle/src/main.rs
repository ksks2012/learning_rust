use std::ops::Add;

// trait bound
fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b 
}

fn test_generic_sum() {
    assert_eq!(sum(1u32, 2u32), 3u32);
    assert_eq!(sum(1u64, 2u64), 3u64);
}

fn main() {
    test_generic_sum();
}
