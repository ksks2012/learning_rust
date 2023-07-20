enum Void {}
struct Foo;
struct Baz {
    foo: Foo,
    qux: (),
    baxz: [u8; 0],
}

// Zero size
fn test_zero_size() {
    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);
}

// Repeated operations: Vec<()>
fn test_repeated_operations() {
    let v: Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }
}

fn main() {
    test_zero_size();
    test_repeated_operations();
}
