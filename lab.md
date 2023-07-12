# Lab: Create a File Size Formatter

In this lab, you will enhance a size formatter application in Rust. Use the [example code](https://github.com/alfredodeza/rust-structs-types-enums/blob/main/examples/14-match-enums/match-enum/src/main.rs) to get started and get an idea on how to use enum and match to handle different sizes. You are tasked with extending the application to allow a user to pass in a String representing size and unit, and then returning a debug representation of a struct that shows all the different representations in KB, MB, and GB . This is an example that takes an input and provides the output required:

```
$ cargo run "24 mb"
Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
```

The end result will be a GitHub repository containing the complete code for the enhanced file reader application.


## Learning Objectives:

- Understand how to define and use enums and structs in Rust.
- Practice error handling using match expressions and handling invalid input.
- Gain familiarity with string parsing and formatting in Rust.
- Learn how to implement the Debug trait to print debug information of a struct.
- Enhance code readability by leveraging Rust's string formatting capabilities.



## Steps:

1. Create a new repository in your account for your Rust project. Use the [Rust template repository](https://github.com/alfredodeza/rust-template) to quickly generate one for your own account. Use this link to [create it in one step](https://github.com/alfredodeza/rust-template/generate).
1. Use the [example code](https://github.com/alfredodeza/rust-structs-types-enums/blob/main/examples/14-match-enums/match-enum/src/main.rs) as a starting point.
1. Add the ability to pass in any file size like "300 kb" or "12 mb"  to avoid hard-coding the sizes in the program.  Use the [Rust docs ](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html) or this sample code to get an idea on how to get the first argument in the console:

```
use std::env;
 
fn main() {
    let args: Vec<String> = env::args().collect();
 
    // The first argument is the size that was used to call the program. Must use quotes to
    println!("Size is: {}.", args[0]);
    // read this as a single argument
}
```

## Hints:

This exercise is much more harder than the other weeks and you'll be applying more concepts and patterns. Here are some things to consider if you are getting stuck:

- You'll need to split the input string to capture the number and the size. You can use the size (e.g. "kb") to match on how to process that number. 
- The `struct` will need to have the derive debug attribute to print it out
- Use `impl` to extend the struct to do the work on the struct for you

Use the example code to assist you with some of the match statements and transformations needed

## Concepts Covered:

* Introduction to Rust: You will familiarize themselves with the provided source code and its components, including enums and structs.
* Enum usage: You will learn how to define and use an enum (FileSize) to represent different file size units.
* Struct usage: Explore the Sizes struct, its initialization, and how to compute and format the file sizes.
* Error handling: Modify the code to handle invalid input formats, such as missing or unrecognized size identifiers.
* String parsing and formatting: Gain experience in parsing string input to extract the size and identifier, and formatting strings to display the file sizes in a human-readable format.
* Implementing the Debug trait: Learn how to derive and use the Debug trait to print debug information of the Sizes struct.

By completing this lab, you will increase your understanding of Rust by enhancing an existing file size formatter application. You'll apply your knowledge of enums, structs, error handling, string parsing, formatting, and the Debug trait to create a functional and improved file size formatter. The resulting GitHub repository will showcase the implementation, allowing users to interact with the application by providing file size
