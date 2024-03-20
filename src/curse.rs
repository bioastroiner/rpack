/*
use curseforge::official::prelude::*;
use once_cell::sync::Lazy;
/// The API base that will be used if a token is not provided.
/// This is an unofficial proxy that works without a token.
static PROXY_API_BASE: &str = "https://api.curse.tools/v1/cf/";
static TOKEN_VARIABLE: &str = "CURSEFORGE_API_TOKEN";
const GAME_MINECRAFT: i32 = 432;
/// Settings are lowered to reduce API spam.
static CLIENT_OPTIONS: ClientOptions = ClientOptions {
    // This is the maximum number of client connections allowed for the host.
    // Increasing this number may result in denial errors.
    max_connections: 1,
};
static CLIENT: Lazy<Client> = Lazy::new(|| match std::env::var(TOKEN_VARIABLE) {
    Ok(token) => {
        eprintln!("Using official CurseForge API with token.");
        Client::new(e::DEFAULT_API_BASE, Some(token), Some(&CLIENT_OPTIONS)).unwrap()
    }
    Err(_) => {
        eprintln!("Using proxy for CurseForge API without token.");
        Client::new(PROXY_API_BASE, None, Some(&CLIENT_OPTIONS)).unwrap()
    }
});
*/