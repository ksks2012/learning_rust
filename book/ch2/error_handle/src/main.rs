use std::fs::File;

fn test_result() {
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}

fn main() -> Result<(), std::io::Error> {
    test_result();

    // return result from main
    let f = File::open("bar.txt")?;
    Ok(())
}
