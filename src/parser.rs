use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn parse_paste_input() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut env_vars = HashMap::new();
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("paste your secrets below (press enter twice to finish)");
    println!("example: KEY=VALUE (one per line)");
    println!();

    let mut lines = Vec::new();
    let mut blank_line_count = 0;

    for line in reader.lines() {
        let line = line?;
        
        if line.is_empty() {
            blank_line_count += 1;
            if blank_line_count >= 2 {
                break;
            }
        } else {
            blank_line_count = 0;
            lines.push(line);
        }
    }

    for line in lines {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();

            if !key.is_empty() {
                env_vars.insert(key, value);
            }
        } else {
            println!("skipping invalid line: {}", line);
        }
    }

    Ok(env_vars)
}
