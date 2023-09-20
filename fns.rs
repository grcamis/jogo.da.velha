use std::io;

pub fn le_linha() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    input.trim().to_string()
}