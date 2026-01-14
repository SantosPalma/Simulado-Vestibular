// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod state;
mod persistence;
mod services;
mod api;

use std::path::PathBuf;
use tauri::Manager;
use services::prova_service::ProvaService;
use services::simulado_service::SimuladoService;
use api::commands::{ProvaServiceWrapper, SimuladoServiceWrapper};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // === Caminho das provas ===
            let provas_dir = if cfg!(debug_assertions) {
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent().unwrap()  // src-tauri/
                    .join("provas")
            } else {
                app_handle.path()
                    .resource_dir()
                    .expect("Falha ao obter resource_dir")
                    .join("provas")
            };

            println!("🎯 Diretório de provas: {:?}", provas_dir);

            // === Caminho do banco de dados ===
            let db_path = if cfg!(debug_assertions) {
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent().unwrap()  // src-tauri/
                    .parent().unwrap()  // raiz do projeto
                    .join("data")
                    .join("simulados.db")
            } else {
                app_handle.path()
                    .app_data_dir()
                    .expect("Falha ao obter app_data_dir")
                    .join("simulados.db")
            };

            println!("🎯 Caminho do banco de dados: {:?}", db_path);

            // Cria diretórios se não existirem (só em desenvolvimento)
            if cfg!(debug_assertions) {
                if let Some(parent) = db_path.parent() {
                    std::fs::create_dir_all(parent)
                        .expect("Falha ao criar diretório de dados");
                }
                if !provas_dir.exists() {
                    println!("⚠️ AVISO: Pasta de provas não encontrada em {:?}", provas_dir);
                }
            }

            // Inicializa o banco
            let conn = persistence::sqlite::connect(&db_path)
                .expect("Falha ao conectar ao banco");

            // Serviço de provas
            let prova_service = ProvaService::new(provas_dir.clone());
            app.manage(ProvaServiceWrapper(std::sync::Arc::new(prova_service)));

            // Serviço de simulados -  Corrigido: passa provas_dir como segundo parâmetro
            let simulado_repo = persistence::repository::SimuladoRepository::new(conn);
            let simulado_service = SimuladoService::new(simulado_repo, provas_dir.clone());
            app.manage(SimuladoServiceWrapper(std::sync::Arc::new(simulado_service)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // === Comandos para Provas ===
            api::commands::listar_provas,
            api::commands::carregar_prova,
            
            // === Comandos para Simulados - Controle Básico ===
            api::commands::iniciar_simulado,
            api::commands::pausar_simulado,
            api::commands::retomar_simulado,
            api::commands::atualizar_tempo_simulado,
            
            // === Comandos para Simulados - Funcionalidades Essenciais ===
            api::commands::obter_estado_simulado,
            api::commands::responder_questao,
            api::commands::avancar_questao,
            api::commands::voltar_questao,
            api::commands::finalizar_simulado,
            api::commands::obter_resultado,
            
            // === Comandos para Simulados - Opcionais ===
            api::commands::listar_simulados,
            api::commands::excluir_simulado,
            
            // === Comandos Adicionais ===
            api::commands::questao_existe,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o aplicativo Tauri");
}