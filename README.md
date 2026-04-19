# Majestic-World Server Manager

Gerenciador de console para servidores Lineage 2 com desligamento seguro e atualizações automáticas, agora reescrito em Rust para máxima performance e uso direto da API do Windows.

## Recursos

- Desligamento seguro via `Ctrl+C` com injeção de eventos via Windows API
- Atualização rápida e compacta dos arquivos `.jar`
- Logs formatados e coloridos no console nativo (erros, avisos, shutdown)
- Reinício automático seguro em caso de necessidade
- Interface enxuta (com atalhos ocultos para funções extras)

## Requisitos

- Rust e Cargo (apenas para compilação)
- Java 17+
- Windows

## Uso

Execute o binário `GameServer.exe`. O menu principal oferece as opções:

1. Start Server
2. Start Server (with updates)

A interface exibirá apenas as opções primárias de listagem. _Atalhos ocultos (3, 4 e 5)_ continuam existindo de forma secreta para executar tarefas individuais como update isolado, shutdown manual e fechamento.

Pressione `Ctrl+C` a qualquer momento em que o servidor estiver rodando para enviar o sinal de desligamento seguro.

## Configuração

O aplicativo buscará um arquivo `GameServer.properties` no mesmo diretório do executável. O padrão de propriedades é o seguinte:

```properties
ServerPath="C:\Users\Mk\Desktop\MyServer\gameserver"
ServerCopyPath="libs" #Optional
JavaPath="C:\Users\Mk\Documents\Java\jdk-25.0.1\bin"
JavaArgs="-server -Dfile.encoding=UTF-8 -Xmx8G -cp config;./libs/* l2.gameserver.GameServer"
OutputJarPath="C:\workspace\java\Majestic-Pack\build\artifacts"
```

## Build

Para compilar uma versão nativa e super otimizada:

```bash
cargo build --release
```

Após o build, o tamanho final ficará super enxuto e disponível sob `target/release/GameServer.exe`.
