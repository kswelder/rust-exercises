use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça dois números e imprima a soma
    
    println!("Informe um numero:");

    let num1: i32 = scan_int();

    println!("Informe outro numero:");

    let num2: i32 = scan_int();

    let result = num1 + num2;

    println!("A soma de {} e {} eh {}", num1, num2, result);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
