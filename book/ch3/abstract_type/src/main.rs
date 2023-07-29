#[derive(Debug)]
struct Foo;
trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T>(t: &T) where T: Bar {
    t.baz();
}

fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}

fn test_triat() {
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}

// strutct equal to trait
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

// Safe trait object
trait SafeBar {
    fn bax(self, x: u32);
    fn bay(&self);
    fn baz(&mut self);
}

// Unsafe trait object
trait UnSafeFoo {
    fn bad<T>(&self, x: T);
    fn new() -> Self;
}

// Fix
trait FixFoo {
    fn bad<T>(&self, x: T);
}

trait FixFoo2: SafeBar {
    fn new() -> Self;
}

// Safe trait object with where
trait FooWhere {
    fn bad<T>(&self, x: T);
    fn new() -> Self where Self: Sized;
}

fn main() {
    println!("Hello, world!");
}
