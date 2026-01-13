import { renderizarHome } from './ui/Home';

document.addEventListener('DOMContentLoaded', () => {
  const app = document.getElementById('app');
  if (app) {
    renderizarHome(app);
  }
});