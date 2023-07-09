// Allow struct to print and compare
#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: u32
}

impl People {
    fn new(name: &'static str, gender: u32) -> Self {
        return People{name: name, gender: gender};
    }

    fn name(&self) {
        println!("name {:?}", self.name);
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn gender(&self) {
        let _gender = if self.gender == 1 {"boy"} else {"girl"};
        println!("gender {:?}", self.gender);
    }
}

fn test_people() {
    // No constructor, so this is static method?
    let alex = People::new("Alex", 1);
    alex.name();
    alex.gender();
    assert_eq!(alex, People{name:"Alex", gender: 1});
    let mut alice = People::new("Alice", 0);
    alice.name();
    alice.gender();
    assert_eq!(alice, People{name:"Alice", gender: 0});
    alice.set_name("Rose");
    alice.name();
    assert_eq!(alice, People{name:"Rose", gender: 0});
}

// tuple structure
struct Color(i32, i32, i32);
fn test_color() {
    let color = Color(0, 1, 2);
    assert_eq!(color.0, 0);
    assert_eq!(color.1, 1);
    assert_eq!(color.2, 2);
}

// new type
struct Interger(u32);
type Int = i32;
fn test_new_type() {
    let int = Interger(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);
}

// unit structure
// Debug: x, y, z have different memory
// Release: x, y, z have same memory (Optimization)
struct Empty;
fn test_unit_stuct() {
    let x = Empty;
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
    let z = Empty;
    println!("{:p}", &z);
    assert_eq!((..), std::ops::RangeFull);
}

fn main() {
    test_people();
    test_color();
    test_new_type();
    test_unit_stuct();
}
