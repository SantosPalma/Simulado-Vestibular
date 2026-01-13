// src/ui/Resultado.ts
import { obterResultado } from '../state/SimuladoClient';
import type { Prova, ResultadoSimulado } from '../state/SimuladoClient';

export type ResultadoCallback = () => void;

export function criarResultado(
  prova: Prova,
  simuladoId: number, // ← recebe o ID do simulado
  onVoltar: ResultadoCallback
): HTMLElement {
  const container = document.createElement('div');
  container.className = 'resultado';

  // Carrega o resultado do backend
  obterResultado(simuladoId)
    .then(resultado => {
      const pontuacao = resultado.pontuacao.toFixed(1);
      
      container.innerHTML = `
        <h2>Resultados</h2>
        <p><strong>Vestibular:</strong> ${prova.vestibular} ${prova.ano}</p>
        <p><strong>Acertos:</strong> ${resultado.acertos} de ${resultado.total_questoes}</p>
        <p><strong>Pontuação:</strong> ${pontuacao}%</p>
      `;

      // Mostra detalhes das questões
      const erradas = resultado.detalhes.filter(d => !d.acertou);
      if (erradas.length > 0) {
        const detalhes = document.createElement('div');
        detalhes.className = 'detalhes';
        detalhes.innerHTML = '<h3>Questões erradas:</h3><ul></ul>';
        const ul = detalhes.querySelector('ul')!;
        
        erradas.forEach(d => {
          const li = document.createElement('li');
          li.textContent = `Q${d.numero}: sua resposta = ${d.resposta_usuario || '—'}, gabarito = ${d.gabarito}`;
          ul.appendChild(li);
        });
        container.appendChild(detalhes);
      }
    })
    .catch(e => {
      console.error('Erro ao carregar resultado:', e);
      container.innerHTML = `<p>Erro ao carregar resultado: ${e}</p>`;
    });

  const btnVoltar = document.createElement('button');
  btnVoltar.textContent = 'Voltar para o início';
  btnVoltar.addEventListener('click', onVoltar);
  container.appendChild(btnVoltar);

  return container;
}