use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Tendo como dado de entrada a altura (h) de uma pessoa,
    // construa um algoritmo que calcule seu peso ideal,
    // utilizando as seguintes fórmulas:

    // Para homens: (72.7*h) - 58
    // Para mulheres: (62.1*h) - 44.7

    println!("Digite sua altura");
    let h = scan();
    println!("Homen: {}\nMulher: {}", (72.7 * h) - 58.0, (62.1 * h) - 44.7);
}

fn scan()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
