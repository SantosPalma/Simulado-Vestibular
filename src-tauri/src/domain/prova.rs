use serde::{Deserialize, Serialize};
use crate::domain::questao::Questao;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prova {
    pub schema_version: String,
    pub content_version: String,
    pub vestibular: String,
    pub ano: u16,
    pub dia: Option<u8>,
    pub duracao_minutos: u16,
    pub total_questoes: usize,
    pub questoes: Vec<Questao>,
}

impl Prova {
    pub fn validate_schema(&self) -> Result<(), ProvaError> {
        if self.schema_version != "1.0" {
            return Err(ProvaError::UnsupportedSchema(self.schema_version.clone()));
        }
        if self.questoes.len() != self.total_questoes {
            return Err(ProvaError::InconsistentQuestionCount {
                expected: self.total_questoes,
                actual: self.questoes.len(),
            });
        }
        Ok(())
    }

    pub fn id(&self) -> String {
        // Deriva um ID consistente: enem_2022_dia1
        let mut id = format!("{}_{}", self.vestibular.to_lowercase(), self.ano);
        if let Some(dia) = self.dia {
            id.push_str(&format!("_dia{}", dia));
        }
        id
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ProvaError {
    #[error("Versão de schema não suportada: {0}. Use '1.0'.")]
    UnsupportedSchema(String),
    #[error("Número de questões inconsistente: esperado {expected}, encontrado {actual}")]
    InconsistentQuestionCount { expected: usize, actual: usize },
}