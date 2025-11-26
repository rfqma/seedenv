use std::io::{self, Write};

pub fn prompt(message: &str) -> Result<String, io::Error> {
    print!("{} ", message);
    io::stdout().flush()?;
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    
    Ok(line.trim().to_string())
}

pub fn prompt_password(message: &str) -> Result<String, io::Error> {
    print!("{} ", message);
    io::stdout().flush()?;
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    
    let password = line.trim().to_string();
    
    Ok(password)
}

pub fn confirm(message: &str) -> Result<bool, io::Error> {
    print!("{} (y/n): ", message);
    io::stdout().flush()?;
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    
    Ok(line.trim().to_lowercase().starts_with('y'))
}
