use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaDeDecisao
    // Faça um Programa que peça dois números e
    // imprima o maior deles. 

    println!("Digite um numero");
    let one = scan();
    println!("Digite outro numero");
    let two = scan();

    if one > two {
        println!("{} eh maior que {}", one, two);
    }
    else {
        println!("{} eh maior que {}", two, one);
    }
}

fn scan()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result: i32 = input.trim().parse().unwrap();
    return result;
}