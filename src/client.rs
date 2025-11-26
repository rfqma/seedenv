use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PublicKey {
    pub key_id: String,
    pub key: String,
}

#[derive(Serialize)]
struct SecretPayload {
    encrypted_value: String,
    key_id: String,
}

#[derive(Deserialize)]
pub struct Secret {
    pub name: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
struct SecretsResponse {
    pub secrets: Vec<Secret>,
}

pub struct GitHubClient {
    client: Client,
    pub owner: String,
    pub repo: String,
    token: String,
}

impl GitHubClient {
    pub fn new(owner: String, repo: String, token: String) -> Self {
        let client = Client::new();
        Self {
            client,
            owner,
            repo,
            token,
        }
    }

    pub async fn get_public_key(&self) -> Result<PublicKey, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/secrets/public-key",
            self.owner, self.repo
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "seedenv")
            .send()
            .await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;
            println!("error body: {}", error_body);
            return Err(format!("failed to get public key: {}", error_body).into());
        }

        let public_key: PublicKey = response.json().await?;
        Ok(public_key)
    }

    pub async fn list_secrets(&self) -> Result<Vec<Secret>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/secrets",
            self.owner, self.repo
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "seedenv")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("failed to list secrets: {}", response.status()).into());
        }

        let secrets_response: SecretsResponse = response.json().await?;
        Ok(secrets_response.secrets)
    }

    pub async fn create_or_update_secret(
        &self,
        secret_name: &str,
        encrypted_value: &str,
        key_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/secrets/{}",
            self.owner, self.repo, secret_name
        );

        let payload = SecretPayload {
            encrypted_value: encrypted_value.to_string(),
            key_id: key_id.to_string(),
        };

        let response = self
            .client
            .put(&url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "seedenv")
            .json(&payload)
            .send()
            .await?;

        if response.status().as_u16() == 201 {
            println!("    ✅ created");
        } else if response.status().as_u16() == 204 {
            println!("    ✅ updated");
        } else {
            return Err(format!(
                "failed to create/update secret {}: {}",
                secret_name,
                response.status()
            )
            .into());
        }

        Ok(())
    }
}
