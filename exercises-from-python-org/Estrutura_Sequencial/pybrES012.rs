use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Tendo como dados de entrada a altura de uma pessoa,
	// construa um algoritmo que calcule seu peso ideal,
	// usando a seguinte fórmula: (72.7*altura) - 58 
    
    println!("Digite sua altura");
    let altura: f32 = scan_float();
    println!("Digite seu peso");
    let peso: f32 = scan_float();
    println!("Seu IMC eh {}", peso / (altura * altura));
}

fn scan_float()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
