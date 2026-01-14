use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoSimulado {
    NaoIniciado,
    EmAndamento,
    Pausado,
    Finalizado,
    FinalizadoPorTempo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModoTempo {
    Cronometrado,
    Livre,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TempoSimulado {
    pub limite_minutos: u16,
    pub decorrido_segundos: u32,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub inicio: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub pausado_em: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub finalizado_em: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressoSimulado {
    pub questao_atual: String, // ex: "Q12"
    pub respondidas: usize,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfiguracoesSimulado {
    pub permitir_ultrapassar_tempo: bool,
    pub mostrar_gabarito_ao_final: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EstadoSimuladoCompleto {
    pub estado: EstadoSimulado,
    pub modo_tempo: ModoTempo,
    pub tempo: TempoSimulado,
    pub progresso: ProgressoSimulado,
    pub respostas: HashMap<String, Option<String>>, // "Q01" => Some("A") ou None
    pub configuracoes: ConfiguracoesSimulado,
}

impl EstadoSimulado {
    pub const fn pode_avancar(&self) -> bool {
        matches!(self, Self::EmAndamento)
    }
    
    pub const fn pode_pausar(&self) -> bool {
        matches!(self, Self::EmAndamento)
    }
    
    pub const fn pode_retomar(&self) -> bool {
        matches!(self, Self::Pausado)
    }
    
    pub const fn esta_ativo(&self) -> bool {
        matches!(self, Self::EmAndamento | Self::Pausado)
    }
}

impl ProgressoSimulado {
    pub fn validate(&self) -> Result<(), String> {
        if self.total == 0 {
            return Ok(());
        }
        
        let numero_str = self.questao_atual.strip_prefix('Q')
            .ok_or_else(|| "ID da questão deve começar com 'Q'".to_string())?;
        
        let numero_atual: usize = numero_str.parse()
            .map_err(|_| "Número da questão inválido".to_string())?;
        
        if numero_atual < 1 || numero_atual > self.total {
            return Err(format!(
                "Questão atual {} fora dos limites [1, {}]", 
                self.questao_atual, self.total
            ));
        }
        
        if self.respondidas > self.total {
            return Err("Número de questões respondidas excede o total".to_string());
        }
        
        Ok(())
    }
}

impl Default for EstadoSimuladoCompleto {
    fn default() -> Self {
        Self {
            estado: EstadoSimulado::NaoIniciado,
            modo_tempo: ModoTempo::Cronometrado,
            tempo: TempoSimulado {
                limite_minutos: 0,
                decorrido_segundos: 0,
                inicio: None,
                pausado_em: None,
                finalizado_em: None,
            },
            progresso: ProgressoSimulado {
                questao_atual: "Q01".to_string(),
                respondidas: 0,
                total: 0,
            },
            respostas: std::collections::HashMap::new(),
            configuracoes: ConfiguracoesSimulado {
                permitir_ultrapassar_tempo: true,
                mostrar_gabarito_ao_final: true,
            },
        }
    }
}

impl Default for TempoSimulado {
    fn default() -> Self {
        Self {
            limite_minutos: 0,
            decorrido_segundos: 0,
            inicio: None,
            pausado_em: None,
            finalizado_em: None,
        }
    }
}

impl Default for ProgressoSimulado {
    fn default() -> Self {
        Self {
            questao_atual: "Q01".to_string(),
            respondidas: 0,
            total: 0,
        }
    }
}

impl Default for ConfiguracoesSimulado {
    fn default() -> Self {
        Self {
            permitir_ultrapassar_tempo: true,
            mostrar_gabarito_ao_final: true,
        }
    }
}