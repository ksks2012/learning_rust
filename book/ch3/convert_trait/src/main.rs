// Inner implementation
// pub trait From<T> {
//     fn from(T) -> Self;
// }

// pub trait Into<T> {
//     fn into(T) -> Self;
// }

fn test_from_string() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
}

#[derive(Debug)]
#[warn(dead_code)]
struct Person { name: String }
impl Person {
    fn new<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}
// impl<T, U> Into<U> for T where U: From<T>

fn test_into_string() {
    let person = Person::new("Alex");
    println!("{:?}", person);
    let person = Person::new("Alex".to_string());
    println!("{:?}", person);
    println!("{:?}", person.name);
}

fn test_into() {
    let a = "hello";
    let b: String = a.into();
    println!("{:?}", b);
}

fn main() {
    test_from_string();
    test_into_string();
    test_into();
}
