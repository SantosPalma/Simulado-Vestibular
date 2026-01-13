// src/ui/Simulado.ts
import {
  obterEstadoSimulado,
  responderQuestao,
  avancarQuestao,
  voltarQuestao,
  pausarSimulado,
  retomarSimulado,
  finalizarSimulado
} from '../state/SimuladoClient';

import type {
  Prova,
  Questao,
  EstadoSimuladoCompleto
} from '../state/SimuladoClient';

// Callback simplificado - só precisa da prova
export type SimuladoCallback = (prova: Prova) => void;
// ... imports ...

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

  const atualizarCabecalho = (estado: EstadoSimuladoCompleto) => {
    cabecalho.innerHTML = `
      <span>${prova.vestibular} • ${prova.ano}</span>
      <span>Tempo: ${formatarTempo(estado.tempo.decorrido_segundos)}</span>
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

      btn.innerHTML = `<strong>${alt.id})</strong> ${alt.texto}`;

      btn.onclick = async () => {
        // Registra resposta no backend
        await responderQuestao(simuladoId, questaoId, alt.id);
        
        // Carrega estado atualizado
        await carregarEstado();
      };

      alternativas.appendChild(btn);
    });

    questaoEl.appendChild(alternativas);
  };

  // Função para verificar se pode avançar
  const podeAvancar = (estado: EstadoSimuladoCompleto): boolean => {
    const questaoId = estado.progresso.questao_atual;
    const indexAtual = prova.questoes.findIndex(q => q.id === questaoId);
    const totalQuestoes = prova.questoes.length;
    
    // Pode avançar se NÃO for a última questão
    return indexAtual < totalQuestoes - 1;
  };

  // Função para verificar se a questão atual foi respondida
  const questaoRespondida = (estado: EstadoSimuladoCompleto): boolean => {
    const questaoId = estado.progresso.questao_atual;
    return estado.respostas[questaoId] !== undefined && estado.respostas[questaoId] !== null;
  };

  const atualizarNavegacao = (estado: EstadoSimuladoCompleto) => {
    const questaoId = estado.progresso.questao_atual;
    const indexAtual = prova.questoes.findIndex(q => q.id === questaoId);
    const totalQuestoes = prova.questoes.length;

    // Botão Anterior
    btnAnterior.disabled = indexAtual <= 0;

    // Botão Próxima - habilita se:
    // 1. Não é a última questão E
    // 2. A questão atual foi respondida
    const podeAvancarAgora = podeAvancar(estado) && questaoRespondida(estado);
    btnAvancar.disabled = !podeAvancarAgora;

    // Eventos de navegação
    btnAnterior.onclick = async () => {
      await voltarQuestao(simuladoId);
      await carregarEstado();
    };

    btnAvancar.onclick = async () => {
      await avancarQuestao(simuladoId);
      await carregarEstado();
    };

    // Lógica de pausar/retomar
    if (estado.estado === 'PAUSADO') {
      btnPausar.textContent = 'Retomar';
    } else {
      btnPausar.textContent = 'Pausar';
    }
    
    btnPausar.onclick = async () => {
      if (estado.estado === 'PAUSADO') {
        await retomarSimulado(simuladoId);
      } else {
        await pausarSimulado(simuladoId);
      }
      await carregarEstado();
    };
  };

  const finalizar = async () => {
    await finalizarSimulado(simuladoId);
    onFinalizar(prova);
  };

  btnFinalizar.onclick = finalizar;

  /* ============================
     CARREGAMENTO DE ESTADO
     ============================ */
   const carregarEstado = async () => {
    try {
      const estado = await obterEstadoSimulado(simuladoId);
      console.log('📊 Estado atual:', {
        questaoAtual: estado.progresso.questao_atual,
        respostas: estado.respostas,
        podeAvancar: podeAvancar(estado),
        questaoRespondida: questaoRespondida(estado)
      });
      
      estadoAtual = estado;
      atualizarCabecalho(estado);
      renderizarQuestao(estado);
      atualizarNavegacao(estado);

      if (intervalId) clearInterval(intervalId);
      if (estado.estado === 'EM_ANDAMENTO') {
        intervalId = window.setInterval(() => {
          estadoAtual!.tempo.decorrido_segundos += 1;
          atualizarCabecalho(estadoAtual!);
        }, 1000);
      }
    } catch (e) {
      console.error('❌ Erro ao carregar estado:', e);
      alert('Erro ao carregar simulado: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
    }
  };

  carregarEstado();
  return container;
}