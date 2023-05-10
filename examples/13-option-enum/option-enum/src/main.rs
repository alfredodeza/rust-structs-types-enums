fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None // This is valid because it is the other variant of Option
    } else {
        Some(x / y) // Creates the Option<i32> value. Some() creates a new instance of Option
    }
}

fn main() {
    let a = 10;
    let b = 2;

    let result = divide(a, b);

    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }
}
