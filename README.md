\# Majestic-World Server Manager

Console application for managing Lineage 2 game servers with safe shutdown and automatic updates.

\## Requirements

\- .NET 9.0

\- Java 17+

\- Windows OS



\## Configuration



Edit these paths in `Program.cs`:

```csharp

private const string ServerPath = @"C:\\Users\\Dev\\Desktop\\MyServer\\gameserver";

private const string OutputJarPath = @"C:\\Recreate\\Lucera\\out\\jar";

```



\## Features



\- \*\*Safe Shutdown\*\* - Sends Ctrl+C to Java process for graceful shutdown

\- \*\*Auto Update\*\* - Copies all JAR files from output directory to server

\- \*\*Color-coded Logs\*\* - Errors in red, warnings in yellow, shutdown in cyan

\- \*\*Auto Restart\*\* - Restarts server on crash (exit code 2)



\## Usage



\### Menu Options



1\. \*\*Start Server\*\* - Start without checking for updates

2\. \*\*Start Server (with updates)\*\* - Update JARs then start

3\. \*\*Update JARs only\*\* - Copy all `.jar` files to server

4\. \*\*Stop Server\*\* - Safe shutdown with data saving

5\. \*\*Exit\*\* - Close manager



\### Shortcuts



\- \*\*Ctrl+C\*\* - Safe shutdown (saves data, disconnects players)



\## How It Works



\*\*Update Process:\*\*

```

Source: C:\\Recreate\\Lucera\\out\\jar\\\*.jar

Destination: C:\\Users\\Dev\\Desktop\\MyServer\\gameserver\\libs\\

```



\*\*Safe Shutdown:\*\*

1\. Sends Ctrl+C signal to Java process

2\. Java saves all data and disconnects players

3\. Waits up to 30 seconds

4\. Forces shutdown if timeout



\## Build

```bash

dotnet build -c Release

```



Output: `bin/Release/net9.0/GS.exe`



\## Example Output

```

Starting GameServer...

Press Ctrl+C to stop server safely



\[21:15:32]  INFO GameServer: Server starting...

\[21:15:40]  INFO GameServer: Server is now online



Ctrl+C detected. Initiating safe shutdown...

Waiting for server to save and shutdown...

✓ Server stopped gracefully

```



\## Workflow



1\. Edit code in IntelliJ IDEA

2\. Build artifacts (JARs → `out\\jar\\`)

3\. Run `GameServer.exe`

4\. Select option 2 (Start with updates)

5\. Ctrl+C to stop

6\. Repeat


\## Troubleshooting



\- \*\*Server won't start:\*\* Check Java in PATH (`java -version`)

\- \*\*Updates not working:\*\* Verify `OutputJarPath` exists

\- \*\*Forced shutdown:\*\* Server took >30s to stop, killed forcefully

