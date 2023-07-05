fn for_loop() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// fn while_true(x: i32) -> i32 {
//     while true {
//         return x + 1;
//     }
// }

fn while_true_fix(x: i32) -> i32 {
    while true { // warning to change to loop
        return x + 1;
    }
    x // warning of unreachable code
}

fn main() {
    for_loop();
    let y = while_true_fix(5);
    assert_eq!(y, 6);
}
