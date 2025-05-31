use core::panic;
use std::io;

fn readint() -> i32 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input int");

    let num: i32 = input
        .trim()
        .parse::<i32>()
        .expect("failed to convert into i32");

    num
}

fn readchar() -> char {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input char");

    let char: char = input
        .trim()
        .chars()
        .next()
        .expect("failed to convert into char");

    char
}

fn main() {
    println!("type in a number: ");
    let num1: i32 = readint();

    println!("choose an operator (+, -, *, /): ");
    let op: char = readchar();

    println!("type in another number: ");
    let num2: i32 = readint();

    let result: i32 = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!(),
    };

    println!("{} {} {} results in {}", num1, op, num2, result);
}
