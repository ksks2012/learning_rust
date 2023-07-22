trait MyAdd<RHS, Output> {
    fn my_add(self, rhs: RHS) -> Output;
}

impl MyAdd<i32, i32> for i32 {
    fn my_add(self, rhs: i32) -> i32 { 
        self + rhs 
    }
}

impl MyAdd<u32, u32> for u32 {
    fn my_add(self, rhs: u32) -> u32 { 
        (self + rhs).try_into().unwrap() // ?
    }

}

fn test_add_trait() {
    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x: i32 = a.my_add(b);
    let y: i32 = c.my_add(d).try_into().unwrap(); // ?
    assert_eq!(x, 3i32);
    assert_eq!(y, 7i32);
}

// Overload
// Add to crate
trait OverloadAdd<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl OverloadAdd<u64> for u32 {
    type Output = u64;
    fn add(self, other: u64) -> Self::Output {
        (self as u64) + other
    }
}

fn test_overload() {
    let a = 1u32;
    let b = 2u64;
    assert_eq!(a.add(b), 3)
}



fn main() {
    test_add_trait();
    test_overload();
}
