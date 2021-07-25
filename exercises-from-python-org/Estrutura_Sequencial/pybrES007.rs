use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que calcule a área de um quadrado,
    // em seguida mostre o dobro desta área para o usuário.

    println!("Digite a area x");
    let area_x = scan_int();
    println!("Digite a area y");
    let area_y = scan_int();
    let area_total = area_x * area_y;
    println!("Area total eh {}", area_total);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
