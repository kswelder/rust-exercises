use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaDeDecisao
    // Faça um Programa que verifique se uma letra digitada
    // é vogal ou consoante.

    let mut letra = String::new();

    println!("Digite uma letra");
    
    io::stdin().read_line(&mut letra).unwrap();
 
    letra = letra.replace('\n', "");

    if letra == "a" || letra == "A" || letra == "e" || letra == "E" || letra == "i" || letra == "I" || letra == "o" || letra == "O" || letra == "u" || letra == "U" {
        println!("{} eh uma vogal", letra);
    }
    else {
        println!("{} eh uma consoante", letra);
    }
}