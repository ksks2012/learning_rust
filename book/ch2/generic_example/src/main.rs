use std::fmt::Debug;

// std::option::Option
// enum Option<T> {
//     Some(T),
//     None,
// }

fn match_option<T: Debug>(option: Option<T>) {
    match option {
        Option::Some(t) => println!("{:?}", t),
        Option::None => println!("None"),
    }
}

fn main() {
    let a: Option<i32> = Some(13);
    let b: Option<&str> = Some("hello");
    let c: Option<char> = Some('A');
    let d: Option<u32> = None;
    match_option(a); // 13
    match_option(b); // "hello"
    match_option(c); // 'A'
    match_option(d); // None
}
