


🚀 Universal Multi-Segment System - Estágio 01: Vendas e Autenticação
Este projeto é um sistema multisegmentos desenvolvido em Rust. A arquitetura foi projetada para ser flexível, permitindo que o sistema funcione inicialmente como uma plataforma de vendas de produtos e, futuramente, seja expandido para o segmento de aluguel de equipamentos e ferramentas.


_______________________________________________________________________________________________________

ESTE È UM PROJETO BASE NA QUALIDADE DE INICIATE NA LINGUAGEM RUST: TENHO MUITO OUTROS PROJETOS PARA SER DESENVOLVIDO:

PRETENDO SER DESENVOVEDOR NA LIGUAGEM RUST > # beckend - microserviços - web3 e em sistema de segurança de software:
 
 -- disponivel para trabalho remoto: estou  desenvolvendo um algorimos para um jogo!! empresa que quer investir aberto a proposta!!

 -- predendo desenvolver um sistema na area de saude: estou projetando a logica do sistema - depois vou criar um mvp ou spa:


 ________________________________________________________________________________________________________




🎯 Visão do Projeto
O objetivo é criar um núcleo (core) robusto que suporte diferentes modelos de negócio no mesmo ecossistema, utilizando a performance e segurança que o ecossistema Rust oferece.

🛠️ Tecnologias e Ambiente
Linguagem: Rust (Backend de alta performance).

Web Framework: Actix-Web (Assíncrono e escalável).

Template Engine: Tera (Renderização dinâmica de HTML).

Ambiente: Windows com PowerShell e VS Code.

📋 Funcionalidades do Primeiro Estágio
Vitrine Dinâmica: Carregamento de produtos via JSON, simulando um catálogo de vendas.

Fluxo de Autenticação: Login validado com redirecionamento automático para o checkout.

Cadastro Multi-Campos: Captura de dados de entrega (CEP, Endereço) integrada ao perfil do usuário.

Módulo de Checkout: Estrutura preparada para receber múltiplos itens e calcular totais.

Gerenciamento de Rotas: Endpoints para adicionar e remover itens do carrinho já configurados.

Estrutura de Pastas
Plaintext
├── static/          # CSS, imagens e scripts front-end
├── templates/       # HTMLs dinâmicos (login, vitrine, checkout)
├── src/
│   └── main.rs      # Lógica central, handlers e roteamento
├── produtos.json    # Banco de dados temporário de produtos
└── Cargo.toml       # Gerenciador de dependências e features (Serde/Derive)

codigo foi enviado para git
