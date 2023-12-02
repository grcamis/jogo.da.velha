use std::io;

use crate::traits::{ComportamentoJogo, Tabuleiro};

pub struct JogoDaVelha {
    pub nome: String,
}

impl JogoDaVelha {
    pub fn new(nome: String) -> Self {
        Self { nome: nome }
    }
}

impl ComportamentoJogo for JogoDaVelha {
    fn inicializa_tabuleiro(&self, tabuleiro: &mut Tabuleiro) {}

    fn imprime_tabuleiro(&self, tabuleiro: &Tabuleiro) {
        for i in 0..3 {
            for j in 0..3 {
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
                Ok(value) if value >= 0 && value <= 2 => return value,
                _ => println!("Por favor, insira um número entre 0 e 2."),
            }
        }
    }

    fn verifica_linha(&self, tabuleiro: &mut Tabuleiro, opcao_escolhida: &str) {
        let mut verifica_jogada = false;
        let mut numero_linha: i32;
        let mut numero_coluna: i32;

        while verifica_jogada == false {
            println!("Jogador {opcao_escolhida} por favor escolha a linha");
            numero_linha = Self::gera_numero(&self);
            println!("Jogador {opcao_escolhida} por favor escolha a coluna");
            numero_coluna = Self::gera_numero(&self);
            if tabuleiro.tamanho[numero_linha as usize][numero_coluna as usize] == "_" {
                tabuleiro.tamanho[numero_linha as usize][numero_coluna as usize] =
                    opcao_escolhida.trim().to_string();
                Self::imprime_tabuleiro(&self, &tabuleiro);
                verifica_jogada = true;
            } else {
                println!("Ponto já utilizado, por favor escolha outros");
            }
        }
    }

    fn verifica_vitoria(&self, tabuleiro: &Tabuleiro, opcao_escolhida: &str) -> bool {
        for i in 0..3 {
            let mut vitoria_vertical: i32 = 0;
            for j in 0..3 {
                if tabuleiro.tamanho[i][j] != opcao_escolhida {
                    break;
                } else {
                    vitoria_vertical += 1;
                }
            }
            if vitoria_vertical == 3 {
                println!("{opcao_escolhida} venceu!");
                return true;
            }
        }

        for i in 0..3 {
            let mut vitoria_horizontal: i32 = 0;
            for j in 0..3 {
                if tabuleiro.tamanho[j][i] != opcao_escolhida {
                    break;
                } else {
                    vitoria_horizontal += 1;
                }
            }
            if vitoria_horizontal == 3 {
                println!("{opcao_escolhida} venceu!");
                return true;
            }
        }

        return false;
    }

    fn verifica_empate(&self, tabuleiro: &Tabuleiro) -> bool {
        for i in 0..3 {
            for j in 0..3 {
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
