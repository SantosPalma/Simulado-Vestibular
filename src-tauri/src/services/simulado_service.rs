use crate::domain::{estado::EstadoSimulado, simulado::Simulado};
use crate::domain::estado::EstadoSimuladoCompleto; 
use crate::persistence::repository::SimuladoRepository;
use crate::state::transitions;
use anyhow::{Result, anyhow};
use chrono::Utc;

 
#[derive(serde::Serialize)]
pub struct SimuladoResumo {
    pub id: i64,
    pub vestibular: String,
    pub ano: i32,
    pub iniciado_em: Option<String>,
    pub estado: String,
}
#[derive(serde::Serialize)]
pub struct DetalheQuestao {
    pub questao_id: String,
    pub numero: u32,
    pub resposta_usuario: Option<String>,
    pub gabarito: String,
    pub acertou: bool,
}

#[derive(serde::Serialize)]
pub struct ResultadoSimulado {
    pub total_questoes: usize,
    pub acertos: usize,
    pub erros: usize,
    pub pontuacao: f64,
    pub detalhes: Vec<DetalheQuestao>, 

}

pub struct SimuladoService {
    repo: SimuladoRepository,
}

impl SimuladoService {
    pub fn new(repo: SimuladoRepository) -> Self {
        Self { repo }
    }

 pub fn iniciar_simulado(
    &self,
    prova_id: String,
    vestibular: String,
    ano: i32,
    duracao_minutos: i32,
) -> Result<i64> {
    let mut simulado = Simulado::novo(prova_id, vestibular, ano, duracao_minutos)?;
    let mut estado = simulado.estado()?;
    
    estado.tempo.limite_minutos = duracao_minutos as u16;
    
    // ✅ Aplica a transição que define tempo.inicio
    transitions::iniciar(&mut estado)?;
    
    // ✅ Verificação de segurança
    if estado.tempo.inicio.is_none() {
        return Err(anyhow!("Falha ao definir tempo de início do simulado"));
    }
    
    simulado.set_estado(&estado)?;
    simulado.iniciado_em = Some(Utc::now());
    
    let id = self.repo.salvar(&simulado)?;
    
    println!("✅ Simulado iniciado com ID: {}, tempo.inicio: {:?}", id, estado.tempo.inicio);
    Ok(id)
}

pub fn atualizar_tempo(&self, simulado_id: i64) -> Result<()> {
    let mut simulado = self.repo.buscar_por_id(simulado_id)?
        .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
    
    let mut estado = simulado.estado()?;
    
    if estado.estado == EstadoSimulado::EmAndamento {
        if let Some(inicio) = estado.tempo.inicio {
            let agora = Utc::now();
            let decorrido_total = agora.signed_duration_since(inicio).num_seconds();
            
            println!("⏰ Atualizando tempo: ID={}, inicio={:?}, agora={:?}, decorrido={}", 
                simulado_id, inicio, agora, decorrido_total);
                
            estado.tempo.decorrido_segundos = decorrido_total.max(0) as u32;
            
            simulado.set_estado(&estado)?;
            self.repo.salvar(&simulado)?;
        } else {
            println!("❌ ERRO: tempo.inicio é None para simulado {}", simulado_id);
            return Err(anyhow!("Tempo não foi iniciado para simulado {}", simulado_id));
        }
    }
    
    Ok(())
}
pub fn pausar_simulado(&self, simulado_id: i64) -> Result<()> {
    let mut simulado = self.repo.buscar_por_id(simulado_id)?
        .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
    
    let mut estado = simulado.estado()?;
    
    // ✅ NÃO RECALCULA O TEMPO AQUI - A TRANSIÇÃO FAZ ISSO!
    transitions::pausar(&mut estado)?;
    
    simulado.set_estado(&estado)?;
    self.repo.salvar(&simulado)?;
    
    println!("⏸️ Simulado {} pausado. Tempo decorrido: {}", simulado_id, estado.tempo.decorrido_segundos);
    Ok(())
}

pub fn retomar_simulado(&self, simulado_id: i64) -> Result<()> {
    let mut simulado = self.repo.buscar_por_id(simulado_id)?
        .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
    
    let mut estado = simulado.estado()?;
    
    // ✅ VERIFICA SE TEMOS tempo.inicio ANTES DE RETOMAR
    if estado.tempo.inicio.is_none() {
        return Err(anyhow!("Tempo não foi iniciado para o simulado {}", simulado_id));
    }
    
    transitions::retomar(&mut estado)?;
    simulado.set_estado(&estado)?;
    self.repo.salvar(&simulado)?;
    
    println!("▶️ Simulado {} retomado. Novo tempo.inicio: {:?}", simulado_id, estado.tempo.inicio);
    Ok(())
}


      // Buscar estado atual do simulado
pub fn obter_estado(&self, simulado_id: i64) -> Result<EstadoSimuladoCompleto> {
        let simulado = self.repo.buscar_por_id(simulado_id)?
            .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
            simulado.estado().map_err(|e| anyhow!(e)) 

    }

     //  Avançar / voltar questão
pub fn avancar_questao(&self, simulado_id: i64) -> Result<()> {
    let mut simulado = self.repo.buscar_por_id(simulado_id)?
        .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
    
    let mut estado = simulado.estado()?;
    
    // Extrai o número da questão atual (ex: "Q01" → 1)
    let atual_numero = estado.progresso.questao_atual
        .strip_prefix('Q')
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);
    
    // Calcula próxima questão
    let proxima_numero = atual_numero + 1;
    let proxima_questao = format!("Q{:02}", proxima_numero); // Q01, Q02, etc.
    
    // Atualiza o estado
    estado.progresso.questao_atual = proxima_questao;
    
    simulado.set_estado(&estado)?;
    self.repo.salvar(&simulado)?;
    Ok(())
}

    pub fn voltar_questao(&self, simulado_id: i64) -> Result<()> {
        let mut simulado = self.repo.buscar_por_id(simulado_id)?
            .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
        
        let mut estado = simulado.estado()?;
        let atual_index = estado.progresso.questao_atual.strip_prefix('Q')
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        
        if atual_index > 1 {
            let anterior = format!("Q{}", atual_index - 1);
            estado.progresso.questao_atual = anterior;
        }
        
        simulado.set_estado(&estado)?;
        self.repo.salvar(&simulado)?;
        Ok(())
    }


    pub fn registrar_resposta(
    &self,
    simulado_id: i64,
    questao_id: String,
    alternativa: Option<String>,
    ) -> Result<()> {
    let mut simulado = self.repo.buscar_por_id(simulado_id)?
        .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
    
    let mut estado = simulado.estado()?;

    // Em registrar_resposta()
    println!("📝 Questão {} respondida: {:?}, respondidas: {}", 
    questao_id, alternativa, estado.progresso.respondidas);
    
    let era_respondida = estado.respostas.contains_key(&questao_id) 
        && estado.respostas[&questao_id].is_some();
    
    let agora_respondida = alternativa.is_some();
    
    if !era_respondida && agora_respondida {
        estado.progresso.respondidas += 1;
    } else if era_respondida && !agora_respondida {
        estado.progresso.respondidas = estado.progresso.respondidas.saturating_sub(1);
    }
    
    estado.respostas.insert(questao_id, alternativa);
    
    simulado.set_estado(&estado)?;
    self.repo.salvar(&simulado)?;
    Ok(())
}

    // Finalizar simulado (manual)
    pub fn finalizar_simulado(&self, simulado_id: i64) -> Result<()> {
        let mut simulado = self.repo.buscar_por_id(simulado_id)?
            .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
        
        let mut estado = simulado.estado()?;
        transitions::finalizar(&mut estado)?;
        simulado.set_estado(&estado)?;
        simulado.finalizado_em = Some(Utc::now());
        self.repo.salvar(&simulado)?;
        Ok(())
    }

 

pub fn calcular_resultado(&self, simulado_id: i64) -> Result<ResultadoSimulado> {
    let simulado = self.repo.buscar_por_id(simulado_id)?
        .ok_or_else(|| anyhow!("Simulado {} não encontrado", simulado_id))?;
    
    let estado = simulado.estado()?;
    let prova_service = crate::services::prova_service::ProvaService::new(
       
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("provas")
    );
    
    let prova = prova_service.carregar(&simulado.prova_id)
        .map_err(|e| anyhow!("Erro ao carregar prova: {}", e))?;

    let mut acertos = 0;
    let mut detalhes = Vec::new();
    
    for questao in &prova.questoes {
        let resposta_usuario = estado.respostas.get(&questao.id).cloned().flatten();
        let acertou = resposta_usuario.as_deref() == Some(&questao.resposta_correta);
        
        if acertou {
            acertos += 1;
        }
        
        detalhes.push(DetalheQuestao {
            questao_id: questao.id.clone(),
            numero: questao.numero,
            resposta_usuario,
            gabarito: questao.resposta_correta.clone(),
            acertou,
        });
    }

    let total = prova.total_questoes;
    let erros = total - acertos;
    let pontuacao = if total > 0 { (acertos as f64 / total as f64) * 100.0 } else { 0.0 };

    Ok(ResultadoSimulado {
        total_questoes: total,
        acertos,
        erros,
        pontuacao,
        detalhes,
    })
}

    // Listar simulados anteriores
 
    pub fn listar_simulados(&self) -> Result<Vec<SimuladoResumo>> {
        // ✅ Use apenas um método
        let todos = self.repo.listar_todos()?;
        
        let mut resumos = Vec::new();
        for sim in todos {
            let estado = sim.estado()?;
            resumos.push(SimuladoResumo {
                id: sim.id,
                vestibular: sim.vestibular,
                ano: sim.ano,
                iniciado_em: sim.iniciado_em.map(|dt| dt.to_rfc3339()),
                estado: format!("{:?}", estado.estado),
            });
        }
        Ok(resumos)
    }

    // Excluir simulado
    pub fn excluir(&self, simulado_id: i64) -> Result<()> {
        // Você precisará adicionar este método no repositório
        unimplemented!("Implementar exclusão no repositório")
    }

}