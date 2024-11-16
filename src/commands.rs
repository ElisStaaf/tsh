use std::env;

pub fn change_directory(command: &str) {
    let dir: &str = command.trim();
    if dir.is_empty() {
        eprintln!("Error: There is no directory of said name.");
        return;
    }

    if let Err(e) = env::set_current_dir(dir) {
        eprintln!("Error: Couldn't enter directory: {}", e);
    }
}
