// #![feature(const_mut_refs)]
const fn init_len() -> usize {
    return 5;
}

fn main() {
    let arr = [0 ; init_len()];
    println!("{:?}", arr)
}
