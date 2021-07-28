use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa para uma loja de tintas.
    // O programa deverá pedir o tamanho em metros quadrados
    // da área a ser pintada.
    // Considere que a cobertura da tinta é de 1 litro para
    // cada 6 metros quadrados e que a tinta é vendida em
    // latas de 18 litros, que custam R$ 80,00 ou em galões
    // de 3,6 litros, que custam R$ 25,00.

    // Informe ao usuário as quantidades de tinta a serem
    // compradas e os respectivos preços em 3 situações:
    
    // comprar apenas latas de 18 litros;
    // comprar apenas galões de 3,6 litros;
    
    // misturar latas e galões, de forma que o desperdício de
    // tinta seja menor.
    // Acrescente 10% de folga e sempre arredonde os valores
    // para cima, isto é, considere latas cheias. 

    let litro_lata = 18.0;
    let valor_lata = 80.0;
    let litro_galao = 3.6;
    let valor_galao = 25.0;
    let metros = 6.0;

    println!("Tamanho da area a ser pintada.");
    let area = scan();

    let litros = (area / 3.0).ceil();
    let lata = (litros / litro_lata).floor();
    let galao = ((litros % litro_lata)/ litro_galao).ceil();    

    println!("
    {} metros a ser pintado
    {} Litros nescessarios
    {} lata(s)
    {} galao(oes)
    Voce pagara R${}
    ", area, area / metros, lata, galao, (lata * valor_lata) + (galao * valor_galao));
}

fn scan()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
