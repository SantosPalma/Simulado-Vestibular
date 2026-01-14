# 📘 Simulador de Provas Offline

![Tauri + TypeScript](https://img.shields.io/badge/Tauri-1.0+-5A189A?logo=rust&logoColor=white)
![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-orange)

**Vestibular Tauri** é um simulador offline de provas de vestibular (como ENEM, Fuvest, etc.) desenvolvido para estudantes que desejam treinar de forma acessível, segura e sem depender de internet. O projeto é 100% gratuito, open source e **não pode ser comercializado**.

## 🎯 Objetivo

Oferecer uma ferramenta educacional de alta qualidade para:
- Estudantes de escolas públicas ou com acesso limitado à internet
- Quem busca privacidade total (sem coleta de dados)
- Professores que desejam criar e compartilhar provas personalizadas
- Instituições sem fins lucrativos que apoiam a educação básica

## 🌍 Função Social

Este projeto nasce com um compromisso social claro: **democratizar o acesso à preparação para vestibulares**. Ao ser totalmente offline, gratuito e de código aberto, ele elimina barreiras tecnológicas e econômicas, permitindo que qualquer pessoa, em qualquer lugar do Brasil (ou do mundo), possa treinar com questões reais de forma ética e segura.

> ✨ **Educação é um direito — não um produto.**

## ⚙️ Tecnologias Utilizadas

| Camada | Tecnologia | Por quê? |
|-------|------------|--------|
| **Frontend** | TypeScript puro + DOM API | Leve, rápido, sem frameworks pesados; ideal para apps desktop simples |
| **Estilização** | CSS puro (sem frameworks) | Total controle visual, performance otimizada e fácil manutenção |
| **Backend** | Rust + Tauri | Segurança, velocidade nativa e acesso ao sistema de arquivos |
| **Banco de Dados** | SQLite | Leve, confiável e integrado ao app sem servidores externos |
| **Arquitetura** | Separation of Concerns | Código organizado em UI, estado e lógica de negócio |

### Principais recursos:
- ✅ Simulados cronometrados com pausa/retomada
- ✅ Navegação entre questões (avançar/voltar)
- ✅ Suporte a imagens nas questões
- ✅ Resultado detalhado (acertos, erros, gabarito)
- ✅ Armazenamento local seguro (sem nuvem)
- ✅ Funciona 100% offline

## 📁 Estrutura do Projeto

```
vestibular-tauri/
├── provas/              # Provas no formato JSON + assets
├── src/                 # Frontend (TypeScript + CSS)
└── src-tauri/           # Backend (Rust)
```

As provas são arquivos JSON simples com suporte a enunciados, alternativas, respostas corretas e imagens — fácil de criar e compartilhar!

## 🛠️ Como Contribuir

Contribuições são bem-vindas! Este é um projeto comunitário voltado para a educação. Você pode:
- Criar novas provas (ENEM, vestibulares regionais, etc.)
- Melhorar a interface de usuário
- Traduzir para outras línguas
- Corrigir bugs ou sugerir funcionalidades

> 💡 **Dica**: Use o VS Code com as extensões oficiais do [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) e [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## 📜 Licença

Este projeto é licenciado sob a **GNU Affero General Public License v3.0 (AGPL-3.0)**.

### Por que AGPL-3.0?
- ✅ **Proíbe uso comercial**: Ninguém pode vender este software ou derivados.
- ✅ **Exige compartilhamento de melhorias**: Qualquer modificação deve ser disponibilizada sob a mesma licença.
- ✅ **Fortalece o software livre**: Garante que o projeto permaneça aberto, ético e acessível.

> ⚠️ **Você pode usar, modificar e distribuir este software — mas nunca vendê-lo.**

[Veja o arquivo LICENSE para mais detalhes.](./LICENSE)

---

Desenvolvido com ❤️ para democratizar o acesso à educação de qualidade.  
**Não é um produto. É um ato de resistência.**
