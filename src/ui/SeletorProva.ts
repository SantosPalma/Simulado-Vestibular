// src/ui/SeletorProva.ts
import { listarProvas, carregarProva, Prova, iniciarSimulado } from '../state/SimuladoClient';

// Callback deve receber (simuladoId, provaId, prova)
export type SeletorProvaCallback = (simuladoId: number, provaId: string, prova: Prova) => void;

export function criarSeletorProva(onProvaSelecionada: SeletorProvaCallback): HTMLElement {
  const container = document.createElement('div');
  container.className = 'seletor-prova';

  const titulo = document.createElement('h2');
  titulo.textContent = 'Escolha uma prova';
  container.appendChild(titulo);

  const lista = document.createElement('ul');
  container.appendChild(lista);

  const carregarProvas = async () => {
    try {
      const ids = await listarProvas();
      lista.innerHTML = '';

      ids.forEach(id => {
        const item = document.createElement('li');
        const botao = document.createElement('button');
        const partes = id.split('/');
        const vestibular = partes[0].toUpperCase();
        const ano = parseInt(partes[1].split('_')[0]);
        
        botao.textContent = `${vestibular} ${ano}`;
        botao.addEventListener('click', async () => {
          try {
            const prova = await carregarProva(id);
            const simuladoId = await iniciarSimulado(id, vestibular, ano, prova.duracao_minutos);
            onProvaSelecionada(simuladoId, id, prova); // ← passa os 3 parâmetros
          } catch (e) {
            alert('Erro ao iniciar simulado: ' + (typeof e === 'string' ? e : 'Erro desconhecido'));
          }
        });
        item.appendChild(botao);
        lista.appendChild(item);
      });
    } catch (e) {
      const erro = document.createElement('div');
      erro.className = 'erro';
      erro.textContent = 'Falha ao carregar provas: ' + (typeof e === 'string' ? e : 'Erro desconhecido');
      container.appendChild(erro);
    }
  };

  carregarProvas();
  return container;
}