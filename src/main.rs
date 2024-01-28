use arithemetics_funs::*;
use std::io;

mod arithemetics_funs;

fn main() {
    let value1: f32 = get_input(String::from("Enter first value: "));
    let value2: f32 = get_input(String::from("Enter second value: "));
    let mut operator: String = String::new();

    println!("Enter operator: ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    match operator.trim() {
        "+" => println!("{}", add(&value1, &value2)),
        "-" => println!("{}", subtract(&value1, &value2)),
        "*" => println!("{}", multiply(&value1, &value2)),
        "/" => println!("{}", divide(&value1, &value2)),
        _ => println!("Invalid operator, please use +, -, *, /"),
    }
}
