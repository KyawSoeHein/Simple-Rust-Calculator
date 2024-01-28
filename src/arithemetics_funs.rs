use std::io;

pub fn get_input(msg: String) -> f32 {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().parse().unwrap();
}

pub fn add(value1: &f32, value2: &f32) -> f32 {
    return value1 + value2;
}

pub fn subtract(value1: &f32, value2: &f32) -> f32 {
    return value1 - value2;
}

pub fn multiply(value1: &f32, value2: &f32) -> f32 {
    return value1 * value2;
}

pub fn divide(value1: &f32, value2: &f32) -> f32 {
    return value1 / value2;
}
