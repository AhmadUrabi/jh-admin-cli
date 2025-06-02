# JH Admin CLI

A command-line interface for managing administrative tasks.

## Features

- Email Management (Zoho)
  - List email users
  - More features coming soon

## Installation

```bash
cargo build --release
```

The executable will be available at `target/release/jh_admin_cli`.

## Environment Variables

The CLI requires the following environment variables to be set:

```bash
# Zoho API credentials
ZOHO_CLIENT_ID=your_client_id
ZOHO_CLIENT_SECRET=your_client_secret
ZOHO_ZOID=your_zoho_organization_id
```

You can set these variables in a `.env` file in the project root.

## Zoho Authentication

The Email module uses Zoho OAuth for authentication. The first time you run a command that requires Zoho authentication, you will be:

1. Redirected to a browser to authenticate with Zoho
2. Asked to provide the authorization code from the redirect URL
3. Automatically issued access and refresh tokens

After the initial authentication, the refresh token is stored in `zoho_refresh_token.json` and used to automatically refresh the access token when needed. The access token is refreshed once every 30 minutes at most.

## Usage

### List Email Users

```bash
jh_admin_cli email list-users
```

This command lists all users in your Zoho organization along with their email addresses.

## Development

### Project Structure

- `src/modules/` - Contains modules for different functionalities
  - `email.rs` - Email management module for Zoho
- `src/models/` - Data models used by the modules

### Adding New Functionality

To add new functionality:

1. Create a new module in `src/modules/` or extend an existing one
2. Define models in `src/models/` if needed
3. Use the `#[derive(Module)]` and `#[derive_tool]` macros to expose functionality to the CLI

## Dependencies

- `clap` - Command-line argument parsing
- `reqwest` - HTTP client
- `serde` - Serialization/deserialization
- `dotenv` - Environment variable loading
- `webbrowser` - Browser interaction for OAuth
- `url` - URL parsing