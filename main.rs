mod damas;
mod fns;
mod jogodavelha;
mod quatroemlinha;
mod traits;

use crate::traits::ComportamentoJogo;
use damas::Damas;
use jogodavelha::JogoDaVelha;
use quatroemlinha::QuatroEmLinha;
use traits::Jogador;
use traits::Tabuleiro;

fn main() {
    println!("Bem vindo aos mini jogos! Escolha a opção desejada para iniciarmos:");
    println!("Digite A caso queira jogar jogo da velha ");
    println!("Digite B caso queira jogar 4 em linha");
    println!("Digite qualquer outra tecla caso queira sair do programa");
    let escolha_interface = fns::le_linha();
    let mut tabuleiro = Tabuleiro {
        tamanho: vec![vec!["_".to_string(); 10]; 10],
    };

    if escolha_interface.trim() == "A" {
        let jogo = JogoDaVelha {
            nome: "Jogo da Velha".to_string(),
        };

        println!("Participante 1 escolha X ou O");
        let mut opcao_escolhida = fns::le_linha();

        while opcao_escolhida.trim() != "X" && opcao_escolhida.trim() != "O" {
            println!("Digite novamente");
            opcao_escolhida = fns::le_linha();
        }

        println!("Voce escolheu a opcao {opcao_escolhida}");
        let jogador1 = Jogador::new(opcao_escolhida);

        let opcao_escolhida_2;
        if jogador1.simbolo.trim() == "X" {
            opcao_escolhida_2 = "O";
        } else {
            opcao_escolhida_2 = "X";
        }

        println!("Participante 2 sera a opcao {opcao_escolhida_2}");
        let jogador2 = Jogador::new(opcao_escolhida_2.to_string());

        loop {
            if jogo.verifica_empate(&tabuleiro) {
                break;
            }

            jogo.verifica_linha(&mut tabuleiro, jogador1.simbolo.as_str().trim());
            if jogo.verifica_vitoria(&tabuleiro, jogador1.simbolo.as_str().trim()) {
                break;
            }

            if jogo.verifica_empate(&tabuleiro) {
                break;
            }

            jogo.verifica_linha(&mut tabuleiro, jogador2.simbolo.as_str().trim());
            if jogo.verifica_vitoria(&tabuleiro, jogador2.simbolo.as_str().trim()) {
                break;
            }
        }
    } else if escolha_interface == "B" {
        let jogo = QuatroEmLinha {
            nome: "Quatro em Linha".to_string(),
        };

        println!("Jogo escolhido - Quatro em linha");
        println!("Jogador 1 você deseja ser qual cor? Vermelho ou Azul? Digite V para Vermelho e A para azul!");
        let mut opcao_escolhida = fns::le_linha();

        while opcao_escolhida.trim() != "A" && opcao_escolhida.trim() != "V" {
            println!("Opcao escolhida invalida, por favor digite A ou V:");
            opcao_escolhida = fns::le_linha();
        }

        println!("Voce escolheu a opcao {opcao_escolhida}");
        let jogador1 = Jogador::new(opcao_escolhida);

        let opcao_escolhida_2;

        if jogador1.simbolo.trim() == "V" {
            opcao_escolhida_2 = "A";
        } else {
            opcao_escolhida_2 = "V";
        }

        println!("Participante 2 sera a opcao {opcao_escolhida_2}");
        let jogador2 = Jogador::new(opcao_escolhida_2.to_string());

        loop {
            if jogo.verifica_empate(&tabuleiro) {
                break;
            }

            jogo.verifica_linha(&mut tabuleiro, jogador1.simbolo.as_str().trim());
            if jogo.verifica_vitoria(&tabuleiro, jogador1.simbolo.as_str().trim()) {
                break;
            }

            if jogo.verifica_empate(&tabuleiro) {
                break;
            }

            jogo.verifica_linha(&mut tabuleiro, jogador2.simbolo.as_str().trim());
            if jogo.verifica_vitoria(&tabuleiro, jogador2.simbolo.as_str().trim()) {
                break;
            }
        }
    } else if escolha_interface.trim() == "C" {
        let jogo = Damas {
            nome: "Damas".to_string(),
        };

        println!("Jogo escolhido - Damas");
        println!("Jogador 1 você será 'o' e o jogador 2 será o x");
        let jogador1 = Jogador::new("o".to_string());
        let jogador2 = Jogador::new("x".to_string());
    } else {
        println!("Fechando o programa.");
    }
}
