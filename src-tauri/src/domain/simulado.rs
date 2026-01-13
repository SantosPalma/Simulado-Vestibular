use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rusqlite::{Row, Result as RusqliteResult};
use crate::domain::estado::EstadoSimuladoCompleto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Simulado {
    pub id: i64,
    pub prova_id: String,
    pub vestibular: String,
    pub ano: i32,
    pub tempo_limite: i32,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub iniciado_em: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub finalizado_em: Option<DateTime<Utc>>,
    pub estado_json: String,
}

impl Simulado {
    // ✅ Construtor
    pub fn novo(
        prova_id: String,
        vestibular: String,
        ano: i32,
        duracao_minutos: i32,
    ) -> Result<Self, serde_json::Error> {
        let estado = EstadoSimuladoCompleto {
            tempo: crate::domain::estado::TempoSimulado {
                limite_minutos: duracao_minutos as u16,
                ..Default::default()
            },
            progresso: crate::domain::estado::ProgressoSimulado {
                total: 0,
                ..Default::default()
            },
            ..Default::default()
        };

        Ok(Simulado {
            id: 0,
            prova_id,
            vestibular,
            ano,
            tempo_limite: duracao_minutos,
            iniciado_em: None,
            finalizado_em: None,
            estado_json: serde_json::to_string(&estado)?,
        })
    }

    // ✅ Acesso ao estado
    pub fn estado(&self) -> Result<EstadoSimuladoCompleto, serde_json::Error> {
        serde_json::from_str(&self.estado_json)
    }

    // ✅ Atualização do estado
    pub fn set_estado(&mut self, estado: &EstadoSimuladoCompleto) -> Result<(), serde_json::Error> {
        self.estado_json = serde_json::to_string(estado)?;
        Ok(())
    }

    // ✅ Mapeamento do SQLite → Rust
    pub fn from_row(row: &Row<'_>) -> RusqliteResult<Self> {
        Ok(Simulado {
            id: row.get("id")?,
            prova_id: row.get("prova_id")?,
            vestibular: row.get("vestibular")?,
            ano: row.get("ano")?,
            tempo_limite: row.get("tempo_limite")?,
            iniciado_em: row.get("iniciado_em")?,
            finalizado_em: row.get("finalizado_em")?,
            estado_json: row.get("estado_json")?,
        })
    }
}