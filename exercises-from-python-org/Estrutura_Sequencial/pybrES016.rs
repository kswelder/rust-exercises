use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um programa para uma loja de tintas.
    // O programa deverá pedir o tamanho em metros quadrados
    // da área a ser pintada.
    // Considere que a cobertura da tinta é de 1 litro para
    // cada 3 metros quadrados e que a tinta é vendida em
    // latas de 18 litros, que custam R$ 80,00.
    // Informe ao usuário a quantidades de latas de tinta a
    // serem compradas e o preço total.

    let metros = 3.0;
    let valor = 80.0;
    println!("Qual o tamanho da area.");
    let area = scan();
    println!("Voce vai gastar {} litros de tinta.\nVoce tera que comprar {} latas de tinta.\nVoce ira gastar R${}",
        (area / metros).ceil(),
        ((area / metros).ceil() / 18.0).ceil(),
        (((area / metros).ceil() / 18.0).ceil()) * valor);
}

fn scan()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
