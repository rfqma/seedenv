# seedenv

Rust CLI tool for bulk uploading repository secrets to GitHub. Interactive prompts guide you through the process - just paste your secrets and they'll be encrypted and uploaded!

## Features

- ✅ Interactive CLI prompts (no config files needed!)
- 📝 Paste mode - add multiple secrets at once
- 🔐 Proper encryption using GitHub's public key
- 📋 Lists existing secrets before upload
- 🔄 Automatically updates existing secrets

## Prerequisites

- GitHub Personal Access Token with `repo` scope
- Repository with Actions enabled

## Installation

### Quick Install

```bash
curl -sSL https://raw.githubusercontent.com/rfqma/seedenv/main/install.sh | bash
```

This will:

- ✅ Detect your OS and architecture
- ✅ Download the appropriate binary
- ✅ Install to `/usr/local/bin/seedenv` (or Windows equivalent)
- ✅ Make it available globally

### Manual Download

Or download the binary directly from [GitHub Releases](https://github.com/rfqma/seedenv/releases):

```bash
# macOS (Intel)
curl -sSL -O https://github.com/rfqma/seedenv/releases/latest/download/seedenv-macos-x86_64
chmod +x seedenv-macos-x86_64
sudo mv seedenv-macos-x86_64 /usr/local/bin/seedenv

# macOS (Apple Silicon)
curl -sSL -O https://github.com/rfqma/seedenv/releases/latest/download/seedenv-macos-aarch64
chmod +x seedenv-macos-aarch64
sudo mv seedenv-macos-aarch64 /usr/local/bin/seedenv

# Linux
curl -sSL -O https://github.com/rfqma/seedenv/releases/latest/download/seedenv-linux-x86_64
chmod +x seedenv-linux-x86_64
sudo mv seedenv-linux-x86_64 /usr/local/bin/seedenv

# Windows
Download seedenv-windows-x86_64.exe from releases
```

## Usage

Once installed, simply run:

```bash
seedenv
```

Then follow the interactive prompts.

## Environment File Format

No configuration file needed! Just provide your GitHub credentials when prompted.

**To add secrets:**

1. When prompted, paste secrets one per line in `KEY=VALUE` format
2. Press Enter twice when done
3. Review and confirm the upload

Example:

```
DATABASE_URL=postgresql://user:pass@localhost:5432/myapp
API_KEY=sk-1234567890abcdef
STRIPE_SECRET=sk_live_1234567890abcdef
JWT_SECRET=your-jwt-secret-here
REDIS_URL=redis://localhost:6379/0
SMTP_PASSWORD=your-smtp-password
```

**Format rules:**

- Use `KEY=VALUE` format
- One secret per line
- Empty lines and lines starting with `#` are ignored
- No spaces around the `=` sign (value can have spaces)

## GitHub Personal Access Token

1. Go to GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token"
3. Select the `repo` scope
4. Copy the generated token and paste it

## How It Works

1. **Interactive Prompts**: Asks for GitHub token, username, and repo name
2. **API Connection**: Fetches repository's public key for encryption
3. **Existing Secrets**: Lists what's already in the repository
4. **User Input**: Prompts to paste secrets in KEY=VALUE format
5. **Validation**: Shows preview of secrets to upload
6. **Confirmation**: Asks for final approval before uploading
7. **Encryption**: Encrypts each secret using libsodium sealed box encryption
8. **Upload**: Uses GitHub REST API to create or update secrets
9. **Status**: Shows progress for each secret uploaded

## Security

- Secrets are encrypted using GitHub's public key before transmission
- Uses HTTPS for all API communications
- Token-based authentication only
- Values are never logged or displayed

## Troubleshooting

### GitHub token errors

- Make sure your token has `repo` scope
- Token should have permissions to create/update secrets
- Check that the repository exists and you have access

### Invalid secret format

- Use `KEY=VALUE` format (no spaces around `=`)
- Values can contain spaces: `KEY=value with spaces`
- Skip lines starting with `#` or empty lines
