// enum without parameter
enum Number {
    Zero,
    One,
    Two,
}

fn test_enum_without_parameter() {
    let a = Number::One;
    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    };
}

// c enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn test_c_enum() {
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violetes are #{:06x}", Color::Blue as i32);
}

// enum with parameter
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Fn-Pointer Type
fn test_enum_with_parameter() {
    let x : fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    let y : fn(String) -> IpAddr = IpAddr::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
}

// Enum example
// enum Option {
//     Some(i32),
//     None,
// }

// fn test_enum_example() {
//     let s = Some(42);
//     let num = s.unwrap();
//     match s {
//         Some(n) => println!("num is {}", n),
//         None => (),
//     };
// }

// Option <T>
fn test_option() {
    let s: &Option<String> = &Some("hello".to_string());
    // Rust 2015 ver
    match s {
        &Some(ref s) => println!("s is: {}", s),
        _ => (),
    };
    // Rust 2018 ver
    match s {
        Some(n) => println!("s is {}", n),
        _ => (),
    };
}

fn main() {
    test_enum_without_parameter();
    test_c_enum();
    test_enum_with_parameter();
    // test_enum_example();
    test_option();
}
