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

/// Lista todos os IDs de provas disponíveis no formato "vestibular/nome_pasta"
pub fn listar_ids(&self) -> Result<Vec<String>, std::io::Error> {
    let mut ids = Vec::new();
    
    if !self.provas_dir.exists() {
        println!("⚠️ Pasta de provas não existe: {:?}", self.provas_dir);
        return Ok(ids);
    }

    println!("🔍 Listando provas em: {:?}", self.provas_dir);
    
    // Percorre cada pasta vestibular (enem, fuvest, etc.)
    for entry in fs::read_dir(&self.provas_dir)? {
        let entry = entry?;
        let vestibular_path = entry.path();
        
        // Verifica se é um diretório (vestibular)
        if vestibular_path.is_dir() {
            if let Some(vestibular) = vestibular_path.file_name().and_then(|s| s.to_str()) {
                println!("📁 Encontrado vestibular: {}", vestibular);
                
                // Agora percorre as SUBPASTAS (provas individuais)
                for prova_entry in fs::read_dir(&vestibular_path)? {
                    let prova_entry = prova_entry?;
                    let prova_path = prova_entry.path();
                    
                    // Verifica se é uma subpasta (prova individual)
                    if prova_path.is_dir() {
                        if let Some(nome_prova) = prova_path.file_name().and_then(|s| s.to_str()) {
                            // ✅ Verifica se existe arquivo prova.json dentro da subpasta
                            let json_path = prova_path.join("prova.json");
                            if json_path.exists() {
                                let id = format!("{}/{}", vestibular, nome_prova);
                                println!("📄 Encontrada prova: {}", id);
                                ids.push(id);
                            } else {
                                println!("⚠️ Pasta {} não contém prova.json", nome_prova);
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("✅ Provas listadas: {:?}", ids);
    Ok(ids)
}
pub fn carregar(&self, prova_id: &str) -> Result<Prova, ProvaServiceError> {
    let prova_path = self.provas_dir.join(prova_id).join("prova.json");
    
    println!("📂 Tentando carregar prova de: {:?}", prova_path);
    
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