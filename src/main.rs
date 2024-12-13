use std::io;

fn main() {
    println!("Irei descobrir seu nome!!\n\n");
    println!("Digite seu nome:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("falha a ler a entrada");

    let name = name.trim();

    println!("Seu nome é: {name}");
}   
