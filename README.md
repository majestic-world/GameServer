# Majestic-World Server Manager

Uma aplicação de console simples e poderosa para gerenciar seus servidores de Lineage 2, com desligamentos seguros e atualizações automáticas.

## ✨ Recursos

* **🛡️ Desligamento Seguro:** Envia um sinal `Ctrl+C` ao processo Java, permitindo um desligamento gracioso que salva todos os dados do jogo.
* **🔄 Atualização Automática:** Copia automaticamente todos os arquivos `.jar` mais recentes do seu diretório de build para a pasta `libs` do servidor.
* **🎨 Logs Coloridos:** Identifique problemas facilmente com erros em vermelho, avisos em amarelo e mensagens de desligamento em ciano.
* **🔁 Reinício Automático:** Reinicia automaticamente o servidor se ele travar (detecta o código de saída 2).

-----

## 📋 Requisitos

* .NET 9.0
* Java 17+
* Windows OS

-----

## ⚙️ Configuração

Antes de executar, você **deve** editar os seguintes caminhos no `Program.cs`:

```csharp
// Caminho para o diretório do seu game server
private const string ServerPath = @"C:\\Users\\Dev\\Desktop\\MyServer\\gameserver";

// Caminho para seus arquivos .jar compilados (saída do build)
private const string OutputJarPath = @"C:\\Recreate\\Lucera\\out\\jar";
```

-----

## 🚀 Uso

### Opções do Menu

1.  **Iniciar Servidor:** Inicia sem verificar atualizações.
2.  **Iniciar Servidor (com atualizações):** Atualiza os JARs e, em seguida, inicia.
3.  **Atualizar JARs apenas:** Copia todos os arquivos `.jar` para o servidor.
4.  **Parar Servidor:** Executa um desligamento seguro.
5.  **Sair:** Fecha o gerenciador.

### Atalhos

> **Importante:** Enquanto o gerenciador estiver em execução, pressione **`Ctrl+C`** a qualquer momento para acionar um **desligamento seguro**.

-----

## 🛠️ Como Funciona

### Processo de Atualização

A lógica de atualização copia todos os arquivos `.jar` do seu caminho de saída especificado para o diretório `libs` do servidor.

* **Origem:** `C:\Recreate\Lucera\out\jar\*.jar`
* **Destino:** `C:\Users\Dev\Desktop\MyServer\gameserver\libs\`

### Desligamento Seguro

1.  Um sinal `Ctrl+C` (SIGINT) é enviado ao processo `java.exe`.
2.  O servidor do jogo (programado para lidar com isso) captura o sinal, salva todos os dados e desconecta os jogadores.
3.  O gerenciador aguarda até 30 segundos para que o processo seja encerrado.
4.  Se o processo ainda estiver em execução após o tempo limite, ele é finalizado à força.

-----

## 📦 Build

Execute o seguinte comando para compilar o executável:

```bash
dotnet build -c Release
```

A saída estará localizada em: `bin/Release/net9.0/GS.exe`

-----

## 🖥️ Exemplo de Saída

```console
Starting GameServer...
Press Ctrl+C to stop server safely

[21:15:32]  INFO GameServer: Server starting...
[21:15:40]  INFO GameServer: Server is now online

Ctrl+C detected. Initiating safe shutdown...
Waiting for server to save and shutdown...
✓ Server stopped gracefully
```

-----

## 🔁 Fluxo de Trabalho (Workflow)

1.  Edite o código do seu servidor L2 no IntelliJ IDEA.
2.  Compile seus artefatos (que gera os JARs em `out\jar\`).
3.  Execute o `GS.exe` (esta aplicação).
4.  Selecione a opção **2** (Iniciar Servidor com atualizações).
5.  Para parar, pressione **`Ctrl+C`**.
6.  Repita.