// #![feature(never_type)]

// Bottom Type
fn foo() -> ! {
    loop {println!("foo")}
}

// Empty enumeration
enum Void {}
fn test_empty_enum() {
    let res: Result<u32, Void> = Ok(0);
    let Ok(num) = res else { todo!() }; // Add else can fix?
}

fn main() {
    let i = if false {
        foo();
    } else {
        100
    };
    assert_eq!(i, 100);
    test_empty_enum();
}
