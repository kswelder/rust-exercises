struct Ficha {nome: &'static str, idade: &'static i32}

trait Cliente {
    fn new(nome: &'static str, idade: &'static i32) -> Self;

    fn nome(&self) -> &'static str;

    fn idade(&self) -> &'static i32;

    fn talk(&self) {
        println!("Meu nome é {} e eu tenho {} anos de idade.", self.nome(), self.idade());
    }
}
/*
impl Ficha {
    fn acond(&self) {
        println!("Meu nome é {}!", self.nome());
    }
}*/

impl Cliente for Ficha {
    fn new(nome: &'static str, idade: &'static i32) -> Ficha {
        Ficha { nome: nome, idade: idade}
    }

    fn nome(&self) -> &'static str {
        self.nome
    }

    fn idade(&self) -> &'static i32 {
        self.idade
    }

    fn talk(&self) {
        println!("Meu nome é {} e eu tenho {} anos de idade.", self.nome, self.idade);
    }
}

fn main() {
    let novo: Ficha = Cliente::new("Welder", &24);

    novo.talk();
    //novo.acond();

    println!("Tudo certo !");
}
