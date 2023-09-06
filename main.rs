use std::io;

fn imprimeTabuleiro(tab: Vec<Vec<String>>) {
    for i in 0..3 {
        for j in 0..3 {
            print!(" {} ", tab[i][j]);
        }
        println!("");
    }
}

fn checaLinhaInclinada(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[0][0] == opcaoEscolhida.trim().to_string() && tabuleiro[1][1] == opcaoEscolhida.trim().to_string() && tabuleiro[2][2] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn checaLinhaInclinada2(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[0][0] == opcaoEscolhida.trim().to_string() && tabuleiro[1][0] == opcaoEscolhida.trim().to_string() && tabuleiro[2][0] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn checaLinhaHorizontalEsquerda(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[0][2] == opcaoEscolhida.trim().to_string() && tabuleiro[1][1] == opcaoEscolhida.trim().to_string() && tabuleiro[2][0] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn checaLinhaVerticalCima(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[0][0] == opcaoEscolhida.trim().to_string() && tabuleiro[0][1] == opcaoEscolhida.trim().to_string() && tabuleiro[0][2] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn checaLinhaHorizontalDireita(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[0][2] == opcaoEscolhida.trim().to_string() && tabuleiro[1][2] == opcaoEscolhida.trim().to_string() && tabuleiro[2][2] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn checaLinhaVerticalBaixo(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[2][0] == opcaoEscolhida.trim().to_string() && tabuleiro[2][1] == opcaoEscolhida.trim().to_string() && tabuleiro[2][2] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn checaLinhaHorizontalMeio(tabuleiro: Vec<Vec<String>>, opcaoEscolhida:&str) -> bool {
    if tabuleiro[0][1] == opcaoEscolhida.trim().to_string() && tabuleiro[1][1] == opcaoEscolhida.trim().to_string() && tabuleiro[2][1] == opcaoEscolhida.trim().to_string() {
        return true;
    }
    return false;
}

fn verificaEmpate(tabuleiro: Vec<Vec<String>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if(tabuleiro[i][j]) == "_" {
                return false;
            }
        }
    }

    return true;

}

fn main() {
   println! ("Participante 1 escolha X ou O");
   let mut opcaoEscolhida = String::new();
   io::stdin().read_line(&mut opcaoEscolhida).expect("Falha ao ler a opcao");

   while opcaoEscolhida.trim() != "X" && opcaoEscolhida.trim() != "O" {
       println! ("Digite novamente");
       opcaoEscolhida = String::new();
       io::stdin().read_line(&mut opcaoEscolhida).expect("Falha ao ler a opcao");
}

   println! ("Voce escolheu a opcao {opcaoEscolhida}");

   let opcaoEscolhida2;
   if opcaoEscolhida.trim() == "X" {
       opcaoEscolhida2 = "O";

    } else {
        opcaoEscolhida2 = "X";
        }

        println! ("Participante 2 sera a opcao {opcaoEscolhida2}");

        let mut tabuleiro: Vec<Vec<String>> = vec![vec!["_".to_string();3];3];

        let mut jogoRodando = true;

        while jogoRodando == true {

        let mut verificaJogada = false;

        if verificaEmpate(tabuleiro.clone()) {
            jogoRodando = false;
            println!("Empate!")
        }

        if jogoRodando == true {
        while verificaJogada == false {

        println!("Primeiro jogador escolha a linha: ");
        let mut linhaEscolhida = String::new();
        io::stdin().read_line(&mut linhaEscolhida).expect("Falha ao ler a opcao");
        let numeroLinha :usize = linhaEscolhida.trim().parse().expect("Falha ao ler o numero");
        println!("Primeiro jogador escolha a coluna: ");
        let mut colunaEscolhida = String::new();
        io::stdin().read_line(&mut colunaEscolhida).expect("Falha ao ler a opcao");
        let numeroColuna :usize = colunaEscolhida.trim().parse().expect("Falha ao ler o numero");

        if tabuleiro[numeroLinha][numeroColuna] == "_" {
            tabuleiro[numeroLinha][numeroColuna] = opcaoEscolhida.trim().to_string();
            imprimeTabuleiro(tabuleiro.clone());
            verificaJogada = true;
        }

        if checaLinhaInclinada(tabuleiro.clone(),opcaoEscolhida.trim()) || checaLinhaInclinada2(tabuleiro.clone(),opcaoEscolhida.trim()) || checaLinhaHorizontalEsquerda(tabuleiro.clone(),opcaoEscolhida.trim()) || checaLinhaVerticalCima(tabuleiro.clone(),opcaoEscolhida.trim()) || checaLinhaHorizontalDireita(tabuleiro.clone(),opcaoEscolhida.trim()) || checaLinhaVerticalBaixo(tabuleiro.clone(),opcaoEscolhida.trim()) || checaLinhaHorizontalMeio(tabuleiro.clone(),opcaoEscolhida.trim()) {
            println!("O Jogador um venceu!");
            jogoRodando = false;
        }

        }

        }

        if verificaEmpate(tabuleiro.clone()) {
            jogoRodando = false;
            println!("Empate!");
        }

        if jogoRodando == true {
        verificaJogada = false;
        while verificaJogada == false {
        println!("Segundo jogador escolha a linha: ");
        let mut linhaEscolhida = String::new();
        io::stdin().read_line(&mut linhaEscolhida).expect("Falha ao ler a opcao");
        let numeroLinha :usize = linhaEscolhida.trim().parse().expect("Falha ao ler o numero");
        println!("Segundo jogador escolha a coluna: ");
        let mut colunaEscolhida = String::new();
        io::stdin().read_line(&mut colunaEscolhida).expect("Falha ao ler a opcao");
        let numeroColuna :usize = colunaEscolhida.trim().parse().expect("Falha ao ler o numero");
                if tabuleiro[numeroLinha][numeroColuna] == "_" {
            tabuleiro[numeroLinha][numeroColuna] = opcaoEscolhida2.trim().to_string();
            imprimeTabuleiro(tabuleiro.clone());
            verificaJogada = true;
        }

                if checaLinhaInclinada(tabuleiro.clone(),opcaoEscolhida2.trim()) || checaLinhaInclinada2(tabuleiro.clone(),opcaoEscolhida2.trim()) || checaLinhaHorizontalEsquerda(tabuleiro.clone(),opcaoEscolhida2.trim()) || checaLinhaVerticalCima(tabuleiro.clone(),opcaoEscolhida2.trim()) || checaLinhaHorizontalDireita(tabuleiro.clone(),opcaoEscolhida2.trim()) || checaLinhaVerticalBaixo(tabuleiro.clone(),opcaoEscolhida2.trim()) || checaLinhaHorizontalMeio(tabuleiro.clone(),opcaoEscolhida2.trim()) {
                    println!("O Jogador dois venceu!");
                    jogoRodando = false;
                    break;
        }

        }
        }
        }

}
