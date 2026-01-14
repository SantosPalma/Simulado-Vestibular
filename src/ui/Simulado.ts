// src/ui/Simulado.ts
import {
  obterEstadoSimulado,
  responderQuestao,
  avancarQuestao,
  voltarQuestao,
  pausarSimulado,
  retomarSimulado,
  finalizarSimulado,
  atualizarTempoSimulado
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
    console.log('🔍 renderizarQuestao() chamada - estado:', estado);
    
    const questaoId = estado.progresso.questao_atual;
    console.log('🔍 Questão atual ID:', questaoId);
    
    const questao = prova.questoes.find(q => q.id === questaoId);
    if (!questao) {
      console.error('❌ Questão não encontrada para ID:', questaoId);
      questaoEl.innerHTML = '<p>Erro: Questão não encontrada</p>';
      return;
    }

    console.log('🔍 Questão encontrada:', {
      id: questao.id,
      numero: questao.numero,
      imagens: questao.imagens
    });

    questaoEl.innerHTML = '';

    const titulo = document.createElement('h3');
    titulo.textContent = `Questão ${questao.numero}`;
    questaoEl.appendChild(titulo);

    const enunciado = document.createElement('p');
    enunciado.innerHTML = questao.enunciado;
    questaoEl.appendChild(enunciado);

    /* Imagens */
    if (questao.imagens && questao.imagens.length > 0) {
      console.log('🔍 Processando', questao.imagens.length, 'imagem(s)');
      
      questao.imagens.forEach((img, index) => {
        console.log(`🔍 Processando imagem ${index + 1}:`, img);
        
        const imgEl = document.createElement('img');
        
        // Constrói o caminho correto
        const partes = provaId.split('/');
        if (partes.length < 2) {
          console.error('❌ Formato de provaId inválido:', provaId);
          return;
        }
        
        const vestibular = partes[0];
        const nomeArquivo = partes.slice(1).join('/'); // Suporta subpastas
        const caminhoImagem = `/provas/${vestibular}/assets/${img}`;
        
        console.log('🔍 Caminho da imagem construído:', caminhoImagem);
        
        imgEl.src = caminhoImagem;
        imgEl.alt = `Imagem ${index + 1} da questão ${questao.numero}`;
        imgEl.className = 'imagem-questao';
        imgEl.loading = 'lazy';
        
        // Tratamento de erro
        imgEl.onerror = () => {
          console.error('❌ Falha ao carregar imagem:', {
            url: imgEl.src,
            imagem: img,
            questao: questao.numero,
            dica: `Verifique se o arquivo existe em: provas/${vestibular}/assets/${img}`
          });
          
          // Placeholder visual
          imgEl.style.backgroundColor = '#f8f9fa';
          imgEl.style.border = '2px dashed #dee2e6';
          imgEl.style.padding = '40px';
          imgEl.style.display = 'block';
          imgEl.style.margin = '16px 0';
          imgEl.alt = `⚠️ Imagem ${index + 1} não carregada`;
          imgEl.textContent = `Imagem não disponível: ${img}`;
        };
        
        // Log de sucesso
        imgEl.onload = () => {
          console.log('✅ Imagem carregada com sucesso:', imgEl.src);
        };
        
        questaoEl.appendChild(imgEl);
      });
    } else {
      console.log('ℹ️ Nenhuma imagem definida para esta questão');
    }

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
        
        await atualizarTempoSimulado(simuladoId);
        const estado = await obterEstadoSimulado(simuladoId);
        estadoAtual = estado;
        atualizarCabecalho();
        
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
      console.log('🔍 Iniciando carregamento do estado...');
      
      await atualizarTempoSimulado(simuladoId);
      const estado = await obterEstadoSimulado(simuladoId);
      estadoAtual = estado;
      
      console.log('📊 Estado carregado com sucesso:', {
        tempo: estado.tempo.decorrido_segundos,
        inicio: estado.tempo.inicio,
        estado: estado.estado,
        questaoAtual: estado.progresso.questao_atual,
        totalQuestoes: prova.questoes.length
      });
      
      if (estado.estado === 'EM_ANDAMENTO') {
        iniciarTimer();
      } else {
        pararTimer();
      }

      atualizarCabecalho();
      renderizarQuestao(estado); // ← Esta linha deve gerar logs
      atualizarNavegacao(estado);
    } catch (e) {
      console.error('❌ Erro ao carregar estado:', e);
      alert('Erro ao carregar simulado: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
      setTimeout(carregarEstado, 2000);
    }
  };

  // Limpeza quando o componente for removido
  const observer = new MutationObserver(() => {
    if (!container.isConnected) {
      pararTimer();
      observer.disconnect();
    }
  });

  observer.observe(document.body, { childList: true, subtree: true });

  // Carrega o estado inicial
  console.log('🔍 Iniciando simulado com ID:', simuladoId, 'Prova ID:', provaId);
  carregarEstado();
  
  return container;
}