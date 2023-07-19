// Array as function arguments
// Need to define size of array in arguments
fn test_array_as_function_arguments(mut arr: [u32; 3]) {
    arr[0] = 3;
    arr[1] = 4;
    arr[2] = 5;
    println!("reset arr {:?}", arr);
}

// Using fat pointer (with reference and size)
fn test_with_fat_pointer(arr: &mut [u32]) {
    arr[0] = 3;
    arr[1] = 4;
    arr[2] = 5;
    println!("reset arr {:?}", arr);
}

// Size between two types
fn test_size_between_two_types() {
    assert_eq!(std::mem::size_of::<&[u32; 5]>(), 8);
    assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);
}

fn main() {
    let arr: [u32; 3] = [1, 2, 3];
    test_array_as_function_arguments(arr);
    println!("origin arr {:?}", arr);
    let mut arr: [u32; 3] = [1, 2, 3];
    test_with_fat_pointer(&mut arr);
    println!("reset after arr {:?}", arr);
    test_size_between_two_types();
}
