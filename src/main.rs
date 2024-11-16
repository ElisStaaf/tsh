use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::process::{exit, Command, Output};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use chrono::Local; 
use std::path::Path;

mod commands;
mod history;
use history::CommandHistory;

fn arrows_library() {
    let output = Command::new("bash")
     .arg("../lib/arrows.sh")
     .output()
     .expect("Error");

    println!("\n{}", String::from_utf8_lossy(&output.stdout));
}

fn list_files_with_size() {
    let current_dir = env::current_dir().unwrap();
    let dir_str = current_dir.to_str().unwrap_or_default();
    let entries = fs::read_dir(current_dir).unwrap();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut arrow_color = ColorSpec::new();
    arrow_color.set_fg(Some(Color::Red));

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();

                let file_size = fs::metadata(&path).unwrap().len();

                let mut color = ColorSpec::new();
                if file_size < 1024 {
                    color.set_fg(Some(Color::Blue)); 
                } else if file_size < 1048576 {
                    color.set_fg(Some(Color::Yellow)); 
                } else {
                    color.set_fg(Some(Color::Red));
                }

                stdout.set_color(&arrow_color).unwrap(); 
                write!(stdout, "↪ ").unwrap();
                stdout.reset().unwrap();

                stdout.set_color(&color).unwrap();
                write!(stdout, "{:<20} {:>10} bytes", file_name, file_size).unwrap();
                stdout.reset().unwrap();
                println!(); 
            }
            Err(_) => continue,
        }
    }
}

fn show_current_time() {
    let now = Local::now(); 
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut arrow_color = ColorSpec::new();
    arrow_color.set_fg(Some(Color::Red));

    stdout.set_color(&arrow_color).unwrap();
    write!(stdout, "↪ ").unwrap();
    stdout.reset().unwrap();

    println!("{}", formatted_time);
}

fn show_hostname() {
    let hostname = env::var("COMPUTERNAME");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut arrow_color = ColorSpec::new();
    arrow_color.set_fg(Some(Color::Red));
    
    stdout.set_color(&arrow_color).unwrap();
    write!(stdout, "↪ ").unwrap();
    stdout.reset().unwrap();

    let mut text_color_hostname = ColorSpec::new();
    text_color_hostname.set_fg(Some(Color::Cyan));

    stdout.set_color(&text_color_hostname).unwrap();
    write!(stdout, "Hostname: ").unwrap();
    stdout.reset().unwrap();

    println!("{}", hostname.unwrap_or("Unknown Hostname".to_string())); 

}



fn main() {
    let mut command_history = CommandHistory::new();

    loop {
        let current_dir = env::current_dir().unwrap();
        let dir_str = current_dir.to_str().unwrap_or_default();

        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let mut green = ColorSpec::new();
        green.set_fg(Some(Color::Green)).set_bold(true);
        stdout.set_color(&green).unwrap();

        let mut arrow_color = ColorSpec::new();
        arrow_color.set_fg(Some(Color::Red));

        stdout.set_color(&arrow_color).unwrap();
        write!(stdout, "").unwrap();
        stdout.reset().unwrap();

        stdout.set_color(&green).unwrap();
        write!(stdout, "{:?} $ ", dir_str).unwrap();
        stdout.reset().unwrap();

        let mut command_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command_input).unwrap();

        let command_input = command_input.trim();

        if command_input.is_empty() {
            continue;
        }

        command_history.add_command(command_input);

        if command_input == "exit" {
            break;
        }

        if command_input == "history" {
            command_history.show_history();
            continue;
        }

        if command_input.starts_with("cd ") {
            commands::change_directory(&command_input[3..]);
            continue;
        }

        if command_input == "ls" {
            list_files_with_size();
            continue;
        }

        if command_input == "now" {
            show_current_time();
            continue;
        }

        if command_input == "hostname" {
            show_hostname();
            continue;
        }

        if command_input == "arrows_library" {
            arrows_library();
            continue;
        }

        let mut parts = command_input.split_whitespace();
        let command = parts.next().unwrap_or_default();

        let output = Command::new(command).args(parts).output();

        match output {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    print!("↪ ");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprint!("→ ");
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Error: Execution of command \"{}\" failed", e);
                exit(1);
            }
        }
    }
}
