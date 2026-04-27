use std::io;

fn anunciar(nome: &str) {
    println!("Parabens, {}! Voce ganhou R$ 40,00 de desconto na compra do livro!", nome);
}

fn main() {
    println!("Digite seu nome:");
    let mut nome = String::new();
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o nome");

    let nome = nome.trim().to_string();

    anunciar(nome);
    println!("Participante: {}", nome);
}
