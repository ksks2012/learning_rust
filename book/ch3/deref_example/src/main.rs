use std::rc::Rc;

// Note: Rust implementation
// pub trait Deref {
//     type Target: ?Sized;
//     fn deref(&self) -> &Self::Target;
// }

// pub trait DerefMut: Deref {
//     fn deref_mut(&mut self) -> &mut Self::Target
// }

// Note: Deref<Target=str>
// &String -> &str
// impl ops:Deref for String {
//     type Target = str;
//     fn deref(&self) -> &str {
//         unsafe { str::from_utf8_unchecked(&self.vec) }
//     }
// }

fn test_link_string() {
    let a = "hello".to_string();
    let b = " world".to_string();
    let c = a + &b;
    println!("{:?}", c);
}

// implementation of deref for Vec<T>
fn foo(s: &[i32]) {
    println!("{:?}", s[0]);
}

fn test_vector_deref() {
    let v= vec![1,2,3];
    foo(&v);
}

// implement Rc pointer for deref
fn test_rc_pointer_deref() {
    let x = Rc::new("hello");
    // Rc<T> implements Deref<Target<T>>
    println!("{:?}", x.chars());
}

fn test_rc_pointer_user_set() {
    let x = Rc::new("hello");
    let y = x.clone(); // Rc<&str>
    let z = (*x).clone(); // &str
}

fn test_match() {
    let x = "hello".to_string();
    match &*x {
        "hello" => { println!("hello") },
        _ => {}
    }
}

fn main() {
    test_link_string();
    test_vector_deref();
    test_rc_pointer_deref();
    test_rc_pointer_user_set();
    test_match();
}
