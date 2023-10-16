struct S(i32);
trait A {
    fn test(&self, i : i32);
}

trait B {
    fn test(&self, i : i32);
}

impl A for S {
    fn test(&self, i : i32) {
        println!("From A: {:?}", i);
    }
}

impl B for S {
    fn test(&self, i : i32) {
        println!("From B: {:?}", i+1);
    }
}

// fully_qualified_syntax_for_disambiguation
fn test_ufcs() {
    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 1);
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);
}

fn test_sub_class() {
    let a: &'static str = "hello world"; // &'static str
    println!("a = {:?}", type(a));
    let b: &str = a as &str; // &str
    println!("b = {:?}", b);
    let c: &'static str = b as &'static str; // &'static str
    println!("c = {:?}", c);
}

fn main() {
    test_ufcs();
    test_sub_class();
}
