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
        fn movimenta(
            mut coluna_peca: i32,
            mut linha_peca: i32,
            lado_movimento: i32,
            opcao_escolhida: &str,
            tabuleiro: &mut Tabuleiro,
        ) -> bool {
            let mut comeu_peca = false;
            fn come_peca(
                coluna_peca: &mut i32,
                linha_peca: &mut i32,
                lado_movimento: i32,
                opcao_escolhida: &str,
                comeu_peca: &mut bool,
                tabuleiro: &mut Tabuleiro,
            ) -> bool {
                if opcao_escolhida.trim() == "o" {
                    if lado_movimento == 0 {
                        if *coluna_peca + 2 > 9 || *linha_peca + 2 > 9 {
                            if *comeu_peca == false {
                                println!("Movimento inválido, você está ultrapassando o tabuleiro pela direita.");
                            }
                            *comeu_peca = false;
                        } else {
                            if tabuleiro.tamanho[*linha_peca as usize + 2]
                                [*coluna_peca as usize + 2]
                                == "_"
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 1]
                                    [*coluna_peca as usize + 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 2]
                                    [*coluna_peca as usize + 2] = "o".to_string();
                                *linha_peca += 2;
                                *coluna_peca += 2;
                                *comeu_peca = true;
                            } else if tabuleiro.tamanho[*linha_peca as usize + 2]
                                [*coluna_peca as usize - 2]
                                == "_"
                                && *comeu_peca == true
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 1]
                                    [*coluna_peca as usize - 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 2]
                                    [*coluna_peca as usize - 2] = "o".to_string();
                                *linha_peca += 2;
                                *coluna_peca -= 2;
                            }
                        }
                    } else if lado_movimento == 1 {
                        if *coluna_peca - 2 < 0 || *linha_peca + 2 < 9 {
                            if *comeu_peca == false {
                                println!("Movimento inválido, você está ultrapassando o tabuleiro pela esquerda.");
                            }
                            *comeu_peca = false;
                        } else {
                            if tabuleiro.tamanho[*linha_peca as usize + 2]
                                [*coluna_peca as usize - 2]
                                == "_"
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 1]
                                    [*coluna_peca as usize - 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 2]
                                    [*coluna_peca as usize - 2] = "o".to_string();
                                *linha_peca += 2;
                                *coluna_peca -= 2;
                            } else if tabuleiro.tamanho[*linha_peca as usize + 2]
                                [*coluna_peca as usize + 2]
                                == "_"
                                && *comeu_peca == true
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 1]
                                    [*coluna_peca as usize + 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize + 2]
                                    [*coluna_peca as usize + 2] = "o".to_string();
                                *linha_peca += 2;
                                *coluna_peca += 2;
                            }
                        }
                    }
                } else if opcao_escolhida.trim() == "x" {
                    if lado_movimento == 0 {
                        if *coluna_peca + 2 > 9 || *linha_peca - 2 < 0 {
                            if *comeu_peca == false {
                                println!("Movimento inválido, você está ultrapassando o tabuleiro pela direita.");
                            }
                            *comeu_peca = false;
                        } else {
                            if tabuleiro.tamanho[*linha_peca as usize - 2]
                                [*coluna_peca as usize + 2]
                                == "_"
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 1]
                                    [*coluna_peca as usize + 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 2]
                                    [*coluna_peca as usize + 2] = "o".to_string();
                                *linha_peca -= 2;
                                *coluna_peca += 2;
                                *comeu_peca = true;
                            } else if tabuleiro.tamanho[*linha_peca as usize - 2]
                                [*coluna_peca as usize - 2]
                                == "_"
                                && *comeu_peca == true
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 1]
                                    [*coluna_peca as usize - 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 2]
                                    [*coluna_peca as usize - 2] = "o".to_string();
                                *linha_peca -= 2;
                                *coluna_peca -= 2;
                            }
                        }
                    } else if lado_movimento == 1 {
                        if *coluna_peca < 0 || *linha_peca - 2 < 0 {
                            if *comeu_peca == false {
                                println!("Movimento inválido, você está ultrapassando o tabuleiro pela esquerda.");
                            }
                            *comeu_peca = false;
                        } else {
                            if tabuleiro.tamanho[*linha_peca as usize - 2]
                                [*coluna_peca as usize - 2]
                                == "_"
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 1]
                                    [*coluna_peca as usize - 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 2]
                                    [*coluna_peca as usize - 2] = "o".to_string();
                                *linha_peca -= 2;
                                *coluna_peca -= 2;
                                *comeu_peca = true;
                            } else if tabuleiro.tamanho[*linha_peca as usize - 2]
                                [*coluna_peca as usize + 2]
                                == "_"
                                && *comeu_peca == true
                            {
                                tabuleiro.tamanho[*linha_peca as usize][*coluna_peca as usize] =
                                    "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 1]
                                    [*coluna_peca as usize + 1] = "_".to_string();
                                tabuleiro.tamanho[*linha_peca as usize - 2]
                                    [*coluna_peca as usize + 2] = "o".to_string();
                                *linha_peca -= 2;
                                *coluna_peca += 2;
                            }
                        }
                    }
                }

                return *comeu_peca;
            }

            let mut realizou_movimento = false;
            if opcao_escolhida.trim() == "o" {
                if lado_movimento == 0 {
                    if coluna_peca + 1 > 9 {
                        println!(
                            "Movimento inválido, você está ultrapassando o tabuleiro pela direita."
                        );
                    } else {
                        if tabuleiro.tamanho[linha_peca as usize + 1][coluna_peca as usize + 1]
                            == "_"
                        {
                            tabuleiro.tamanho[linha_peca as usize][coluna_peca as usize] =
                                "_".to_string();
                            tabuleiro.tamanho[linha_peca as usize + 1][coluna_peca as usize + 1] =
                                "o".to_string();
                            realizou_movimento = true;
                        } else if tabuleiro.tamanho[linha_peca as usize + 1]
                            [coluna_peca as usize + 1]
                            == "x"
                        {
                            while come_peca(
                                &mut coluna_peca,
                                &mut linha_peca,
                                lado_movimento,
                                opcao_escolhida,
                                &mut comeu_peca,
                                tabuleiro,
                            ) {
                                realizou_movimento = true;
                            }
                        }
                    }
                } else if lado_movimento == 1 {
                    if coluna_peca - 1 < 0 {
                        println!("Movimento inválido, você está ultrapassando o tabuleiro pela esquerda.");
                    } else {
                        if tabuleiro.tamanho[linha_peca as usize + 1][coluna_peca as usize - 1]
                            == "_"
                        {
                            tabuleiro.tamanho[linha_peca as usize][coluna_peca as usize] =
                                "_".to_string();
                            tabuleiro.tamanho[linha_peca as usize + 1][coluna_peca as usize - 1] =
                                "o".to_string();
                            realizou_movimento = true;
                        } else if tabuleiro.tamanho[linha_peca as usize + 1]
                            [coluna_peca as usize - 1]
                            == "x"
                        {
                            while come_peca(
                                &mut coluna_peca,
                                &mut linha_peca,
                                lado_movimento,
                                opcao_escolhida,
                                &mut comeu_peca,
                                tabuleiro,
                            ) {
                                realizou_movimento = true;
                            }
                        }
                    }
                }
            } else if opcao_escolhida.trim() == "x" {
                if lado_movimento == 0 {
                    if coluna_peca + 1 > 9 {
                        println!(
                            "Movimento inválido, você está ultrapassando o tabuleiro pela direita."
                        );
                    } else {
                        if tabuleiro.tamanho[linha_peca as usize - 1][coluna_peca as usize + 1]
                            == "_"
                        {
                            tabuleiro.tamanho[linha_peca as usize][coluna_peca as usize] =
                                "_".to_string();
                            tabuleiro.tamanho[linha_peca as usize - 1][coluna_peca as usize + 1] =
                                "x".to_string();
                            realizou_movimento = true;
                        } else if tabuleiro.tamanho[linha_peca as usize + 1]
                            [coluna_peca as usize + 1]
                            == "o"
                        {
                            if come_peca(
                                &mut coluna_peca,
                                &mut linha_peca,
                                lado_movimento,
                                opcao_escolhida,
                                &mut comeu_peca,
                                tabuleiro,
                            ) == true
                            {
                                realizou_movimento = true;
                            }
                        }
                    }
                } else if lado_movimento == 1 {
                    if coluna_peca - 1 < 0 {
                        println!("Movimento inválido, você está ultrapassando o tabuleiro pela esquerda.");
                    } else {
                        if tabuleiro.tamanho[linha_peca as usize - 1][coluna_peca as usize - 1]
                            == "_"
                        {
                            tabuleiro.tamanho[linha_peca as usize][coluna_peca as usize] =
                                "_".to_string();
                            tabuleiro.tamanho[linha_peca as usize - 1][coluna_peca as usize - 1] =
                                "x".to_string();
                            realizou_movimento = true;
                        } else if tabuleiro.tamanho[linha_peca as usize - 1]
                            [coluna_peca as usize - 1]
                            == "o"
                        {
                            if come_peca(
                                &mut coluna_peca,
                                &mut linha_peca,
                                lado_movimento,
                                opcao_escolhida,
                                &mut comeu_peca,
                                tabuleiro,
                            ) == true
                            {
                                realizou_movimento = true;
                            }
                        }
                    }
                }
            }

            return realizou_movimento;
        }

        let mut verifica_jogada_atual = false;
        let mut numero_linha_atual: i32 = 0;
        let mut numero_coluna_atual: i32 = 0;
        let mut direcao_movimento = -1;

        while verifica_jogada_atual == false {
            let mut seleciona_peca = false;
            while seleciona_peca == false {
                println!(
                    "Jogador {opcao_escolhida} por favor escolha a linha da peça que deseja mover"
                );
                numero_linha_atual = Self::gera_numero(&self);
                println!(
                    "Jogador {opcao_escolhida} por favor escolha a coluna da peça que deseja mover"
                );
                numero_coluna_atual = Self::gera_numero(&self);
                if tabuleiro.tamanho[numero_linha_atual as usize][numero_coluna_atual as usize]
                    == opcao_escolhida
                {
                    seleciona_peca = true;
                } else {
                    println!("Você selecionou uma peça inválida");
                }
            }
            let mut escolhe_lado = false;
            while escolhe_lado == false {
                println!("Você deseja movimentar sua peça para a direita ou para a esquerda? Digite 0 - Direita e 1 para Esquerda");
                direcao_movimento = Self::gera_numero(&self);
                escolhe_lado = true;
            }

            if (escolhe_lado == true && seleciona_peca == true) {
                if (movimenta(
                    numero_coluna_atual,
                    numero_linha_atual,
                    direcao_movimento,
                    opcao_escolhida,
                    tabuleiro,
                ) == true)
                {
                    verifica_jogada_atual = true;
                }
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
}
