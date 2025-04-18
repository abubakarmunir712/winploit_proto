use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use dirs;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    
    loop {
        // Read command from client
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(()); // Client disconnected
        }

        let input = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        
        // Exit condition
        if input == "exit" {
            stream.write_all(b"Goodbye\n")?;
            return Ok(());
        }

        // Skip empty input
        if input.is_empty() {
            stream.write_all(b"Empty command\n")?;
            continue;
        }

        // Split input into command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            stream.write_all(b"Invalid command\n")?;
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];
        let current_dir = env::current_dir()?;

        // Handle cd command specially
        if command == "cd" {
            let target_dir = if args.is_empty() {
                // If no argument, go to home directory
                dirs::home_dir().unwrap_or(current_dir)
            } else {
                PathBuf::from(args[0])
            };

            // Change directory
            match env::set_current_dir(&target_dir) {
                Ok(_) => {
                    let new_dir = env::current_dir()?;
                    let response = format!("Changed directory to {}\n", new_dir.display());
                    stream.write_all(response.as_bytes())?;
                }
                Err(e) => {
                    let response = format!("Error changing directory: {}\n", e);
                    stream.write_all(response.as_bytes())?;
                }
            }
            continue;
        }

        // Execute other commands
        let output = Command::new(command)
            .args(args)
            .current_dir(current_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        // Send output or error back to client
        match output {
            Ok(output) => {
                let mut response = Vec::new();
                response.extend_from_slice(&output.stdout);
                response.extend_from_slice(&output.stderr);
                if response.is_empty() {
                    response = b"Command executed successfully\n".to_vec();
                }
                stream.write_all(&response)?;
            }
            Err(e) => {
                let response = format!("Error executing command: {}\n", e);
                stream.write_all(response.as_bytes())?;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}