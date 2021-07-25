use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça um número e então mostre a mensagem
    // O número informado foi [número]
    

    let mut input = String::new();
    //io::stdin().read_line(&mut input).unwrap();
    //let n: i32 = input.trim().parse().unwrap();

    println!("Informe um numero:");

    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();

    println!("O numero informado foi [ {} ]", number);
}
