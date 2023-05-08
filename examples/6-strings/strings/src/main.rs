fn print_str(s: &str) {
    let new_string = format!("{}! other stuff here", s);
    println!("{}", new_string);
}

fn print_string(mut s: String) {
    println!("{}", s);
}

fn main() {
    let s = "hello, world!";
    print_str(s);

    // String is growable and mutable whereas str is not.
    // String is owned by the code that creates it
    let mut salutation = String::from("hello");
    print_string(salutation);
}
