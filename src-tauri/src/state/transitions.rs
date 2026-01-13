// transitions.rs
use crate::domain::estado::{EstadoSimulado, ModoTempo, EstadoSimuladoCompleto};
use thiserror::Error;
use chrono::{Utc, Duration};

#[derive(Debug, Error)]
pub enum TransicaoErro {
    #[error("Estado inválido para esta transição")]
    EstadoInvalido,
    #[error("Tempo não foi iniciado")]
    TempoNaoIniciado,
}

pub fn iniciar(estado: &mut EstadoSimuladoCompleto) -> Result<(), TransicaoErro> {
    if estado.estado != EstadoSimulado::NaoIniciado {
        return Err(TransicaoErro::EstadoInvalido);
    }
    
    estado.tempo.inicio = Some(Utc::now());
    estado.estado = EstadoSimulado::EmAndamento;
    
    println!("⏰ Transição iniciar: tempo.inicio definido para {:?}", estado.tempo.inicio);
    Ok(())
}

pub fn pausar(estado: &mut EstadoSimuladoCompleto) -> Result<(), TransicaoErro> {
    if estado.estado != EstadoSimulado::EmAndamento {
        return Err(TransicaoErro::EstadoInvalido);
    }
    
    // ✅ CALCULA E SALVA O TEMPO DECORRIDO ANTES DE PAUSAR
    if let Some(inicio) = estado.tempo.inicio {
        let agora = Utc::now();
        let decorrido_total = agora.signed_duration_since(inicio).num_seconds();
        estado.tempo.decorrido_segundos = decorrido_total.max(0) as u32;
        
        // ✅ MANTÉM O tempo.inicio para uso futuro!
        println!("⏸️ Pausando: inicio={:?}, decorrido={}", inicio, estado.tempo.decorrido_segundos);
    } else {
        return Err(TransicaoErro::TempoNaoIniciado);
    }
    
    estado.tempo.pausado_em = Some(Utc::now());
    estado.estado = EstadoSimulado::Pausado;
    Ok(())
}

pub fn retomar(estado: &mut EstadoSimuladoCompleto) -> Result<(), TransicaoErro> {
    if estado.estado != EstadoSimulado::Pausado {
        return Err(TransicaoErro::EstadoInvalido);
    }
    
    let pausado_em = estado.tempo.pausado_em.ok_or(TransicaoErro::TempoNaoIniciado)?;
    let inicio = estado.tempo.inicio.ok_or(TransicaoErro::TempoNaoIniciado)?;
    
    // ✅ CALCULA A DURAÇÃO DA PAUSA
    let duracao_pausa = Utc::now().signed_duration_since(pausado_em);
    
    // ✅ ATUALIZA O TEMPO DE INÍCIO PARA COMPENSAR O TEMPO DE PAUSA
    estado.tempo.inicio = Some(inicio + duracao_pausa);
    estado.tempo.pausado_em = None;
    
    println!("▶️ Retomando: novo_inicio={:?}", estado.tempo.inicio);
    
    estado.estado = EstadoSimulado::EmAndamento;
    Ok(())
}
pub fn finalizar(estado: &mut EstadoSimuladoCompleto) -> Result<(), TransicaoErro> {
    match estado.estado {
        EstadoSimulado::EmAndamento | EstadoSimulado::Pausado => {
            estado.tempo.finalizado_em = Some(Utc::now());
            estado.estado = EstadoSimulado::Finalizado;
            Ok(())
        }
        _ => Err(TransicaoErro::EstadoInvalido),
    }
}

pub fn verificar_expiracao_tempo(estado: &mut EstadoSimuladoCompleto) -> Result<(), TransicaoErro> {
    if estado.estado != EstadoSimulado::EmAndamento {
        return Ok(());
    }

    let inicio = estado.tempo.inicio.ok_or(TransicaoErro::TempoNaoIniciado)?;
    let agora = Utc::now();
    let decorrido = (agora - inicio).num_seconds().max(0) as u32;
    estado.tempo.decorrido_segundos = decorrido;

    let limite_segundos = estado.tempo.limite_minutos as u32 * 60;
    if limite_segundos > 0 && decorrido >= limite_segundos {
        if estado.configuracoes.permitir_ultrapassar_tempo {
            estado.modo_tempo = ModoTempo::Livre;
        } else {
            finalizar_por_tempo(estado);
        }
    }
    Ok(())
}

fn finalizar_por_tempo(estado: &mut EstadoSimuladoCompleto) {
    estado.tempo.finalizado_em = Some(Utc::now());
    estado.estado = EstadoSimulado::FinalizadoPorTempo;
}