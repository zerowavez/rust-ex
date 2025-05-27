use std::io;

fn main() {
    let mut input1: String = String::new();
    let mut input2: String = String::new();

    println!("escreva um número: ");

    io::stdin()
        .read_line(&mut input1)
        .expect("erro ao ler o número");

    println!("escreva outro número: ");

    io::stdin()
        .read_line(&mut input2)
        .expect("erro ao ler o número");

    let num1: i32 = input1.trim().parse().expect("erro");
    let num2: i32 = input2.trim().parse().expect("erro");

    let soma = num1 + num2;

    println!("o resultado de {} + {} é {}", num1, num2, soma)

}