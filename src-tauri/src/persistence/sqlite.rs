use rusqlite::{Connection, Result as RusqliteResult};
use std::path::Path;

pub fn connect(db_path: &Path) -> RusqliteResult<Connection> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    }

    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS usuario (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS simulado (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vestibular TEXT NOT NULL,
            ano INTEGER NOT NULL,
            prova_id TEXT NOT NULL,
            tempo_limite INTEGER NOT NULL,
            iniciado_em DATETIME,
            finalizado_em DATETIME,
            estado_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS resposta (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            simulado_id INTEGER NOT NULL,
            questao_id TEXT NOT NULL,
            alternativa_marcada TEXT,
            correta BOOLEAN,
            FOREIGN KEY (simulado_id) REFERENCES simulado(id)
        );

        CREATE TABLE IF NOT EXISTS resultado (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            simulado_id INTEGER NOT NULL UNIQUE,
            total_questoes INTEGER,
            acertos INTEGER,
            erros INTEGER,
            pontuacao REAL,
            FOREIGN KEY (simulado_id) REFERENCES simulado(id)
        );

        CREATE TABLE IF NOT EXISTS anotacao (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT,
            conteudo TEXT NOT NULL,
            criada_em DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS agenda_evento (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            descricao TEXT,
            data_inicio DATETIME NOT NULL,
            data_fim DATETIME
        );

        CREATE INDEX IF NOT EXISTS idx_resposta_simulado ON resposta(simulado_id);
        CREATE INDEX IF NOT EXISTS idx_simulado_vestibular ON simulado(vestibular);
        "
    )?;

    Ok(conn)
}
