use tauri::State;
use std::sync::Arc;
use anyhow::Context;
use crate::domain::prova::Prova;
use crate::domain::estado::EstadoSimuladoCompleto;
use crate::services::prova_service::ProvaService;
use crate::services::simulado_service::{SimuladoService, SimuladoResumo, ResultadoSimulado};

// === Wrappers para compartilhar serviços entre threads ===
pub struct ProvaServiceWrapper(pub Arc<ProvaService>);
pub struct SimuladoServiceWrapper(pub Arc<SimuladoService>);

// === Comandos para Provas ===

#[tauri::command]
pub async fn listar_provas(
    service: State<'_, ProvaServiceWrapper>,
) -> Result<Vec<String>, String> {
    service.0.listar_ids()
        .map_err(|e| format!("Erro ao listar provas: {}", e))
}

#[tauri::command]
pub async fn carregar_prova(
    service: State<'_, ProvaServiceWrapper>,
    prova_id: String,
) -> Result<Prova, String> {
    service.0.carregar(&prova_id)
        .map_err(|e| format!("Erro ao carregar prova '{}': {}", prova_id, e))
}

// === Comandos para Simulados ===

#[tauri::command]
pub async fn iniciar_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    prova_id: String,
    vestibular: String,
    ano: i32,
    duracao_minutos: i32,
) -> Result<i64, String> {
    println!("🔧 Iniciando simulado: prova_id={}, vestibular={}, ano={}", prova_id, vestibular, ano);
    service.0
        .iniciar_simulado(prova_id, vestibular, ano, duracao_minutos)
        .map_err(|e| format!("Erro ao iniciar simulado: {}", e))
}

#[tauri::command]
pub async fn atualizar_tempo_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
     println!("🔧 Atualizando tempo para simulado {}", simulado_id);
    service.0
        .atualizar_tempo(simulado_id)
        .map_err(|e| format!("Erro ao atualizar tempo: {}", e))
}

#[tauri::command]
pub async fn pausar_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
    service.0
        .pausar_simulado(simulado_id)
        .map_err(|e| format!("Erro ao pausar simulado: {}", e))
}

#[tauri::command]
pub async fn retomar_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
    service.0
        .retomar_simulado(simulado_id)
        .map_err(|e| format!("Erro ao retomar simulado: {}", e))
}

// Obter estado atual do simulado
#[tauri::command]
pub async fn obter_estado_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<EstadoSimuladoCompleto, String> {
    service.0
        .obter_estado(simulado_id)
        .map_err(|e| format!("Erro ao obter estado do simulado: {}", e))
}

// Registrar resposta de questão
#[tauri::command]
pub async fn responder_questao(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
    questao_id: String,
    alternativa: Option<String>,
) -> Result<(), String> {
    service.0
        .registrar_resposta(simulado_id, questao_id, alternativa)
        .map_err(|e| format!("Erro ao registrar resposta: {}", e))
}

// Avançar / voltar questão
#[tauri::command]
pub async fn avancar_questao(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
    service.0
        .avancar_questao(simulado_id)
        .map_err(|e| format!("Erro ao avançar questão: {}", e))
}

#[tauri::command]
pub async fn voltar_questao(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
    service.0
        .voltar_questao(simulado_id)
        .map_err(|e| format!("Erro ao voltar questão: {}", e))
}

// Finalizar simulado (manual)
#[tauri::command]
pub async fn finalizar_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
    service.0
        .finalizar_simulado(simulado_id)
        .map_err(|e| format!("Erro ao finalizar simulado: {}", e))
}

// Obter resultado final
#[tauri::command]
pub async fn obter_resultado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<ResultadoSimulado, String> {
    service.0
        .calcular_resultado(simulado_id)
        .map_err(|e| format!("Erro ao calcular resultado: {}", e))
}

// Listar simulados anteriores
#[tauri::command]
pub async fn listar_simulados(
    service: State<'_, SimuladoServiceWrapper>,
) -> Result<Vec<SimuladoResumo>, String> {
    service.0
        .listar_simulados()
        .map_err(|e| format!("Erro ao listar simulados: {}", e))
}

// Excluir simulado
#[tauri::command]
pub async fn excluir_simulado(
    service: State<'_, SimuladoServiceWrapper>,
    simulado_id: i64,
) -> Result<(), String> {
    service.0
        .excluir(simulado_id)
        .map_err(|e| format!("Erro ao excluir simulado: {}", e))
}

// Comando para verificar se questão existe
#[tauri::command]
pub async fn questao_existe(
    service: State<'_, SimuladoServiceWrapper>,
    prova_id: String,
    questao_id: String,
) -> Result<bool, String> {
    let prova_service = crate::services::prova_service::ProvaService::new(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("provas")
    );
    
    prova_service
        .questao_existe(&prova_id, &questao_id)
        .map_err(|e| format!("Erro ao verificar questão: {}", e))
}