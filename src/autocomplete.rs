use std::fs;
use std::path::Path;

pub fn autocomplete(input: &str, current_dir: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    let base_path = if input.is_empty() {
        Path::new(current_dir).to_path_buf()
    } else if input.starts_with('/') {
        Path::new(input).to_path_buf()
    } else {
        Path::new(current_dir).join(input)
    };

    if let Some(parent) = base_path.parent() {
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    if file_name_str.starts_with(base_path.file_name().unwrap_or_default().to_string_lossy().as_ref()) {
                        suggestions.push(file_name_str.to_string());
                    }
                }
            }
        }
    }

    suggestions
}
