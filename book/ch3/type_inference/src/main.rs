fn sum(a: u32, b: i32) -> u32 {
    a + (b as u32)
}

fn test_type_inference() {
    let a = 1;
    let b = 2;
    assert_eq!(sum(a, b), 3);

    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    assert_eq!(vec, [5]);
}

fn test_user_type() {
    let x = "1";
    let int_x: i32 = x.parse().unwrap();
    assert_eq!(int_x, 1);
}

// turbofish
fn test_turbofish() {
    let x = "1";
    let int_x = x.parse::<i32>().unwrap();
    assert_eq!(int_x, 1);
}

fn main() {
    test_type_inference();
    test_user_type();
    test_turbofish();
}
