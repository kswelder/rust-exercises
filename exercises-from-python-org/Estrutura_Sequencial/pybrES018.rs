use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um programa que peça o tamanho de um arquivo
    // para download (em MB) e a velocidade de um link de
    // Internet (em Mbps), calcule e informe o tempo aproximado
    // de download do arquivo usando este link (em minutos).

    let velocidade = 100.0; // MBPS
    println!("Digite quantos MB eh o pacote.");
    let pacote = scan();

    let retorno = ((velocidade / 8.0)* pacote).ceil();
    let horas = (retorno / 60 as f32)/ 60 as f32;
    println!("Seu download estara pronto em {}:{}:{}", horas.floor(), (horas % 60 as f32).floor(), ((horas % 60 as f32)/ 60 as f32).ceil());
}

fn scan()-> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().unwrap();
    return number;
}
