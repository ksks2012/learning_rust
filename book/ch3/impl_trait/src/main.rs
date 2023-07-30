use std::fmt::Debug;
// Refactor ch2/trait_example
pub trait Fly {
    fn fly(&self) -> bool;
}

#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;

impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}

fn fly_static(s: impl Fly+Debug) -> bool {
    s.fly()
}

fn can_fly(s: impl Fly+Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can not fly", s);
    }
    s
}

// dyn Trait
fn dyn_can_fly(s: impl Fly+Debug+'static) -> Box<dyn Fly> {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can not fly", s);
    }
    Box::new(s)
}

fn test_fly_static() {
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);
}

fn test_can_fly() {
    let pig = Pig;
    let _pig = can_fly(pig); // can not fly
    let duck = Duck;
    let _duck = can_fly(duck); // can fly
}

fn test_dyn_can_fly() {
    let pig = Pig;
    let _pig = dyn_can_fly(pig); // can not fly
    let duck = Duck;
    let _duck = dyn_can_fly(duck); // can fly
}


fn main() {
    test_fly_static();
    test_can_fly();
    test_dyn_can_fly();
}   
