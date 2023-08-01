use std::thread;

// Note: Lang Item standard implementation
// #[lang = "sized"]
// pub trait Sized {

// }

struct Foo<T>(T);
struct Bar<T: ?Sized>(T);


// Note: Copy standard implementation
// #[lang="copy"]
// pub trait Copy: Clone {

// }

// Clone trait standard implementation
// pub trait Clone : Sized {
//     fn clone(&self) -> Self;
//     fn clone_from(&mut self, source: &Self) {
//         *self = source.clone();
//     }
// }

// Implement
// struct MyStruct;
// impl Copy for MyStruct {}
// impl Clone for MyStruct {
//     fn clone(&self) -> MyStruct {
//         *self
//     }
// }

// Use derive
#[derive(Copy, Clone)]
struct MyStruct;


// Note: Compile error
// fn test_copy<T: Copy>(i: T) {
//     println!("HAHAHA");
// }

fn test_thread_spawn() {
    let mut x = vec![1, 2, 3, 4];
    thread::spawn(|| x);
}

fn test_thread_move() {
    let mut x = vec![1, 2, 3, 4];
    thread::spawn(move || x.push(5));
    // x.push(2); // Error after borrowed
}

// Note: Implent Send and Sync for all types
unsafe imp Send for .. {}
impl<T: ?Sized> !Send for *const T {}
impl<T: ?Sized> !Send for *mut T {}

fn main() {
    // let a = "String".to_string();
    // test_copy(a);

    test_thread_spawn();
    test_thread_move();
}
