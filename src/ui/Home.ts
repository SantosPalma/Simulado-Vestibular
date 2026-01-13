// src/ui/Home.ts
import { criarSeletorProva } from './SeletorProva';
import { criarSimulado } from './Simulado'; // ← componente gerenciado
import { criarResultado } from './Resultado';
import type { Prova } from '../state/SimuladoClient';

export function renderizarHome(containerPai: HTMLElement): void {
  limparContainer(containerPai);

  const seletor = criarSeletorProva((simuladoId, provaId, prova) => {
    renderizarSimulado(containerPai, simuladoId, provaId, prova);
  });

  containerPai.appendChild(seletor);
}

function renderizarSimulado(
  containerPai: HTMLElement,
  simuladoId: number,
  provaId: string,
  prova: Prova
): void {
  limparContainer(containerPai);
  
  // Passa o simuladoId para o resultado
  const simulado = criarSimulado(simuladoId, provaId, prova, () => {
    renderizarResultado(containerPai, prova, simuladoId); // ← passa simuladoId
  });
  
  containerPai.appendChild(simulado);
}

function renderizarResultado(
  containerPai: HTMLElement,
  prova: Prova,
  simuladoId: number // ← recebe simuladoId, não respostas
): void {
  limparContainer(containerPai);
  
  // Passa simuladoId para carregar do backend
  const resultado = criarResultado(prova, simuladoId, () => {
    renderizarHome(containerPai);
  });
  
  containerPai.appendChild(resultado);
}

function limparContainer(container: HTMLElement): void {
  container.replaceChildren();
}