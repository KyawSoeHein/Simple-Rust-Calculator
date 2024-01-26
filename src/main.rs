use std::io;

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

fn get_input(msg: String) -> f32 {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().parse().unwrap();
}

fn add(value1: &f32, value2: &f32) -> f32 {
    return value1 + value2;
}

fn subtract(value1: &f32, value2: &f32) -> f32 {
    return value1 - value2;
}

fn multiply(value1: &f32, value2: &f32) -> f32 {
    return value1 * value2;
}

fn divide(value1: &f32, value2: &f32) -> f32 {
    return value1 / value2;
}
