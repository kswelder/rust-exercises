use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaDeDecisao
    // Faça um programa para a leitura de duas notas parciais de um aluno.
    // O programa deve calcular a média alcançada por aluno e apresentar:

    // A mensagem "Aprovado", se a média alcançada for maior ou igual a sete;
    // A mensagem "Reprovado", se a média for menor do que sete;
    // A mensagem "Aprovado com Distinção", se a média for igual a dez.

    println!("Digite sua nota");
    let nota1 = scan();
    println!("Digite sua outra nota");
    let nota2 = scan();
    let media = (nota1 + nota2)/ 2 as f32;

    if media == 10 as f32 {
        println!("Aprovado com Distincao");
    }
    else if media >= 7 as f32 {
        println!("Aprovado");
    }
    else if media < 7 as f32 {
        println!("Reprovado");
    }
}

fn scan()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result: f32 = input.trim().parse().unwrap();
    return result;
}