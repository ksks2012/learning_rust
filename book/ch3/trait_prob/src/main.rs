// orphan rule
use std::ops::Add;
#[derive(PartialEq)]
struct Int(i32);
impl Add<i32> for Int {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        (self.0) + other
    }
}

impl Add<i32> for Box<Int> {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        (self.0) + other
    }
}


fn test_orphan_rule() {
    assert_eq!(Int(3) + 3, 6);
    assert_eq!(Box::new(Int(3)) + 3, 6);
}

// overlap
// impl<T> AnyTrait for T {...}
// impl<T> AnyTrait for T where T: Copy {...}
// impl<T> AnyTrait for i32 {...}

// performance issue of overlap
// impl<R, T: Add<R> + Clone> AddAssign<R> for T {
//     fn add_assign(&mut self, rhs: R) {
//         let tmp = self.clone() + rhs;
//         *self = tmp;
//     }
// }

// Sepcialization
// #![feature(sepcialization)]
struct Diver<T> {
    inner: T,
}
trait Swimmer {
    fn swim(&self) {
        println!("swimming");
    }
}
impl<T> Swimmer for Diver<T> {
// Conflict -> not implement
// impl Swimmer for Diver<&'static str>  {
    fn swim(&self) {
        println!("drowning, help!");
    }
}

fn test_special() {
    let x = Diver::<&'static str> {
        inner: "Bob"
    };
    x.swim();
    let y = Diver::<String> {
        inner: String::from("Alice")
    };
    y.swim();
}

// Generic Associated Types
// trait StreamingIterator {
//      type Item<'a>;
//      fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
// }

fn main() {
    test_orphan_rule();
    test_special();
}
