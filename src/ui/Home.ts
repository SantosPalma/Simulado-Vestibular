// src/ui/Home.ts
import { criarSeletorProva } from './SeletorProva';
import { criarSimulado } from './Simulado';
import { criarResultado } from './Resultado';
import type { Prova } from '../state/SimuladoClient';

export function renderizarHome(containerPai: HTMLElement): void {
  limparContainer(containerPai);

  try {
    const seletor = criarSeletorProva((simuladoId, provaId, prova) => {
      renderizarSimulado(containerPai, simuladoId, provaId, prova);
    });
    containerPai.appendChild(seletor);
  } catch (e) {
    console.error('Erro ao renderizar home:', e);
    containerPai.innerHTML = `<div class="erro">Erro ao carregar a interface inicial</div>`;
  }
}

function renderizarSimulado(
  containerPai: HTMLElement,
  simuladoId: number,
  provaId: string,
  prova: Prova
): void {
  limparContainer(containerPai);
  
  try {
    const simulado = criarSimulado(simuladoId, provaId, prova, () => {
      renderizarResultado(containerPai, prova, simuladoId);
    });
    containerPai.appendChild(simulado);
  } catch (e) {
    console.error('Erro ao renderizar simulado:', e);
    containerPai.innerHTML = `<div class="erro">Erro ao carregar o simulado</div>`;
    setTimeout(() => renderizarHome(containerPai), 3000);
  }
}

function renderizarResultado(
  containerPai: HTMLElement,
  prova: Prova,
  simuladoId: number
): void {
  limparContainer(containerPai);
  
  try {
    const resultado = criarResultado(prova, simuladoId, () => {
      renderizarHome(containerPai);
    });
    containerPai.appendChild(resultado);
  } catch (e) {
    console.error('Erro ao renderizar resultado:', e);
    containerPai.innerHTML = `<div class="erro">Erro ao carregar o resultado</div>`;
    setTimeout(() => renderizarHome(containerPai), 3000);
  }
}

function limparContainer(container: HTMLElement): void {
  container.replaceChildren();
}