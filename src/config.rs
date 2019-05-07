use std::env;

pub struct Config {
    token: String
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            token: env::var("DISCORD_TOKEN").expect("Expected the DISCORD_TOKEN environment variable.")
        }
    }

    /// Get the bot authentication token
    pub fn token(&self) -> &str {
        &self.token
    }
}