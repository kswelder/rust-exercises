use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça a temperatura em graus Celsius,
    // transforme e mostre em graus Fahrenheit.

    println!("Digite a temperatura");
    let celsius = scan_int();
    let fahrenheit = ((celsius + 32)* 9)/ 5;
    println!("{} Celsius sao {} fahrenheit", celsius, fahrenheit);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
