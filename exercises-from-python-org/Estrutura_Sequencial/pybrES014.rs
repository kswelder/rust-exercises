use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // João Papo-de-Pescador, homem de bem,
    // comprou um microcomputador para controlar o
    // rendimento diário de seu trabalho.
    // Toda vez que ele traz um peso de peixes maior
    // que o estabelecido pelo regulamento de pesca
    // do estado de São Paulo (50 quilos) deve pagar
    // uma multa de R$ 4,00 por quilo excedente.
    // João precisa que você faça um programa que leia
    // a variável peso (peso de peixes) e calcule o excesso.
    // Gravar na variável excesso a quantidade de quilos além
    // do limite e na variável multa o valor da multa que
    // João deverá pagar.
    // Imprima os dados do programa com as mensagens adequadas.

    let exedente: f32 = 50.0;
    let multa: f32 = 4.0;

    println!("Digite quantos KG de peixes voce pegou.");
    let peixes = scan_int();
    if exedente < peixes {
        println!("Voce pegou {}KG peixes a mais do limite.\nSua multa eh de R${}", peixes - exedente, (peixes - exedente) * multa);
    }
    else {
        println!("Voce pegou {}KG peixes, entao nao precisa pagar multa.", peixes);
    }
}

fn scan_int()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
