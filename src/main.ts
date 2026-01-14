// Copyright (C) 2026 Tiago. Licenciado sob AGPL-3.0.
import { renderizarHome } from './ui/Home';

document.addEventListener('DOMContentLoaded', () => {
  const app = document.getElementById('app');
  if (app) {
    renderizarHome(app);
  }
});