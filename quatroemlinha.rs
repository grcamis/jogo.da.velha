use std::io;

use crate::traits::{ComportamentoJogo, Tabuleiro};

pub struct QuatroEmLinha {
    pub nome: String,
}

impl QuatroEmLinha {
    pub fn new(nome: String) -> Self {
        Self { nome: nome }
    }
}

impl ComportamentoJogo for QuatroEmLinha {
    fn inicializa_tabuleiro(&self, tabuleiro: &mut Tabuleiro) {}

    fn imprime_tabuleiro(&self, tabuleiro: &Tabuleiro) {
        for i in 0..6 {
            for j in 0..7 {
                print!(" {} ", tabuleiro.tamanho[i][j]);
            }
            println!("");
        }
    }

    fn gera_numero(&self) -> i32 {
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

    fn verifica_linha(&self, tabuleiro: &mut Tabuleiro, opcao_escolhida: &str) {
        let mut verifica_jogada = false;
        let mut numero_coluna: i32;

        while verifica_jogada == false {
            println!("Jogador {opcao_escolhida} por favor escolha a coluna");
            numero_coluna = Self::gera_numero(&self);
            if tabuleiro.tamanho[0][numero_coluna as usize] == "_" {
                for i in (0..6).rev() {
                    if tabuleiro.tamanho[i][numero_coluna as usize] == "_" {
                        tabuleiro.tamanho[i][numero_coluna as usize] =
                            opcao_escolhida.trim().to_string();
                        break;
                    }
                }
                Self::imprime_tabuleiro(&self, &tabuleiro);
                verifica_jogada = true;
            } else {
                println!("Linha cheia.");
            }
        }
    }

    fn verifica_vitoria(&self, tabuleiro: &Tabuleiro, opcao_escolhida: &str) -> bool {
        for i in 0..6 {
            for j in 0..3 {
                let mut vitoria_horizontal: i32 = 0;
                for k in (j..j + 4).rev() {
                    if tabuleiro.tamanho[k][i] != opcao_escolhida {
                        break;
                    } else {
                        vitoria_horizontal += 1;
                    }
                }
                if vitoria_horizontal == 4 {
                    println!("{opcao_escolhida} venceu!");
                    return true;
                }
            }
        }

        for i in 0..6 {
            for j in 0..4 {
                let mut vitoria_vertical: i32 = 0;
                for k in j..j + 4 {
                    if tabuleiro.tamanho[i][k] != opcao_escolhida {
                        break;
                    } else {
                        vitoria_vertical += 1;
                    }
                }
                if vitoria_vertical == 4 {
                    println!("{opcao_escolhida} venceu!");
                    return true;
                }
            }
        }

        for i in 0..6 {
            for j in 0..4 {
                let mut vitoria_vertical: i32 = 0;
                for k in j..j + 4 {
                    if tabuleiro.tamanho[i][k] != opcao_escolhida {
                        break;
                    } else {
                        vitoria_vertical += 1;
                    }
                }
                if vitoria_vertical == 4 {
                    println!("{opcao_escolhida} venceu!");
                    return true;
                }
            }
        }

        for i in 3..6 {
            for j in 0..4 {
                let mut contador = 0;
                let mut vitoria_diagonal = 0;
                for k in j..j + 4 {
                    if tabuleiro.tamanho[i - contador][k] != opcao_escolhida {
                        break;
                    } else {
                        vitoria_diagonal += 1;
                        contador += 1;
                    }
                }
                if vitoria_diagonal == 4 {
                    println!("{opcao_escolhida} venceu!");
                    return true;
                }
            }
        }

        for i in 3..6 {
            for j in 0..4 {
                let mut contador = 0;
                let mut vitoria_diagonal = 0;
                for k in (j..j + 4).rev() {
                    if tabuleiro.tamanho[i - contador][k] != opcao_escolhida {
                        break;
                    } else {
                        vitoria_diagonal += 1;
                        contador += 1;
                    }
                }
                if vitoria_diagonal == 4 {
                    println!("{opcao_escolhida} venceu!");
                    return true;
                }
            }
        }

        return false;
    }

    fn verifica_empate(&self, tabuleiro: &Tabuleiro) -> bool {
        for i in 0..6 {
            for j in 0..7 {
                if (tabuleiro.tamanho[i][j]) == "_" {
                    return false;
                }
            }
        }

        println!("Empate!");
        return true;
    }

    fn inicializa_jogo(
        &self,
        tabuleiro: &mut Tabuleiro,
        jogador1_simbolo: &str,
        jogador2_simbolo: &str,
    ) {
        loop {
            if self.verifica_empate(&tabuleiro) {
                break;
            }

            self.verifica_linha(tabuleiro, jogador1_simbolo.trim());
            if self.verifica_vitoria(&tabuleiro, jogador1_simbolo.trim()) {
                break;
            }

            if self.verifica_empate(&tabuleiro) {
                break;
            }

            self.verifica_linha(tabuleiro, jogador2_simbolo.trim());
            if self.verifica_vitoria(&tabuleiro, jogador2_simbolo.trim()) {
                break;
            }
        }
    }
}
