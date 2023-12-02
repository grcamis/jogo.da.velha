use std::io;

use crate::traits::{ComportamentoJogo, Tabuleiro};

pub struct Damas {
    pub nome: String,
}

impl Damas {
    pub fn new(nome: String) -> Self {
        Self { nome: nome }
    }
}

impl ComportamentoJogo for Damas {
    fn inicializa_tabuleiro(&self, tabuleiro: &mut Tabuleiro) {
        for i in 0..4 {
            for j in 0..10 {
                if j % 2 == 0 && i % 2 == 0 {
                    tabuleiro.tamanho[i][j] = "o".to_string();
                } else if j % 2 != 0 && i % 2 != 0 {
                    tabuleiro.tamanho[i][j] = "o".to_string();
                }
            }
        }

        for i in 6..10 {
            for j in 0..10 {
                if j % 2 == 0 && i % 2 == 0 {
                    tabuleiro.tamanho[i][j] = "x".to_string();
                } else if j % 2 != 0 && i % 2 != 0 {
                    tabuleiro.tamanho[i][j] = "x".to_string();
                }
            }
        }
    }

    fn imprime_tabuleiro(&self, tabuleiro: &Tabuleiro) {
        for i in 0..10 {
            for j in 0..10 {
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
                Ok(value) if value >= 0 && value <= 9 => return value,
                _ => println!("Por favor, insira um número entre 0 e 9."),
            }
        }
    }

    fn verifica_linha(&self, tabuleiro: &mut Tabuleiro, opcao_escolhida: &str) {
        let mut realizou_movimento = false;

        while realizou_movimento == false {
            println!("Digite a linha da peça que você deseja movimentar:");
            let linha_escolhida = self.gera_numero();
            println!("Digite a coluna da peça que você deseja movimentar:");
            let coluna_escolhida = self.gera_numero();

            let lado_escolhido;
            if opcao_escolhida == "o" || opcao_escolhida == "x" {
                println!("Você deseja se movimentar para a direita ou para a esquerda? 1 para a esquerda e 0 para a direita");
                lado_escolhido = self.gera_numero();
            } else {
                println!("Você deseja se movimentar para a direita ou para a esquerda? 0 para frente direita 1 para frente esquerda 2 para trás direita 3 para trás esquerda");
                lado_escolhido = self.gera_numero();
            }

            if tabuleiro.tamanho[linha_escolhida as usize][coluna_escolhida as usize]
                == opcao_escolhida
            {
                if opcao_escolhida == "o" {
                    if lado_escolhido == 0 {
                        let mut nova_linha = linha_escolhida + 1;
                        let mut nova_coluna = coluna_escolhida + 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    "o".to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida + 2;
                            nova_coluna = coluna_escolhida + 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize - 1]
                                        [nova_coluna as usize - 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        "o".to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }

                        if nova_linha == 9 {
                            tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                "O".to_string();
                        }
                    } else if lado_escolhido == 1 {
                        let mut nova_linha = linha_escolhida + 1;
                        let mut nova_coluna = coluna_escolhida - 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    "o".to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida + 2;
                            nova_coluna = coluna_escolhida - 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize - 1]
                                        [nova_coluna as usize + 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        "o".to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }

                        if nova_linha == 9 {
                            tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                "O".to_string();
                        }
                    } else {
                        println!("Movimento inválido");
                    }
                } else if opcao_escolhida == "x" {
                    if lado_escolhido == 0 {
                        let mut nova_linha = linha_escolhida - 1;
                        let mut nova_coluna = coluna_escolhida + 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    "x".to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida - 2;
                            nova_coluna = coluna_escolhida + 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize + 1]
                                        [nova_coluna as usize - 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        "x".to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }

                        if nova_linha == 0 {
                            tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                "X".to_string();
                        }
                    } else if lado_escolhido == 1 {
                        let mut nova_linha = linha_escolhida - 1;
                        let mut nova_coluna = coluna_escolhida - 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    "x".to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida - 2;
                            nova_coluna = coluna_escolhida - 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize + 1]
                                        [nova_coluna as usize + 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        "x".to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }

                        if nova_linha == 0 {
                            tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                "X".to_string();
                        }
                    } else {
                        println!("Movimento inválido");
                    }
                } else {
                    if lado_escolhido == 0 {
                        let mut nova_linha = linha_escolhida + 1;
                        let mut nova_coluna = coluna_escolhida + 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    opcao_escolhida.to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida + 2;
                            nova_coluna = coluna_escolhida + 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize - 1]
                                        [nova_coluna as usize - 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        opcao_escolhida.to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }
                    } else if lado_escolhido == 1 {
                        let mut nova_linha = linha_escolhida + 1;
                        let mut nova_coluna = coluna_escolhida - 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    opcao_escolhida.to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida + 2;
                            nova_coluna = coluna_escolhida - 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize - 1]
                                        [nova_coluna as usize + 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        opcao_escolhida.to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }
                    } else if lado_escolhido == 2 {
                        let mut nova_linha = linha_escolhida - 1;
                        let mut nova_coluna = coluna_escolhida + 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    opcao_escolhida.to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida - 2;
                            nova_coluna = coluna_escolhida + 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize + 1]
                                        [nova_coluna as usize - 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        opcao_escolhida.to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }
                    } else if lado_escolhido == 3 {
                        let mut nova_linha = linha_escolhida - 1;
                        let mut nova_coluna = coluna_escolhida - 1;

                        if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] == "_" {
                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                tabuleiro.tamanho[linha_escolhida as usize]
                                    [coluna_escolhida as usize] = "_".to_string();
                                tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                    opcao_escolhida.to_string();
                                realizou_movimento = true;
                            }
                        } else if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                            != opcao_escolhida
                        {
                            nova_linha = linha_escolhida - 2;
                            nova_coluna = coluna_escolhida - 2;

                            if nova_linha > 9
                                || nova_coluna > 9
                                || nova_linha < 0
                                || nova_coluna < 0
                            {
                                println!("Você está ultrapassando com o movimento selecionado, por favor escolha outro");
                            } else {
                                if tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize]
                                    == "_"
                                {
                                    tabuleiro.tamanho[linha_escolhida as usize]
                                        [coluna_escolhida as usize] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize + 1]
                                        [nova_coluna as usize + 1] = "_".to_string();
                                    tabuleiro.tamanho[nova_linha as usize][nova_coluna as usize] =
                                        opcao_escolhida.to_string();
                                    realizou_movimento = true;
                                } else {
                                    println!("Movimento inválido");
                                }
                            }
                        }
                    } else {
                        println!("Movimento inválido");
                    }
                }
            } else {
                println!("Movimento inválido");
            }
        }
    }

    fn verifica_vitoria(&self, tabuleiro: &Tabuleiro, opcao_escolhida: &str) -> bool {
        for i in 0..10 {
            for j in 0..10 {
                if tabuleiro.tamanho[i][j] != opcao_escolhida && tabuleiro.tamanho[i][j] != "_" {
                    return false;
                }
            }
        }

        return true;
    }

    fn verifica_empate(&self, tabuleiro: &Tabuleiro) -> bool {
        return false;
    }

    fn inicializa_jogo(
        &self,
        tabuleiro: &mut Tabuleiro,
        jogador1_simbolo: &str,
        jogador2_simbolo: &str,
    ) {
        self.inicializa_tabuleiro(tabuleiro);
        self.imprime_tabuleiro(tabuleiro);

        loop {
            self.verifica_linha(tabuleiro, jogador1_simbolo.trim());
            self.imprime_tabuleiro(tabuleiro);
            if self.verifica_vitoria(tabuleiro, jogador1_simbolo.trim()) {
                break;
            }

            self.verifica_linha(tabuleiro, jogador2_simbolo.trim());
            self.imprime_tabuleiro(tabuleiro);
            if self.verifica_vitoria(tabuleiro, jogador2_simbolo.trim()) {
                break;
            }
        }
    }
}
