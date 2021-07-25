use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que pergunte quanto você ganha por hora
    // e o número de horas trabalhadas no mês.
    // Calcule e mostre o total do seu salário no referido mês.

    println!("Quanto voce ganha por hora?");
    let dinheiro = scan_int() as f32;
    println!("Quantas horas voce trabalha por mes?");
    let horas = scan_int() as f32;
    let salario = dinheiro * horas;
    println!("Voce ganha R${} por mes.", salario);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
