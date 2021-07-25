use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Tendo como dados de entrada a altura de uma pessoa,
	// construa um algoritmo que calcule seu peso ideal,
	// usando a seguinte fórmula: (72.7*altura) - 58 
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
