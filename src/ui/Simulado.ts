// src/ui/Simulado.ts
import {
  obterEstadoSimulado,
  responderQuestao,
  avancarQuestao,
  voltarQuestao,
  pausarSimulado,
  retomarSimulado,
  finalizarSimulado,
  atualizarTempoSimulado // ← Certifique-se de que isso existe em SimuladoClient.ts
} from '../state/SimuladoClient';

import type {
  Prova,
  Questao,
  EstadoSimuladoCompleto
} from '../state/SimuladoClient';

export type SimuladoCallback = (prova: Prova) => void;

export function criarSimulado(
  simuladoId: number,
  provaId: string,
  prova: Prova,
  onFinalizar: SimuladoCallback
): HTMLElement {
  const container = document.createElement('div');
  container.className = 'simulado';

  /* ============================
     ELEMENTOS DA UI
     ============================ */
  const cabecalho = document.createElement('header');
  cabecalho.className = 'simulado-cabecalho';

  const questaoEl = document.createElement('section');
  questaoEl.className = 'simulado-questao';

  const navegacao = document.createElement('footer');
  navegacao.className = 'simulado-navegacao';

  const btnAnterior = document.createElement('button');
  btnAnterior.textContent = 'Anterior';
  btnAnterior.disabled = true;

  const btnAvancar = document.createElement('button');
  btnAvancar.textContent = 'Próxima';

  const btnPausar = document.createElement('button');
  btnPausar.textContent = 'Pausar';

  const btnFinalizar = document.createElement('button');
  btnFinalizar.textContent = 'Finalizar';

  navegacao.append(btnAnterior, btnAvancar, btnPausar, btnFinalizar);
  container.append(cabecalho, questaoEl, navegacao);

  /* ============================
     ESTADO LOCAL
     ============================ */
  let estadoAtual: EstadoSimuladoCompleto | null = null;
  let intervalId: number | null = null;

  /* ============================
     FUNÇÕES AUXILIARES
     ============================ */
  const formatarTempo = (segundos: number): string => {
    const mins = Math.floor(segundos / 60);
    const secs = segundos % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const atualizarCabecalho = () => {
    if (!estadoAtual) return;
    
    cabecalho.innerHTML = `
      <span>${prova.vestibular} • ${prova.ano}</span>
      <span>Tempo: ${formatarTempo(estadoAtual.tempo.decorrido_segundos)}</span>
    `;
  };

  const renderizarQuestao = (estado: EstadoSimuladoCompleto) => {
    const questaoId = estado.progresso.questao_atual;
    const questao = prova.questoes.find(q => q.id === questaoId);
    if (!questao) return;

    questaoEl.innerHTML = '';

    const titulo = document.createElement('h3');
    titulo.textContent = `Questão ${questao.numero}`;
    questaoEl.appendChild(titulo);

    const enunciado = document.createElement('p');
    enunciado.innerHTML = questao.enunciado;
    questaoEl.appendChild(enunciado);

    /* Imagens */
    questao.imagens.forEach(img => {
      const imgEl = document.createElement('img');
      // Correção: o caminho deve ser relativo à pasta da prova
      imgEl.src = `../provas/${provaId}/assets/${img}`;
      imgEl.alt = `Imagem da questão ${questao.numero}`;
      imgEl.className = 'imagem-questao';
      questaoEl.appendChild(imgEl);
    });

    /* Alternativas */
    const alternativas = document.createElement('div');
    alternativas.className = 'alternativas';

    questao.alternativas.forEach(alt => {
      const btn = document.createElement('button');
      btn.className = 'alternativa';

      if (estado.respostas[questaoId] === alt.id) {
        btn.classList.add('selecionada');
      }

      const idSpan = document.createElement('strong');
      idSpan.textContent = `${alt.id}) `;
      idSpan.style.marginRight = '8px';
      
      const textSpan = document.createElement('span');
      textSpan.textContent = alt.texto;
      
      btn.appendChild(idSpan);
      btn.appendChild(textSpan);

      btn.onclick = async () => {
        try {
          await responderQuestao(simuladoId, questaoId, alt.id);
          await carregarEstado();
        } catch (e) {
          console.error('Erro ao registrar resposta:', e);
          alert('Erro ao registrar resposta: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
        }
      };

      alternativas.appendChild(btn);
    });

    questaoEl.appendChild(alternativas);
  };

  const podeAvancar = (estado: EstadoSimuladoCompleto): boolean => {
    const questaoId = estado.progresso.questao_atual;
    const indexAtual = prova.questoes.findIndex(q => q.id === questaoId);
    return indexAtual < prova.questoes.length - 1;
  };

  const questaoRespondida = (estado: EstadoSimuladoCompleto): boolean => {
    const questaoId = estado.progresso.questao_atual;
    return estado.respostas[questaoId] !== undefined && 
           estado.respostas[questaoId] !== null && 
           estado.respostas[questaoId] !== '';
  };

  // Timer que atualiza o tempo a cada segundo
  const iniciarTimer = () => {
    if (intervalId) return;
    
    intervalId = window.setInterval(async () => {
      try {
        if (!estadoAtual) return;
        
        // Atualiza o tempo no backend
        await atualizarTempoSimulado(simuladoId);
        
        // Busca o estado atualizado
        const estado = await obterEstadoSimulado(simuladoId);
        estadoAtual = estado;
        atualizarCabecalho();
        
        // ✅ CORREÇÃO: Usa o limite do estado, não da prova
        const limiteSegundos = (estado.tempo.limite_minutos || 0) * 60;
        if (limiteSegundos > 0 && estado.tempo.decorrido_segundos >= limiteSegundos) {
          finalizar();
        }
      } catch (e) {
        console.error('Erro ao atualizar tempo:', e);
      }
    }, 1000);
  };

  const pararTimer = () => {
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  };

  const finalizar = async () => {
    pararTimer();
    try {
      await finalizarSimulado(simuladoId);
      onFinalizar(prova);
    } catch (e) {
      console.error('Erro ao finalizar simulado:', e);
      alert('Erro ao finalizar: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
    }
  };

  const atualizarNavegacao = (estado: EstadoSimuladoCompleto) => {
    const questaoId = estado.progresso.questao_atual;
    const indexAtual = prova.questoes.findIndex(q => q.id === questaoId);

    btnAnterior.disabled = indexAtual <= 0;
    btnAvancar.disabled = !(podeAvancar(estado) && questaoRespondida(estado));

    btnAnterior.onclick = async () => {
      try {
        await voltarQuestao(simuladoId);
        await carregarEstado();
      } catch (e) {
        console.error('Erro ao voltar questão:', e);
        alert('Erro ao voltar: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
      }
    };

    btnAvancar.onclick = async () => {
      try {
        await avancarQuestao(simuladoId);
        await carregarEstado();
      } catch (e) {
        console.error('Erro ao avançar questão:', e);
        alert('Erro ao avançar: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
      }
    };

    if (estado.estado === 'PAUSADO') {
      btnPausar.textContent = 'Retomar';
      pararTimer();
    } else {
      btnPausar.textContent = 'Pausar';
      iniciarTimer();
    }
    
    btnPausar.onclick = async () => {
      try {
        if (estado.estado === 'PAUSADO') {
          await retomarSimulado(simuladoId);
        } else {
          await pausarSimulado(simuladoId);
        }
        await carregarEstado();
      } catch (e) {
        console.error('Erro ao pausar/retomar:', e);
        alert('Erro na operação: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
      }
    };
  };

  btnFinalizar.onclick = finalizar;

  const carregarEstado = async () => {
  try {
    // ✅ PRIMEIRO: Atualiza o tempo no backend
    await atualizarTempoSimulado(simuladoId);
    
    // ✅ DEPOIS: Carrega o estado atualizado
    const estado = await obterEstadoSimulado(simuladoId);
    estadoAtual = estado;
    
    console.log('📊 Estado carregado:', {
      tempo: estado.tempo.decorrido_segundos,
      inicio: estado.tempo.inicio,
      estado: estado.estado
    });
    
    // Inicia ou para o timer baseado no estado
    if (estado.estado === 'EM_ANDAMENTO') {
      iniciarTimer();
    } else {
      pararTimer();
    }

    atualizarCabecalho();
    renderizarQuestao(estado);
    atualizarNavegacao(estado);
  } catch (e) {
    console.error('❌ Erro ao carregar estado:', e);
    alert('Erro ao carregar simulado: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
    
    // Tenta novamente após 2 segundos
    setTimeout(carregarEstado, 2000);
  }
};

  // ✅ Limpeza quando o componente for removido
  const observer = new MutationObserver(() => {
    if (!container.isConnected) {
      pararTimer();
      observer.disconnect();
    }
  });

  observer.observe(document.body, { childList: true, subtree: true });

  // Carrega o estado inicial
  carregarEstado();
  
  return container;
}