use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, SetTitle},
};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Console::{
    AttachConsole, FreeConsole, GenerateConsoleCtrlEvent, SetConsoleCtrlHandler,
};

static SERVER_PROCESS_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone)]
struct ServerConfig {
    server_path: String,
    server_copy_path: String,
    java_path: String,
    java_args: String,
    output_jar_paths: Vec<String>,
}

impl ServerConfig {
    fn resolved_copy_path(&self) -> PathBuf {
        let sp = Path::new(&self.server_path);
        if self.server_copy_path.is_empty() {
            sp.to_path_buf()
        } else {
            sp.join(&self.server_copy_path)
        }
    }

    fn load() -> Self {
        let exe_dir = env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(Path::new(""))
            .to_path_buf();
        let config_path = exe_dir.join("GameServer.properties");

        if !config_path.exists() {
            set_color(Color::Red);
            println!("Configuration file not found: GameServer.properties");
            println!("Press any key to exit...");
            reset_color();
            wait_for_key();
            std::process::exit(1);
        }

        let content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                set_color(Color::Red);
                println!("Error loading configuration: {}", e);
                reset_color();
                println!("\nPress any key to exit...");
                wait_for_key();
                std::process::exit(1);
            }
        };

        let mut config = ServerConfig {
            server_path: String::new(),
            server_copy_path: String::new(),
            java_path: String::new(),
            java_args: String::new(),
            output_jar_paths: Vec::new(),
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let mut val = value.trim();
                if let Some(idx) = val.find('#') {
                    val = val[..idx].trim();
                }
                let val = val.trim_matches('"');
                match key {
                    "ServerPath" => config.server_path = val.to_string(),
                    "JavaPath" => config.java_path = val.to_string(),
                    "JavaArgs" => config.java_args = val.to_string(),
                    "ServerCopyPath" => config.server_copy_path = val.to_string(),
                    "OutputJarPath" => {
                        config.output_jar_paths = val
                            .split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        if config.server_path.is_empty()
            || config.java_path.is_empty()
            || config.java_args.is_empty()
            || config.output_jar_paths.is_empty()
        {
            set_color(Color::Red);
            println!("Error: Configuration file is incomplete!");
            println!("Please check all required properties are set.");
            reset_color();
            println!("\nPress any key to exit...");
            wait_for_key();
            std::process::exit(1);
        }

        config
    }
}

extern "system" fn console_ctrl_handler(ctrl_type: u32) -> BOOL {
    if ctrl_type == 0 || ctrl_type == 2 {
        println!("\n\nShutdown signal received. Stopping server safely...");
        let pid = SERVER_PROCESS_ID.load(Ordering::SeqCst);
        if pid != 0 {
            stop_server_by_pid(pid);
        }
        thread::sleep(Duration::from_millis(2000));
        return BOOL::from(true);
    }
    BOOL::from(false)
}

fn set_color(color: Color) {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, SetForegroundColor(color));
}

fn reset_color() {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, ResetColor);
}

fn draw_header() {
    set_color(Color::Cyan);
    println!("╔═══════════════════════════════════════╗");
    println!("║       GameServer Manager By Mk        ║");
    println!("╚═══════════════════════════════════════╝");
    reset_color();
    println!();
}

fn wait_for_key() {
    crossterm::terminal::enable_raw_mode().unwrap();
    loop {
        if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
            if let crossterm::event::Event::Key(_) = crossterm::event::read().unwrap() {
                break;
            }
        }
    }
    crossterm::terminal::disable_raw_mode().unwrap();
}

fn start_server(config: &ServerConfig, update_first: bool) {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, Clear(ClearType::All), crossterm::cursor::MoveTo(0, 0));
    
    if update_first {
        update_jars(config);
    }

    set_color(Color::Cyan);
    println!("Starting server... (Press Ctrl+C to stop safely)\n");
    reset_color();

    use std::os::windows::process::CommandExt;
    
    let mut child = match Command::new(Path::new(&config.java_path).join("java"))
        .raw_arg(&config.java_args)
        .current_dir(&config.server_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => {
                set_color(Color::Red);
                println!("[ERROR] Failed to start process: {}", e);
                reset_color();
                return;
            }
        };

    let pid = child.id();
    SERVER_PROCESS_ID.store(pid, Ordering::SeqCst);
    
    let stdout_reader = BufReader::new(child.stdout.take().unwrap());
    let stderr_reader = BufReader::new(child.stderr.take().unwrap());

    thread::spawn(move || {
        for line in stdout_reader.lines() {
            if let Ok(data) = line {
                if data.is_empty() { continue; }
                if data.contains("ERROR") || data.contains("Exception") {
                    set_color(Color::Red);
                } else if data.contains("WARNING") || data.contains("WARN") {
                    set_color(Color::Yellow);
                } else if data.contains("Shutdown") || data.contains("Saving") {
                    set_color(Color::Cyan);
                }
                println!("{}", data);
                reset_color();
            }
        }
    });

    thread::spawn(move || {
        for line in stderr_reader.lines() {
            if let Ok(data) = line {
                if !data.is_empty() {
                    set_color(Color::Red);
                    println!("[ERROR] {}", data);
                    reset_color();
                }
            }
        }
    });

    let status = child.wait().unwrap();
    SERVER_PROCESS_ID.store(0, Ordering::SeqCst);

    let exit_code = status.code().unwrap_or(0);
    match exit_code {
        2 => {
            set_color(Color::Yellow);
            println!("\n========================================");
            println!("Server Restarting...");
            println!("========================================");
            reset_color();
            thread::sleep(Duration::from_millis(2000));
            start_server(config, true);
        }
        1 => {
            set_color(Color::Red);
            println!("\n========================================");
            println!("Server Terminated with Error!");
            println!("========================================");
            reset_color();
            println!("\nPress any key to continue...");
            wait_for_key();
        }
        _ => {}
    }
}

fn stop_server_by_pid(pid: u32) {
    if pid == 0 {
        println!("Server is not running.");
        return;
    }

    set_color(Color::Yellow);
    println!("\n========================================");
    println!("Initiating Safe Shutdown...");
    println!("========================================");
    reset_color();

    unsafe {
        if AttachConsole(pid).is_ok() {
            let _ = SetConsoleCtrlHandler(None, true);
            let _ = GenerateConsoleCtrlEvent(0, 0);
            thread::sleep(Duration::from_millis(500));
            let _ = FreeConsole();
            let _ = SetConsoleCtrlHandler(Some(console_ctrl_handler), true);
        }
    }
    
    use windows::Win32::System::Threading::{OpenProcess, WaitForSingleObject, TerminateProcess, PROCESS_ALL_ACCESS};
    unsafe {
        if let Ok(handle) = OpenProcess(PROCESS_ALL_ACCESS, false, pid) {
            let wait_result = WaitForSingleObject(handle, 30000);
            if wait_result == windows::Win32::Foundation::WAIT_TIMEOUT {
                set_color(Color::Red);
                println!("\nServer did not stop in time. Forcing shutdown...");
                reset_color();
                let _ = TerminateProcess(handle, 1);
            } else {
                set_color(Color::Green);
                println!("\nServer stopped gracefully");
                reset_color();
            }
            let _ = windows::Win32::Foundation::CloseHandle(handle);
        }
    }
}

fn stop_server() {
    let pid = SERVER_PROCESS_ID.load(Ordering::SeqCst);
    stop_server_by_pid(pid);
}

fn update_jars(config: &ServerConfig) {
    let mut total_updated = 0;

    for output_path in &config.output_jar_paths {
        let p = Path::new(output_path);
        if !p.exists() {
            set_color(Color::Red);
            println!("Output not found: {}", output_path);
            reset_color();
            continue;
        }

        let mut jar_files = Vec::new();
        if let Ok(entries) = fs::read_dir(p) {
            for entry in entries.filter_map(Result::ok) {
                if let Some(ext) = entry.path().extension() {
                    if ext == "jar" {
                        jar_files.push(entry.path());
                    }
                }
            }
        }

        if jar_files.is_empty() {
            continue;
        }

        set_color(Color::Cyan);
        println!("[{}]\n", output_path);
        reset_color();

        for jar_file in jar_files {
            let file_name = jar_file.file_name().unwrap_or_default();
            let dest_file = config.resolved_copy_path().join(file_name);

            if fs::copy(&jar_file, &dest_file).is_ok() {
                println!("{} (Ok)", file_name.to_string_lossy());
                total_updated += 1;
            }
        }
    }

    if total_updated > 0 {
        println!();
        set_color(Color::Cyan);
        println!("{} JAR(s) updated successfully.", total_updated);
        reset_color();
    }
}

fn main() {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, SetTitle("GameServer Manager By Mk"));

    let config = ServerConfig::load();

    unsafe {
        let _ = SetConsoleCtrlHandler(Some(console_ctrl_handler), true);
    }

    loop {
        let _ = execute!(stdout, Clear(ClearType::All), crossterm::cursor::MoveTo(0, 0));
        draw_header();

        println!("1. Start Server");
        println!("2. Start Server (with updates)");
        print!("\nSelect option: ");
        let _ = stdout.flush();

        let mut option = String::new();
        if io::stdin().read_line(&mut option).is_err() {
            continue;
        }

        match option.trim() {
            "1" => start_server(&config, false),
            "2" => start_server(&config, true),
            "3" => update_jars(&config),
            "4" => stop_server(),
            "5" => {
                let mut should_stop = false;
                crossterm::terminal::enable_raw_mode().unwrap();
                loop {
                    if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
                        if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
                            if event.code == crossterm::event::KeyCode::Char('y') || event.code == crossterm::event::KeyCode::Char('Y') {
                                should_stop = true;
                            }
                            break;
                        }
                    }
                }
                crossterm::terminal::disable_raw_mode().unwrap();
                if should_stop {
                    stop_server();
                } else {
                    continue;
                }
                return;
            }
            _ => {}
        }
    }
}
