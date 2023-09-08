use std::io;

fn imprimeTabuleiro(tab: &Vec<Vec<String>>) {
    for i in 0..3 {
        for j in 0..3 {
            print!(" {} ", tab[i][j]);
        }
        println!("");
    }
}

fn geraNumero() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.trim().parse::<i32>() {
            Ok(value) if value >= 0 && value <= 2 => return value,
            _ => println!("Por favor, insira um número entre 0 e 2."),
        }
    }
}

fn verificaCasa(tabuleiro: &mut Vec<Vec<String>>, opcaoEscolhida: &str) {
    let mut verificaJogada = false;
    let mut numeroLinha: i32;
    let mut numeroColuna: i32;

    while verificaJogada == false {
        println!("Jogador {opcaoEscolhida} por favor escolha a linha");
        numeroLinha = geraNumero();
        println!("Jogador {opcaoEscolhida} por favor escolha a coluna");
        numeroColuna = geraNumero();
        if tabuleiro[numeroLinha as usize][numeroColuna as usize] == "_" {
            tabuleiro[numeroLinha as usize][numeroColuna as usize] =
                opcaoEscolhida.trim().to_string();
            imprimeTabuleiro(&tabuleiro);
            verificaJogada = true;
        } else {
            println!("Ponto já utilizado, por favor escolha outros");
        }
    }
}

fn checaJogadas(tabuleiro: &Vec<Vec<String>>, opcaoEscolhida: &str) -> bool {
    for i in 0..3 {
        let mut vitoriaVertical: i32 = 0;
        for j in 0..3 {
            if tabuleiro[i][j] != opcaoEscolhida {
                break;
            } else {
                vitoriaVertical += 1;
            }
        }
        if vitoriaVertical == 3 {
            println!("{opcaoEscolhida} venceu!");
            return true;
        }
    }

    for i in 0..3 {
        let mut vitoriaHorizontal: i32 = 0;
        for j in 0..3 {
            if tabuleiro[j][i] != opcaoEscolhida {
                break;
            } else {
                vitoriaHorizontal += 1;
            }
        }
        if vitoriaHorizontal == 3 {
            println!("{opcaoEscolhida} venceu!");
            return true;
        }
    }

    return false;
}

fn verificaEmpate(tabuleiro: &Vec<Vec<String>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if (tabuleiro[i][j]) == "_" {
                return false;
            }
        }
    }

    println!("Empate!");
    return true;
}

fn le_linha() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    input.trim().to_string()
}

fn main() {
    println!("Participante 1 escolha X ou O");
    let mut opcaoEscolhida = le_linha();

    while opcaoEscolhida.trim() != "X" && opcaoEscolhida.trim() != "O" {
        println!("Digite novamente");
        opcaoEscolhida = le_linha();
    }

    println!("Voce escolheu a opcao {opcaoEscolhida}");

    let opcaoEscolhida2;
    if opcaoEscolhida.trim() == "X" {
        opcaoEscolhida2 = "O";
    } else {
        opcaoEscolhida2 = "X";
    }

    println!("Participante 2 sera a opcao {opcaoEscolhida2}");

    let mut tabuleiro: Vec<Vec<String>> = vec![vec!["_".to_string(); 3]; 3];

    loop {
        if verificaEmpate(&tabuleiro) {
            break;
        }

        verificaCasa(&mut tabuleiro, opcaoEscolhida.as_str().trim());
        if checaJogadas(&tabuleiro, opcaoEscolhida.as_str().trim()) {
            break;
        }

        if verificaEmpate(&tabuleiro) {
            break;
        }

        verificaCasa(&mut tabuleiro, opcaoEscolhida2);
        if checaJogadas(&tabuleiro, opcaoEscolhida2) {
            break;
        }
    }
}
