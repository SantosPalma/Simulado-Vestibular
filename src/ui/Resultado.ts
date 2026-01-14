// src/ui/Resultado.ts
import { obterResultado } from '../state/SimuladoClient';
import type { Prova, ResultadoSimulado, DetalheQuestao } from '../state/SimuladoClient';

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
      
      // Separa questões corretas e erradas
      const certas = resultado.detalhes.filter(d => d.acertou);
      const erradas = resultado.detalhes.filter(d => !d.acertou);
      
      // Constrói o conteúdo
      let html = `
        <h2>Resultados do Simulado</h2>
        <div class="resumo-resultado">
          <div class="cartao acertos">
            <h3>Acertos</h3>
            <p class="numero-grande">${resultado.acertos}</p>
            <p>de ${resultado.total_questoes}</p>
          </div>
          <div class="cartao pontuacao">
            <h3>Pontuação</h3>
            <p class="numero-grande">${pontuacao}%</p>
            <p>${resultado.erros} erro(s)</p>
          </div>
        </div>
        
        <div class="info-prova">
          <p><strong>Vestibular:</strong> ${prova.vestibular} ${prova.ano}</p>
          <p><strong>Questões respondidas:</strong> ${resultado.acertos + resultado.erros} de ${resultado.total_questoes}</p>
        </div>
      `;

      // Mostra detalhes das questões corretas
      if (certas.length > 0) {
        html += `
          <div class="detalhes grupo-certas">
            <h3>Questões Corretas (${certas.length})</h3>
            <ul>
        `;
        certas.forEach(d => {
          html += `
            <li class="certa">
              <strong>Q${d.numero}</strong>: ${d.gabarito} 
              <span class="mini-badge">✓</span>
            </li>
          `;
        });
        html += '</ul></div>';
      }

      // Mostra detalhes das questões erradas
      if (erradas.length > 0) {
        html += `
          <div class="detalhes grupo-erradas">
            <h3>❌ Questões Erradas (${erradas.length})</h3>
            <ul>
        `;
        erradas.forEach(d => {
          html += `
            <li class="errada">
              <strong>Q${d.numero}</strong>: 
              sua resposta = <span class="sua-resposta">${d.resposta_usuario || '—'}</span>, 
              gabarito = <span class="gabarito">${d.gabarito}</span>
            </li>
          `;
        });
        html += '</ul></div>';
      }

      // Se não houver detalhes (todas corretas ou todas erradas)
      if (certas.length === 0 && erradas.length === 0) {
        html += '<p class="sem-detalhes">Nenhuma questão respondida para exibir detalhes.</p>';
      }

      // Adiciona o botão "Voltar"
      html += `
        <div class="botoes-resultado">
          <button class="btn-voltar">voltar para o início</button>
        </div>
      `;
      
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
        <div class="erro-container">
          <h2>Erro ao carregar resultado</h2>
          <p>${typeof e === 'string' ? e : 'Ocorreu um erro desconhecido'}</p>
          <button class="btn-voltar">Tentar novamente</button>
        </div>
      `;
      const btnVoltar = container.querySelector('.btn-voltar');
      if (btnVoltar) {
        btnVoltar.addEventListener('click', () => {
          criarResultado(prova, simuladoId, onVoltar);
        });
      }
    });

  return container;
}