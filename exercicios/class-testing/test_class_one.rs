struct Ficha {nome: &'static str}

trait Cliente {
    fn new(nome: &'static str) -> Self;

    fn nome(&self) -> &'static str;

    fn talk(&self) {
        println!("Meu nome é {}", self.nome());
    }
}
/*
impl Ficha {
    fn acond(&self) {
        println!("Meu nome é {}!", self.nome());
    }
}*/

impl Cliente for Ficha {
    fn new(nome: &'static str) -> Ficha {
        Ficha { nome: nome}
    }

    fn nome(&self) -> &'static str {
        self.nome
    }

    fn talk(&self) {
        println!("Meu nome é {}", self.nome);
    }
}

fn main() {
    let novo: Ficha = Cliente::new("Welder");

    novo.talk();
    //novo.acond();

    println!("Tudo certo !");
}
