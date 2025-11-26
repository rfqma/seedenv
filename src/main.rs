mod client;
mod crypto;
mod input;
mod parser;

use client::GitHubClient;
use crypto::encrypt_secret;
use input::{confirm, prompt, prompt_password};
use parser::parse_paste_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let banner = r#"
    █▀ █▀▀ █▀▀ █▀▄ █▀▀ █▄░█ █░█
    ▄█ ██▄ ██▄ █▄▀ ██▄ █░▀█ ▀▄▀
    "#;

    println!("{}", banner);

    let token = prompt_password("github personal access token:")?;
    let owner = prompt("github username:")?;
    let repo = prompt("github repository name:")?;

    println!();

    let github_client = GitHubClient::new(owner.clone(), repo.clone(), token);

    println!("fetching repository public key...");
    let public_key = github_client.get_public_key().await?;
    println!("public key retrieved ({})", public_key.key_id);
    println!();

    println!("📋 fetching existing repository secrets...");
    let existing_secrets = github_client.list_secrets().await?;
    let existing_secret_names: std::collections::HashSet<String> =
        existing_secrets.iter().map(|s| s.name.clone()).collect();

    if !existing_secret_names.is_empty() {
        println!("found {} existing repository secrets:", existing_secrets.len());
        for secret in &existing_secrets {
            println!(" • {} (updated at {})", secret.name, secret.updated_at);
        }
    } else {
        println!("no existing repository secrets found");
    }
    println!();

    let env_vars = parse_paste_input()?;

    if env_vars.is_empty() {
        println!("no secrets found. exiting.");
        return Ok(());
    }

    println!();
    println!("found {} secrets to upload", env_vars.len());
    for key in env_vars.keys() {
        let status = if existing_secret_names.contains(key) {
            "🔄 update"
        } else {
            "➕ create"
        };
        println!("  {} {}", status, key);
    }
    println!();
    
    let should_confirm = confirm("upload secrets to {}/{}")?;

    if !should_confirm {
        println!("upload cancelled.");
        return Ok(());
    }

    println!();
    println!("uploading secrets...");
    for (key, value) in env_vars {
        let encrypted_value = encrypt_secret(&value, &public_key.key)?;

        if existing_secret_names.contains(&key) {
            println!("  🔄 updating '{}'...", key);
        } else {
            println!("  ➕ creating '{}'...", key);
        }

        github_client
            .create_or_update_secret(&key, &encrypted_value, &public_key.key_id)
            .await?;
    }

    println!();
    println!("all secrets uploaded successfully!");

    Ok(())
}
