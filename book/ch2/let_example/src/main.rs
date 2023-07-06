fn if_let() {
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }
    assert_eq!(binary, 1);
}

fn match_example() {
    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
}

fn while_let() {
    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) =  v.pop() {
        println!("{}", x);
    }
    
}

fn main() {
    if_let();
    match_example();
    while_let();
}
