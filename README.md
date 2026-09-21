# Chat P2P com libp2p (Rust)

Aplicação de chat ponto a ponto (P2P) desenvolvida em Rust utilizando a biblioteca libp2p como middleware de comunicação distribuída. A descoberta de participantes é feita via mDNS e a troca de mensagens via gossipsub (protocolo de publish/subscribe).

## Autores

- Amanda de Oliveira Pereira
- Erik de Oliveira Pádua
- Kaio Leandro Garcia Silvestrini

Trabalho desenvolvido para a disciplina de Sistemas Distribuídos.

## Aviso importante

Este projeto só funciona entre dispositivos conectados na mesma rede local (mesmo Wi-Fi ou mesma LAN). A descoberta automática de participantes usa mDNS, um protocolo baseado em multicast que não atravessa redes diferentes nem a internet em geral. Se os participantes estiverem em redes distintas, o programa não encontra o outro lado automaticamente.

## Sobre o projeto

Cada execução do programa (`cargo run`) é um nó independente do chat.

Ao iniciar, o nó escuta conexões e anuncia sua presença na rede local via mDNS. Quando outro nó é descoberto, os dois se conectam automaticamente e passam a trocar mensagens através de um tópico compartilhado no gossipsub. Toda mensagem enviada é assinada, tem timestamp e é exibida com o nome escolhido pelo usuário.

Eventos de conexão, desconexão e mensagens trocadas são registrados em um arquivo de log (`chat.log`), na mesma pasta do projeto.

## Pré-requisitos

É necessário ter a toolchain do Rust instalada (compilador `rustc` e gerenciador de pacotes `cargo`). O passo a passo abaixo cobre Windows e Linux separadamente.

### Windows

1. Instale o Rust via rustup, disponível em https://rustup.rs (ou diretamente pelo instalador https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe). Durante a instalação, mantenha as opções padrão (toolchain stable, target MSVC).

2. Instale as ferramentas de build do C++. O Rust no Windows depende do linker do Visual Studio. Se o instalador do rustup avisar que elas estão faltando, aceite instalar — ou instale manualmente o Build Tools for Visual Studio, marcando o workload "Desktop development with C++".

3. Verifique a instalação abrindo um novo terminal (PowerShell ou CMD) e rodando:
   ```powershell
   rustc --version
   cargo --version
   ```
   Se aparecerem números de versão, está tudo certo.

4. Opcionalmente, instale o VSCode com a extensão rust-analyzer, que facilita o desenvolvimento (autocomplete, checagem de erros em tempo real).

5. Na primeira execução do programa, o Windows pode exibir um aviso do Firewall pedindo permissão de rede. É necessário permitir o acesso em redes privadas, senão a conexão com outros participantes não funciona.

### Linux

1. Instale as ferramentas de build básicas. Em distribuições baseadas em Debian/Ubuntu:
   ```bash
   sudo apt update
   sudo apt install -y build-essential pkg-config
   ```
   Em distribuições baseadas em Fedora:
   ```bash
   sudo dnf groupinstall -y "Development Tools"
   ```

2. Instale o Rust via rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Siga as instruções na tela (a opção padrão de instalação é suficiente).

3. Carregue o ambiente do Rust na sessão atual:
   ```bash
   source "$HOME/.cargo/env"
   ```
   Em novas sessões de terminal isso já é feito automaticamente.

4. Verifique a instalação:
   ```bash
   rustc --version
   cargo --version
   ```

## Clonando o repositório

```bash
git clone <URL-do-repositório>
cd <pasta-do-repositório>
```

## Executando o projeto

```bash
cargo run
```

Na primeira execução, o cargo baixa e compila todas as dependências, o que pode levar alguns minutos. Nas execuções seguintes, a compilação é incremental e bem mais rápida.

Ao rodar, o programa pede um nome:

```
Digite seu nome para entrar no chat:
```

Depois disso, ele fica escutando conexões e mensagens. Repita esse processo em outro computador conectado à mesma rede local para iniciar uma conversa entre os dois.

## Usando o chat

Digite uma mensagem e pressione Enter para enviá-la a todos os participantes conectados. Mensagens recebidas de outros participantes aparecem automaticamente no terminal, junto com o nome de quem enviou e o horário. Para encerrar, use Ctrl+C.

## Log de eventos

Todas as execuções gravam eventos em um arquivo `chat.log`, criado na pasta do projeto. Nele ficam registrados, com data e hora: início de um novo chat, mensagens enviadas e recebidas, e conexão/desconexão de participantes.

Se dois participantes rodarem o programa a partir da mesma pasta (por exemplo, testando na mesma máquina), o arquivo `chat.log` é compartilhado entre os processos, misturando os eventos dos dois lados — isso é esperado nesse cenário e não indica erro.

## Dependências (crates) utilizadas

| Crate | Finalidade |
|---|---|
| libp2p | Middleware de rede P2P (transporte, criptografia, descoberta, pub/sub) |
| tokio | Runtime assíncrono |
| futures | Utilitários para streams assíncronas |
| chrono | Geração de timestamps |

## Estrutura do projeto

```
.
├── Cargo.toml      # Dependências do projeto
├── src/
│   └── main.rs     # Código-fonte do chat
└── chat.log        # Gerado automaticamente na execução
```
