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
        
        if !self.provas_dir.exists() {
            println!("⚠️ Pasta de provas não existe: {:?}", self.provas_dir);
            return Ok(ids);
        }

        println!("🔍 Listando provas em: {:?}", self.provas_dir);
        
        for entry in fs::read_dir(&self.provas_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let vestibular = path.file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                
                println!("📁 Encontrado vestibular: {}", vestibular);
                
                if let Ok(vestibular_dir) = fs::read_dir(&path) {
                    for prova_entry in vestibular_dir {
                        let prova_entry = prova_entry?;
                        let prova_path = prova_entry.path();
                        
                        if prova_path.is_file() && prova_path.extension().map_or(false, |ext| ext == "json") {
                            if let Some(nome_arquivo) = prova_path.file_stem().and_then(|s| s.to_str()) {
                                let id = format!("{}/{}", vestibular, nome_arquivo);
                                println!("📄 Encontrada prova: {}", id);
                                ids.push(id);
                            }
                        }
                    }
                }
            }
        }
        
        println!("✅ Provas listadas: {:?}", ids);
        Ok(ids)
    }

    /// Carrega uma prova pelo ID (ex: "enem/2022_dia1")
    pub fn carregar(&self, prova_id: &str) -> Result<Prova, ProvaServiceError> {
        // Correção: o ID já inclui a pasta + nome do arquivo
        // prova_id = "enem/2022_dia1" → caminho = "provas/enem/2022_dia1.json"
        let prova_path = self.provas_dir.join(prova_id).with_extension("json");
        
        println!("📂 Tentando carregar prova de: {:?}", prova_path); // Debug
        
        if !prova_path.exists() {
            println!("❌ Arquivo não encontrado: {:?}", prova_path);
            return Err(ProvaServiceError::NaoEncontrada(prova_id.to_string()));
        }

        let conteudo = fs::read_to_string(&prova_path)
            .map_err(|e| {
                println!("❌ Erro ao ler arquivo: {}", e);
                ProvaServiceError::LeituraFalhou(prova_path.clone(), e)
            })?;

        let prova: Prova = serde_json::from_str(&conteudo)
            .map_err(|e| {
                println!("❌ Erro ao parsear JSON: {}", e);
                ProvaServiceError::ParseJson(prova_path, e)
            })?;

        prova.validate_schema()
            .map_err(ProvaServiceError::Validacao)?;

        println!("✅ Prova carregada com sucesso: {}", prova_id);
        Ok(prova)
    }

    pub fn questao_existe(&self, prova_id: &str, questao_id: &str) -> Result<bool, String> {
        match self.carregar(prova_id) {
            Ok(prova) => {
                let existe = prova.questoes.iter().any(|q| q.id == questao_id);
                println!("🔍 Questão {} {} na prova {}", 
                    questao_id, 
                    if existe { "ENCONTRADA" } else { "NÃO ENCONTRADA" },
                    prova_id);
                Ok(existe)
            },
            Err(e) => Err(format!("Erro ao carregar prova {}: {}", prova_id, e))
        }
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