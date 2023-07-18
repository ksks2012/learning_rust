fn main() {
    let str = "Hello, world!";
    let ptr =  str.as_ptr(); // Fat Pointer
    let len = str.len();
    println!("prt: {:p}, len: {}", ptr, len);
}
