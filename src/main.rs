#[allow(unused_imports)]
#[allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";

//     io::stdin()
//         .read_line(&mut name)
//         .expect("Didn't receive input");
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

fn main() {
    //const usually uses uppercase variable names
    const ONE_MIL: u32 = 1_000_00; //unsigned integer 32 bits
    const PI: f32 = 3.14; // float integer of 32 bits

    //string have double quotes and char have single quotes
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age was not assigned a number");

    //we used same variable name for different data types anf that is called shadowing

    age = age + 1;

    println!("I am {} and I want {}", age, ONE_MIL);
}
