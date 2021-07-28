use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que pergunte quanto você ganha por
    // hora e o número de horas trabalhadas no mês.
    // Calcule e mostre o total do seu salário no referido mês,
    // sabendo-se que são descontados 11% para o Imposto de
    // Renda, 8% para o INSS e 5% para o sindicato,
    // faça um programa que nos dê:

    // salário bruto.
    // quanto pagou ao INSS.
    // quanto pagou ao sindicato.
    // o salário líquido.
    // calcule os descontos e o salário líquido, conforme a tabela abaixo:

    // + Salário Bruto : R$
    // - IR (11%) : R$
    // - INSS (8%) : R$
    // - Sindicato ( 5%) : R$
    // = Salário Liquido : R$

    // Obs.: Salário Bruto - Descontos = Salário Líquido.

    println!("Digite quanto voce ganha por hora.");
    let ganho = scan();
    println!("Digite quantas horas voce trabalha por mes.");
    let horas = scan();
    let mes = ganho * horas;
    println!("Salario Bruto: R${}\nIR: R${}\nINSS: R${}\nSindicato: R${}\nSalario Liquido: R${}", mes, mes * 0.11, mes * 0.08, mes * 0.05, mes - (mes * 0.24));
}

fn scan()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
