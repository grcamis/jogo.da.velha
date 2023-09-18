use std::io;

mod jogodavelha;

fn imprimeTabuleiro(tab: &Vec<Vec<String>>) {
    for i in 0..6 {
        for j in 0..7 {
            print!(" {} ", tab[i][j]);
        }
        println!("");
    }
}

pub fn geraNumero() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.trim().parse::<i32>() {
            Ok(value) if value >= 0 && value <= 6 => return value,
            _ => println!("Por favor, insira um número entre 0 e 6."),
        }
    }
}

pub fn verificaLinha(tabuleiro: &mut Vec<Vec<String>>, opcaoEscolhida: &str) {
    let mut verificaJogada = false;
    let mut numeroLinha: i32;
    let mut numeroColuna: i32;

    while verificaJogada == false {
        println!("Jogador {opcaoEscolhida} por favor escolha a coluna");
        numeroColuna = geraNumero();
        if tabuleiro[0][numeroColuna as usize] == "_" {
            for i in (0..6).rev() {
                if tabuleiro[i][numeroColuna as usize] == "_" {
                    tabuleiro[i][numeroColuna as usize] = opcaoEscolhida.trim().to_string();
                    break;
                }
            }
            imprimeTabuleiro(&tabuleiro);
            verificaJogada = true;
        } else {
            println!("Linha cheia.");
        }
    }
}

fn verificaEmpate(tabuleiro: &Vec<Vec<String>>) -> bool {
    for i in 0..6 {
        for j in 0..7 {
            if (tabuleiro[i][j]) == "_" {
                return false;
            }
        }
    }

    println!("Empate!");
    return true;
}

fn verificaVitoria(tabuleiro: &Vec<Vec<String>>, opcaoEscolhida: &str) -> bool {
    for i in 0..6 {
        for j in 0..3 {
            let mut vitoriaHorizontal: i32 = 0;
            for k in (j..j + 4).rev() {
                if tabuleiro[k][i] != opcaoEscolhida {
                    break;
                } else {
                    vitoriaHorizontal += 1;
                }
            }
            if vitoriaHorizontal == 4 {
                println!("{opcaoEscolhida} venceu!");
                return true;
            }
        }
    }

    for i in 0..6 {
        for j in 0..4 {
            let mut vitoriaVertical: i32 = 0;
            for k in j..j + 4 {
                if tabuleiro[i][k] != opcaoEscolhida {
                    break;
                } else {
                    vitoriaVertical += 1;
                }
            }
            if vitoriaVertical == 4 {
                println!("{opcaoEscolhida} venceu!");
                return true;
            }
        }
    }

    for i in 0..6 {
        for j in 0..4 {
            let mut vitoriaVertical: i32 = 0;
            for k in j..j + 4 {
                if tabuleiro[i][k] != opcaoEscolhida {
                    break;
                } else {
                    vitoriaVertical += 1;
                }
            }
            if vitoriaVertical == 4 {
                println!("{opcaoEscolhida} venceu!");
                return true;
            }
        }
    }

    for i in 3..6 {
        for j in 0..4 {
            let mut contador = 0;
            let mut vitoriaDiagonal = 0;
            for k in j..j + 4 {
                if tabuleiro[i - contador][k] != opcaoEscolhida {
                    break;
                } else {
                    vitoriaDiagonal += 1;
                    contador += 1;
                }
            }
            if vitoriaDiagonal == 4 {
                println!("{opcaoEscolhida} venceu!");
                return true;
            }
        }
    }

    for i in 3..6 {
        for j in 0..4 {
            let mut contador = 0;
            let mut vitoriaDiagonal = 0;
            for k in (j..j + 4).rev() {
                if tabuleiro[i - contador][k] != opcaoEscolhida {
                    break;
                } else {
                    vitoriaDiagonal += 1;
                    contador += 1;
                }
            }
            if vitoriaDiagonal == 4 {
                println!("{opcaoEscolhida} venceu!");
                return true;
            }
        }
    }

    return false;
}

fn main() {
    println!("Bem vindo aos mini jogos! Escolha a opção desejada para iniciarmos:");
    println!("Digite A caso queira jogar jogo da velha ");
    println!("Digite B caso queira jogar 4 em linha");
    println!("Digite qualquer outra tecla caso queira sair do programa");
    let mut escolhaInterface = jogodavelha::le_linha();

    if escolhaInterface.trim() == "A" {
        println!("Participante 1 escolha X ou O");
        let mut opcaoEscolhida = jogodavelha::le_linha();

        while opcaoEscolhida.trim() != "X" && opcaoEscolhida.trim() != "O" {
            println!("Digite novamente");
            opcaoEscolhida = jogodavelha::le_linha();
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
            if jogodavelha::verificaEmpate(&tabuleiro) {
                break;
            }

            jogodavelha::verificaCasa(&mut tabuleiro, opcaoEscolhida.as_str().trim());
            if jogodavelha::checaJogadas(&tabuleiro, opcaoEscolhida.as_str().trim()) {
                break;
            }

            if jogodavelha::verificaEmpate(&tabuleiro) {
                break;
            }

            jogodavelha::verificaCasa(&mut tabuleiro, opcaoEscolhida2);
            if jogodavelha::checaJogadas(&tabuleiro, opcaoEscolhida2) {
                break;
            }
        }
    } else if escolhaInterface == "B" {
        println!("Jogo escolhido - Quatro em linha");
        println!("Jogador 1 você deseja ser qual cor? Vermelho ou Azul? Digite V para Vermelho e A para azul!");
        let mut opcaoEscolhida = jogodavelha::le_linha();

        while opcaoEscolhida.trim() != "A" && opcaoEscolhida.trim() != "V" {
            println!("Opcao escolhida invalida, por favor digite A ou V:");
            opcaoEscolhida = jogodavelha::le_linha();
        }

        let opcaoEscolhida2;

        if opcaoEscolhida.trim() == "V" {
            opcaoEscolhida2 = "A";
        } else {
            opcaoEscolhida2 = "V";
        }

        println!("Jogador 1 será o {opcaoEscolhida} e jogador 2 será o {opcaoEscolhida2}");

        let mut tabuleiro: Vec<Vec<String>> = vec![vec!["_".to_string(); 7]; 6];

        loop {
            if verificaEmpate(&tabuleiro) {
                break;
            }

            verificaLinha(&mut tabuleiro, opcaoEscolhida.as_str().trim());
            if verificaVitoria(&tabuleiro, opcaoEscolhida.as_str().trim()) {
                break;
            }

            if verificaEmpate(&tabuleiro) {
                break;
            }

            verificaLinha(&mut tabuleiro, opcaoEscolhida2);
            if verificaVitoria(&tabuleiro, opcaoEscolhida2) {
                break;
            }
        }
    } else {
        println!("Fechando o programa.");
    }
}
