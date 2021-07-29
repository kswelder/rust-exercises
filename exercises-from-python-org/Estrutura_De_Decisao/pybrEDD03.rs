use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaDeDecisao
    // Faça um Programa que verifique se uma letra digitada é
    // "F" ou "M". Conforme a letra escrever:
    // F - Feminino, M - Masculino, Sexo Inválido.

    let mut sex = String::new();
    
    println!("Diga seu sexo, (F ou M)");
    
    io::stdin().read_line(&mut sex).unwrap();
    sex = sex.replace('\n', "");

    if sex == "M" {
        println!("Voce eh homen.");
    }
    else if sex == "F" {
        println!("Voce eh mulher");
    }
    else {
        println!("Sexo invalido");
    }
}