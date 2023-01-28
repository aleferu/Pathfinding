use std::{fs, env};
use std::collections::HashMap;

// Modify the settings in the extra_folder folder, nothing to see here.
pub fn get_settings() -> HashMap<String, String> {
    let os_name = env::consts::OS;

    let content: String;
    if os_name.eq("windows") {
        content = fs::read_to_string("extra_folder\\settings.toml").expect("File not found (Windows)");
    } else {
        content = fs::read_to_string("extra_folder/settings.toml").expect("File not found");
    }

    let content_lines: Vec<&str> = content.lines().filter(|l| l.contains("=")).collect();

    let mut settings: HashMap<String, String> = HashMap::new();
    for line in content_lines {
        let line_splitted: Vec<&str> = line.split("=").collect();
        let key = line_splitted.get(0).unwrap();
        let key_len = key.len();
        let value = line_splitted.get(1).unwrap();
        let value_len = value.len();

        // Ugly, but does the work, ty Rust compiler :D
        if value.contains("\"") {
            let value = &&value[1..value_len-1];
            settings.insert(key[..key_len-1].to_string(), value[1..].to_string());
        } else {
            settings.insert(key[..key_len-1].to_string(), value[1..].to_string());
        }
    }
    settings
}