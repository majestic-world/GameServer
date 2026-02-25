using System.Diagnostics;
using System.Runtime.InteropServices;

namespace GameServer;

internal static class ServerManager
{
    private const uint CtrlCEvent = 0;
    private static Process? _serverProcess;
    private static ServerConfig _config = null!;

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool GenerateConsoleCtrlEvent(uint dwCtrlEvent, uint dwProcessGroupId);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool AttachConsole(uint dwProcessId);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool FreeConsole();

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool SetConsoleCtrlHandler(ConsoleCtrlDelegate? handlerRoutine, bool add);

    private static void Main()
    {
        Console.Title = "GameServer Manager By Mk";
        _config = ServerConfig.Load();

        SetConsoleCtrlHandler(ConsoleCtrlHandler, true);

        while (true)
        {
            Console.Clear();
            DrawHeader();

            Console.WriteLine("1. Start Server");
            Console.WriteLine("2. Start Server (with updates)");
            Console.Write("\nSelect option: ");

            var option = Console.ReadLine();

            switch (option)
            {
                case "1":
                    StartServer(false);
                    break;
                case "2":
                    StartServer(true);
                    break;
                case "3":
                    UpdateJars();
                    break;
                case "4":
                    StopServer();
                    break;
                case "5":
                    if (Console.ReadKey().Key == ConsoleKey.Y)
                        StopServer();
                    else
                        continue;

                    return;
            }
        }
    }

    private static bool ConsoleCtrlHandler(uint ctrlType)
    {
        if (ctrlType is not (CtrlCEvent or 2)) return true;
        Console.WriteLine("\n\nShutdown signal received. Stopping server safely...");
        StopServer();
        Thread.Sleep(2000);
        return true;
    }

    private static void DrawHeader()
    {
        Console.ForegroundColor = ConsoleColor.Cyan;
        Console.WriteLine("╔═══════════════════════════════════════╗");
        Console.WriteLine("║       GameServer Manager By Mk        ║");
        Console.WriteLine("╚═══════════════════════════════════════╝");
        Console.ResetColor();
        Console.WriteLine();
    }

    private static void StartServer(bool updateFirst)
    {
        if (updateFirst) UpdateJars();

        Console.Clear();
        Console.ForegroundColor = ConsoleColor.Green;
        Console.WriteLine("Starting...");
        Console.WriteLine("Press Ctrl+C to stop server safely\n");
        Console.ResetColor();

        var startInfo = new ProcessStartInfo
        {
            FileName = Path.Combine(_config.JavaPath, "java"),
            Arguments = _config.JavaArgs,
            WorkingDirectory = _config.ServerPath,
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            RedirectStandardInput = true,
            CreateNoWindow = false
        };

        _serverProcess = Process.Start(startInfo);

        Console.CancelKeyPress += (_, e) =>
        {
            e.Cancel = true;
            StopServer();
        };

        if (_serverProcess != null)
        {
            _serverProcess.OutputDataReceived += (_, e) =>
            {
                if (string.IsNullOrEmpty(e.Data)) return;
                if (e.Data.Contains("ERROR") || e.Data.Contains("Exception"))
                    Console.ForegroundColor = ConsoleColor.Red;
                else if (e.Data.Contains("WARNING") || e.Data.Contains("WARN"))
                    Console.ForegroundColor = ConsoleColor.Yellow;
                else if (e.Data.Contains("Shutdown") || e.Data.Contains("Saving"))
                    Console.ForegroundColor = ConsoleColor.Cyan;

                Console.WriteLine(e.Data);
                Console.ResetColor();
            };

            _serverProcess.ErrorDataReceived += (_, e) =>
            {
                if (!string.IsNullOrEmpty(e.Data))
                {
                    Console.ForegroundColor = ConsoleColor.Red;
                    Console.WriteLine($"[ERROR] {e.Data}");
                    Console.ResetColor();
                }
            };

            _serverProcess.BeginOutputReadLine();
            _serverProcess.BeginErrorReadLine();
            _serverProcess.WaitForExit();

            var exitCode = _serverProcess.ExitCode;

            switch (exitCode)
            {
                case 2:
                    Console.ForegroundColor = ConsoleColor.Yellow;
                    Console.WriteLine("\n========================================");
                    Console.WriteLine("Server Restarting...");
                    Console.WriteLine("========================================");
                    Console.ResetColor();
                    Thread.Sleep(2000);
                    StartServer(true);
                    break;
                case 1:
                    Console.ForegroundColor = ConsoleColor.Red;
                    Console.WriteLine("\n========================================");
                    Console.WriteLine("Server Terminated with Error!");
                    Console.WriteLine("========================================");
                    Console.ResetColor();
                    Console.WriteLine("\nPress any key to continue...");
                    Console.ReadKey();
                    break;
            }
        }

        _serverProcess = null;
    }

    private static void StopServer()
    {
        if (_serverProcess == null || _serverProcess.HasExited)
        {
            Console.WriteLine("Server is not running.");
            return;
        }

        Console.ForegroundColor = ConsoleColor.Yellow;
        Console.WriteLine("\n========================================");
        Console.WriteLine("Initiating Safe Shutdown...");
        Console.WriteLine("========================================");
        Console.ResetColor();

        try
        {
            if (AttachConsole((uint)_serverProcess.Id))
            {
                SetConsoleCtrlHandler(null, true);
                GenerateConsoleCtrlEvent(CtrlCEvent, 0);
                Thread.Sleep(500);
                FreeConsole();
                SetConsoleCtrlHandler(ConsoleCtrlHandler, true);
            }

            if (!_serverProcess.WaitForExit(30000))
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("\nServer did not stop in time. Forcing shutdown...");
                Console.ResetColor();
                _serverProcess.Kill();
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine("\nServer stopped gracefully");
                Console.ResetColor();
            }
        }
        catch (Exception ex)
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine($"\nError during shutdown: {ex.Message}");
            Console.ResetColor();

            try
            {
                _serverProcess.Kill();
            }
            catch
            {
                // ignored
            }
        }

        _serverProcess = null;
    }

    private static void UpdateJars()
    {
        Console.ForegroundColor = ConsoleColor.Yellow;
        Console.WriteLine("\n========================================");
        Console.WriteLine("Checking for Updates");
        Console.WriteLine("========================================");
        Console.ResetColor();
        Console.WriteLine();

        if (!Directory.Exists(_config.OutputJarPath))
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine($"Output directory not found: {_config.OutputJarPath}");
            Console.ResetColor();
            return;
        }

        var jarFiles = Directory.GetFiles(_config.OutputJarPath, "*.jar");

        if (jarFiles.Length == 0)
        {
            return;
        }

        Console.WriteLine($"Found {jarFiles.Length} JAR file(s):\n");

        var updated = 0;

        foreach (var jarFile in jarFiles)
        {
            var fileName = Path.GetFileName(jarFile);
            var destFile = Path.Combine(_config.ServerPath, "libs", fileName);

            try
            {
                Console.Write($"Updating {fileName}... ");
                File.Copy(jarFile, destFile, true);
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine("Ok");
                Console.ResetColor();
                updated++;
            }
            catch (Exception ex)
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine($"✗ Error: {ex.Message}");
                Console.ResetColor();
            }
        }

        Console.WriteLine();
        if (updated > 0)
        {
            Console.ForegroundColor = ConsoleColor.Green;
            Console.WriteLine($"{updated} JAR file(s) updated successfully");
            Console.ResetColor();
        }
        else
        {
            Console.ForegroundColor = ConsoleColor.Yellow;
            Console.WriteLine("No files were updated");
            Console.ResetColor();
        }
    }

    private delegate bool ConsoleCtrlDelegate(uint ctrlType);
}

internal class ServerConfig
{
    private const string ConfigFileName = "GameServer.properties";

    public string ServerPath { get; private set; } = string.Empty;
    public string JavaPath { get; private set; } = string.Empty;
    public string JavaArgs { get; private set; } = string.Empty;
    public string OutputJarPath { get; private set; } = string.Empty;

    public static ServerConfig Load()
    {
        var config = new ServerConfig();
        var configPath = Path.Combine(AppContext.BaseDirectory, ConfigFileName);

        if (!File.Exists(configPath))
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine($"Configuration file not found: {ConfigFileName}");
            Console.WriteLine("Press any key to exit...");
            Console.ResetColor();
            Console.ReadKey();
            Environment.Exit(1);
        }

        try
        {
            var lines = File.ReadAllLines(configPath);

            foreach (var line in lines)
            {
                if (string.IsNullOrWhiteSpace(line) || line.TrimStart().StartsWith("#"))
                    continue;

                var parts = line.Split('=', 2);
                if (parts.Length != 2)
                    continue;

                var key = parts[0].Trim();
                var value = parts[1].Trim().Trim('"');

                switch (key)
                {
                    case "ServerPath":
                        config.ServerPath = value;
                        break;
                    case "JavaPath":
                        config.JavaPath = value;
                        break;
                    case "JavaArgs":
                        config.JavaArgs = value;
                        break;
                    case "OutputJarPath":
                        config.OutputJarPath = value;
                        break;
                }
            }
            
            if (string.IsNullOrEmpty(config.ServerPath) ||
                string.IsNullOrEmpty(config.JavaPath) ||
                string.IsNullOrEmpty(config.JavaArgs) ||
                string.IsNullOrEmpty(config.OutputJarPath))
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("Error: Configuration file is incomplete!");
                Console.WriteLine("Please check all required properties are set.");
                Console.ResetColor();
                Console.WriteLine("\nPress any key to exit...");
                Console.ReadKey();
                Environment.Exit(1);
            }

            return config;
        }
        catch (Exception ex)
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine($"Error loading configuration: {ex.Message}");
            Console.ResetColor();
            Console.WriteLine("\nPress any key to exit...");
            Console.ReadKey();
            Environment.Exit(1);
            return null!;
        }
    }
}