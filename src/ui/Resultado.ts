// src/ui/Resultado.ts
import { obterResultado } from '../state/SimuladoClient';
import type { Prova, ResultadoSimulado } from '../state/SimuladoClient';

export type ResultadoCallback = () => void;

export function criarResultado(
  prova: Prova,
  simuladoId: number,
  onVoltar: ResultadoCallback
): HTMLElement {
  const container = document.createElement('div');
  container.className = 'resultado';

  // Carrega o resultado do backend
  obterResultado(simuladoId)
    .then(resultado => {
      const pontuacao = resultado.pontuacao.toFixed(1);
      
      // Constrói todo o conteúdo incluindo o botão
      let html = `
        <h2>Resultados</h2>
        <p><strong>Vestibular:</strong> ${prova.vestibular} ${prova.ano}</p>
        <p><strong>Acertos:</strong> ${resultado.acertos} de ${resultado.total_questoes}</p>
        <p><strong>Pontuação:</strong> ${pontuacao}%</p>
      `;

      // Mostra detalhes das questões
      const erradas = resultado.detalhes.filter(d => !d.acertou);
      if (erradas.length > 0) {
        html += '<div class="detalhes"><h3>Questões erradas:</h3><ul>';
        erradas.forEach(d => {
          html += `<li>Q${d.numero}: sua resposta = ${d.resposta_usuario || '—'}, gabarito = ${d.gabarito}</li>`;
        });
        html += '</ul></div>';
      }

      // Adiciona o botão "Voltar"
      html += '<button class="btn-voltar">Voltar para o início</button>';
      
      container.innerHTML = html;
      
      // Adiciona o evento de clique ao botão
      const btnVoltar = container.querySelector('.btn-voltar');
      if (btnVoltar) {
        btnVoltar.addEventListener('click', onVoltar);
      }
    })
    .catch(e => {
      console.error('Erro ao carregar resultado:', e);
      container.innerHTML = `
        <p>Erro ao carregar resultado: ${e}</p>
        <button class="btn-voltar">Voltar para o início</button>
      `;
      const btnVoltar = container.querySelector('.btn-voltar');
      if (btnVoltar) {
        btnVoltar.addEventListener('click', onVoltar);
      }
    });

  return container;
}