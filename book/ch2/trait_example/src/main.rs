struct Duck;
struct Pig;

trait Fly {
    fn fly(&self) -> bool;
}

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

fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}

fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}


fn main() {
    let pig = Pig;
    assert_eq!(fly_static::<Pig>(pig), false);
    assert_eq!(fly_dyn(&Pig), false);
    let duck = Duck;
    assert_eq!(fly_static::<Duck>(duck), true);
    assert_eq!(fly_dyn(&Duck), true);
}   
