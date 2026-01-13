use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Questao {
    pub id: String,           // ex: "Q01"
    pub area_id: String,      // ex: "linguagens"
    pub numero: u32,
    pub enunciado: String,
    #[serde(default)]
    pub imagens: Vec<String>, // caminhos relativos a assets/
    pub alternativas: Vec<Alternativa>,
    pub resposta_correta: String, // ex: "C"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alternativa {
    pub id: String,   // "A", "B", ...
    pub texto: String,
}