use std::fs;
use std::path::{Path, PathBuf};
use crate::domain::prova::{Prova, ProvaError};

pub struct ProvaService {
    provas_dir: PathBuf,
}

impl ProvaService {
    pub fn new(provas_dir: PathBuf) -> Self {
        Self { provas_dir }
    }

    /// Lista todos os IDs de provas disponíveis no formato "vestibular/nome_arquivo"
    pub fn listar_ids(&self) -> Result<Vec<String>, std::io::Error> {
        let mut ids = Vec::new();
        
        if self.provas_dir.exists() {
            // Percorre cada pasta vestibular (enem, fuvest, etc.)
            for entry in fs::read_dir(&self.provas_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let vestibular = entry.file_name().to_string_lossy().into_owned();
                    let vestibular_path = entry.path();
                    
                    // Dentro de cada pasta, procura arquivos JSON
                    for prova_entry in fs::read_dir(vestibular_path)? {
                        let prova_entry = prova_entry?;
                        let path = prova_entry.path();
                        
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                            if let Some(nome_arquivo) = path.file_stem().and_then(|s| s.to_str()) {
                                // Cria ID no formato "vestibular/nome_arquivo"
                                ids.push(format!("{}/{}", vestibular, nome_arquivo));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(ids)
    }

    /// Carrega uma prova pelo ID (ex: "enem/2022_dia1")
    pub fn carregar(&self, prova_id: &str) -> Result<Prova, ProvaServiceError> {
        // Correção: o ID já inclui a pasta + nome do arquivo
        // prova_id = "enem/2022_dia1" → caminho = "provas/enem/2022_dia1.json"
        let prova_path = self.provas_dir.join(prova_id).with_extension("json");
        
        println!("📂 Tentando carregar prova de: {:?}", prova_path); // Debug
        
        if !prova_path.exists() {
            return Err(ProvaServiceError::NaoEncontrada(prova_id.to_string()));
        }

        let conteudo = fs::read_to_string(&prova_path)
            .map_err(|e| ProvaServiceError::LeituraFalhou(prova_path.clone(), e))?;

        let prova: Prova = serde_json::from_str(&conteudo)
            .map_err(|e| ProvaServiceError::ParseJson(prova_path, e))?;

        prova.validate_schema()
            .map_err(ProvaServiceError::Validacao)?;

        Ok(prova)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProvaServiceError {
    #[error("Prova não encontrada: {0}")]
    NaoEncontrada(String),
    #[error("Erro ao ler arquivo {0}: {1}")]
    LeituraFalhou(PathBuf, #[source] std::io::Error),
    #[error("Erro ao fazer parse do JSON em {0}: {1}")]
    ParseJson(PathBuf, #[source] serde_json::Error),
    #[error("Falha na validação da prova: {0}")]
    Validacao(#[from] ProvaError),
}