use std::io;

fn anunciar(nome: &str) {
    println!("Parabens, {}! Voce ganhou 10% de desconto!", nome);
}

fn main() {
    println!("Digite seu nome:");
    let mut nome = String::new();
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o nome");

    // let nome = nome.trim().to_string();

    anunciar(nome);
    println!("Participante: {}", nome);
}
