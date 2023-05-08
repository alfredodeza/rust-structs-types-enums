#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

fn main() {
    let alfredo = Person{
        first_name: "Alfredo".to_string(),
        last_name: "Sanchez".to_string(),
        age: Some(23),
    };
    println!("The person's first name is: {}", alfredo.first_name);
    println!("The person's age is: {:?}", alfredo.age);
}