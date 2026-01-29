# Majestic-World Server Manager

Gerenciador de console para servidores Lineage 2 com desligamento seguro e atualizações automáticas.

## Recursos

- Desligamento seguro via `Ctrl+C`
- Atualização automática dos arquivos `.jar`
- Logs coloridos (erros, avisos, shutdown)
- Reinício automático em caso de crash

## Requisitos

- .NET 10 (Native AOT)
- Java 17+
- Windows

## Configuração

Edite os caminhos no `Program.cs`:

```csharp
private const string ServerPath = @"C:\Users\Dev\Desktop\MyServer\gameserver";
private const string OutputJarPath = @"C:\Recreate\Lucera\out\jar";
```

## Uso

Execute `GS.exe` e escolha uma opção:

1. Iniciar servidor
2. Iniciar servidor (com atualizações)
3. Atualizar JARs apenas
4. Parar servidor
5. Sair

Pressione `Ctrl+C` para desligamento seguro a qualquer momento.

## Build

```bash
publish -c Release -r win-x64
```