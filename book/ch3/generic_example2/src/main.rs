// Generic functions
// 3.16
fn foo<T>(x: T) -> T {
    return x;
}

fn test_generic_function() {
    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");
}

#[derive(Debug, PartialEq)]
// Generic structs
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}

fn test_generic_struct() {
    let point1 = Point::new(1, 2);
    assert_eq!(point1, Point { x: 1, y: 2 });
    let point2 = Point::new("1", "2");
    assert_eq!(point2, Point { x: "1", y: "2" });
}

// Monomorphization
// 3.20: compile monomorphization to 3.16
fn foo_1(x: i32) -> i32 {
    x
}

fn foo_2(x: &'static str) -> &'static str {
    x
}

// Auto inference
#[derive(Debug, PartialEq)]
struct Foo(i32);
#[derive(Debug, PartialEq)]
struct Bar(i32, i32);
trait Inst {
    fn new(x: i32) -> Self;
}

impl Inst for Foo {
    fn new(x: i32) -> Self {
        Foo(x)
    }
}

impl Inst for Bar {
    fn new(x: i32) -> Self {
        Bar(x, x + 10)
    }
}

fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)
}

fn test_auto_inference() {
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));
    
    let b: Bar = foobar(10);
    assert_eq!(b, Bar(10, 20));
}

fn main() {
    test_generic_function();
    test_generic_struct();
    test_auto_inference();
}
