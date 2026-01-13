// src/state/SimuladoClient.ts
import { invoke } from '@tauri-apps/api/core';

export interface Prova {
  id: any;
  schema_version: string;
  content_version: string;
  vestibular: string;
  ano: number;
  dia?: number;
  duracao_minutos: number;
  total_questoes: number;
  questoes: Questao[];
}

export interface Questao {
  id: string;
  area_id: string;
  numero: number;
  enunciado: string;
  imagens: string[];
  alternativas: Alternativa[];
  resposta_correta: string;
}

export interface Alternativa {
  id: string;
  texto: string;
}

export interface EstadoSimuladoCompleto {
  estado: string;
  modo_tempo: string;
  tempo: {
    limite_minutos: number;
    decorrido_segundos: number;
    inicio: string | null;
    pausado_em: string | null;
    finalizado_em: string | null;
  };
  progresso: {
    questao_atual: string;
    respondidas: number;
    total: number;
  };
  respostas: Record<string, string | null>;
  configuracoes: {
    permitir_ultrapassar_tempo: boolean;
    mostrar_gabarito_ao_final: boolean;
  };
}

export async function listarProvas(): Promise<string[]> {
  return invoke<string[]>('listar_provas');
}

export async function carregarProva(provaId: string): Promise<Prova> {
  return invoke<Prova>('carregar_prova', { provaId });
}
export async function iniciarSimulado(
  provaId: string,
  vestibular: string,
  ano: number,
  duracaoMinutos: number
): Promise<number> {
  return await invoke('iniciar_simulado', {
    provaId,
    vestibular,
    ano,
    duracaoMinutos
  });
}


export interface DetalheQuestao {
  questao_id: string;
  numero: number;
  resposta_usuario: string | null;
  gabarito: string;
  acertou: boolean;
}

export interface ResultadoSimulado {
  total_questoes: number;
  acertos: number;
  erros: number;
  pontuacao: number;
  detalhes: DetalheQuestao[];
}

export async function obterEstadoSimulado(simuladoId: number): Promise<EstadoSimuladoCompleto> {
  return await invoke('obter_estado_simulado', { simuladoId });
}

export async function responderQuestao(
  simuladoId: number,
  questaoId: string,
  alternativa: string | null
): Promise<void> {
  return await invoke('responder_questao', { simuladoId, questaoId, alternativa });
}

export async function avancarQuestao(simuladoId: number): Promise<void> {
  return await invoke('avancar_questao', { simuladoId });
}

export async function voltarQuestao(simuladoId: number): Promise<void> {
  return await invoke('voltar_questao', { simuladoId });
}


export async function pausarSimulado(simuladoId: number): Promise<void> {
  return await invoke('pausar_simulado', { simuladoId });
}

export async function retomarSimulado(simuladoId: number): Promise<void> {
  return await invoke('retomar_simulado', { simuladoId });
}

export async function finalizarSimulado(simuladoId: number): Promise<void> {
  return await invoke('finalizar_simulado', { simuladoId });
}


export async function obterResultado(simuladoId: number): Promise<ResultadoSimulado> {
  return await invoke('obter_resultado', { simuladoId });
}

export async function atualizarTempoSimulado(simuladoId: number): Promise<void> {
  return await invoke('atualizar_tempo_simulado', { simuladoId });
}