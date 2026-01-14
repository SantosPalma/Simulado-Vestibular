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

        let mut ids = std::collections::HashSet::new();
        for (i, questao) in self.questoes.iter().enumerate() {
            if !questao.id.starts_with('Q') || questao.id.len() < 2 {
                return Err(ProvaError::InvalidQuestionId(questao.id.clone()));
            }
            
            // Verifica unicidade
            if !ids.insert(questao.id.clone()) {
                return Err(ProvaError::DuplicateQuestionId(questao.id.clone()));
            }
            
            // Verifica número da questão corresponde ao índice
            if questao.numero != (i + 1) as u32 {
                return Err(ProvaError::QuestionNumberMismatch {
                    expected: (i + 1) as u32,
                    actual: questao.numero,
                    id: questao.id.clone(),
                });
            }
        }

        Ok(())
    }

    pub fn id(&self) -> String {
        let mut id = format!("{}_{}", self.vestibular.to_lowercase(), self.ano);
        if let Some(dia) = self.dia {
            id.push_str(&format!("_dia{}", dia));
        }
        id
    }

      pub fn path_id(&self) -> String {
        let nome_arquivo = if let Some(dia) = self.dia {
            format!("{}_dia{}", self.ano, dia)
        } else {
            self.ano.to_string()
        };
        format!("{}/{}", self.vestibular.to_lowercase(), nome_arquivo)
    }
    
    /// Gera um ID legível para exibição
    pub fn display_id(&self) -> String {
        let mut id = format!("{} {}", self.vestibular, self.ano);
        if let Some(dia) = self.dia {
            id.push_str(&format!(" (Dia {})", dia));
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
    #[error("ID de questão inválido: {0}. Deve estar no formato Q01, Q02, etc.")]
    InvalidQuestionId(String),
    #[error("ID de questão duplicado: {0}")]
    DuplicateQuestionId(String),
    #[error("Número da questão {id} inconsistente: esperado {expected}, encontrado {actual}")]
    QuestionNumberMismatch { expected: u32, actual: u32, id: String },
}