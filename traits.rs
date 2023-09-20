pub struct Jogador {
    pub simbolo: String,
}

impl Jogador {
    pub fn new(simbolo: String) -> Self {
        Self {
            simbolo: simbolo,
        }
    }
}

pub struct Tabuleiro {
    pub tamanho: Vec<Vec<String>>,
}

impl Tabuleiro {
    pub fn new(tamanho: Vec<Vec<String>>) -> Self {
        Self {
            tamanho: tamanho,
        }
    }
}

pub trait ComportamentoJogo {
    fn imprime_tabuleiro(&self,tabuleiro: &Tabuleiro);
    fn gera_numero(&self) -> i32;
    fn verifica_linha(&self,tabuleiro: &mut Tabuleiro, opcao_escolhida: &str);
    fn verifica_vitoria(&self,tabuleiro: &Tabuleiro, opcao_escolhida: &str) -> bool;
    fn verifica_empate(&self,tabuleiro: &Tabuleiro) -> bool;
}