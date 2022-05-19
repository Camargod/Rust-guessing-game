use std::io;
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut item = String::new();
    let random : i8 = thread_rng().gen_range(1..11);

    println!("Advinhe o número entre 1 e 10:");

    io::stdin().read_line(&mut item).expect("Erro ao ler linha");

    println!("É o adivinhas:{}",item);
    let number_item : i8 = item.trim().parse().unwrap();
    match random.cmp(&number_item){
        Ordering::Less => println!("Muito baixo"),
        Ordering::Equal => println!("Acertou :)"),
        Ordering::Greater => println!("Muito alto")
    }

    println!("Numero gerado: {}",random);
}
