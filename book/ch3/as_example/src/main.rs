fn test_basic_as() {
    let a = 1u32;
    let b = a as u64;
    let c = 3u64;
    let d = c as u32;
}

fn test_trans_split() {
    let a = std::u32::MAX; //4294967295
    let b = a as u16;
    assert_eq!(b, 65535);
    let e = -1i32;
    let f = e as u32;
    println!("{:?}", e.abs()); // 1
    println!("{:?}", f); // 4294967295
}



fn main() {
    test_basic_as();
    test_trans_split();
}
