// src/ui/SeletorProva.ts
import { listarProvas, carregarProva, Prova, iniciarSimulado } from '../state/SimuladoClient';

export type SeletorProvaCallback = (simuladoId: number, provaId: string, prova: Prova) => void;

export function criarSeletorProva(onProvaSelecionada: SeletorProvaCallback): HTMLElement {
  const container = document.createElement('div');
  container.className = 'seletor-prova';

  const titulo = document.createElement('h2');
  titulo.textContent = 'Escolha uma prova';
  container.appendChild(titulo);

  // Elemento de status (carregando/erro)
  const statusEl = document.createElement('div');
  statusEl.className = 'status';
  statusEl.textContent = 'Carregando provas...';
  container.appendChild(statusEl);

  // Lista de provas (inicialmente escondida)
  const lista = document.createElement('ul');
  lista.style.display = 'none'; // Esconde inicialmente
  container.appendChild(lista);

  const carregarProvas = async () => {
    try {
      // Mostra estado de carregamento
      statusEl.textContent = 'Carregando provas...';
      statusEl.className = 'status carregando';
      lista.style.display = 'none';

      const ids = await listarProvas();
      console.log('✅ Provas encontradas:', ids);

      if (ids.length === 0) {
        throw new Error('Nenhuma prova encontrada. Verifique a pasta "provas/"');
      }

      // Limpa e mostra a lista
      lista.innerHTML = '';
      lista.style.display = 'block';
      statusEl.style.display = 'none'; // Esconde o status

      ids.forEach(id => {
        const item = document.createElement('li');
        const botao = document.createElement('button');
        
        try {
          const partes = id.split('/');
          const vestibular = partes[0].toUpperCase();
          const ano = parseInt(partes[1].split('_')[0]);
          botao.textContent = `${vestibular} ${ano}`;
        } catch (e) {
          console.warn('Formato de ID inválido:', id, e);
          botao.textContent = id.replace('_', ' ').toUpperCase();
        }

        botao.addEventListener('click', async () => {
  let originalText = botao.textContent || '';

  try {
    botao.textContent = 'Iniciando...';
    botao.disabled = true;

    const partes = id.split('/');
    const vestibular = partes[0].toUpperCase();
    const ano = parseInt(partes[1].split('_')[0]);

    const prova = await carregarProva(id);
    const simuladoId = await iniciarSimulado(
      id,
      vestibular,
      ano,
      prova.duracao_minutos
    );

    onProvaSelecionada(simuladoId, id, prova);

  } catch (e) {
    console.error('❌ Erro ao iniciar simulado:', e);

    alert(
      'Erro ao iniciar simulado: ' +
      (typeof e === 'string' ? e : 'Erro desconhecido')
    );

    botao.textContent = originalText;
    botao.disabled = false;
  }
});

        
        item.appendChild(botao);
        lista.appendChild(item);
      });
    } catch (e) {
      console.error('❌ Erro ao carregar provas:', e);
      
      // Mostra erro visível
      statusEl.textContent = `Erro: ${typeof e === 'string' ? e : 'Falha ao carregar provas'}`;
      statusEl.className = 'status erro';
      statusEl.style.display = 'block';
      
      // Adiciona botão de recarregar
      const btnRecarregar = document.createElement('button');
      btnRecarregar.className = 'btn-recarregar';
      btnRecarregar.textContent = 'Tentar novamente';
      btnRecarregar.onclick = carregarProvas;
      
      // Remove conteúdo anterior e adiciona botão
      while (statusEl.firstChild) {
        statusEl.removeChild(statusEl.firstChild);
      }
      statusEl.appendChild(document.createTextNode(statusEl.textContent));
      statusEl.appendChild(document.createElement('br'));
      statusEl.appendChild(btnRecarregar);
    }
  };

  // Inicia o carregamento
  carregarProvas();
  
  return container;
}