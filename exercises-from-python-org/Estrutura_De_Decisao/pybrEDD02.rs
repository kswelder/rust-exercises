use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaDeDecisao
    // Faça um Programa que peça um valor e mostre na tela
    // se o valor é positivo ou negativo.

    println!("Digite um numero");
    let number = scan();
    if number < 0 {
        println!("{} eh negativo", number);
    }
    else {
        println!("{} eh positivo", number);
    }
}

fn scan()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result: i32 = input.trim().parse().unwrap();
    return result;
}