
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
    });
}