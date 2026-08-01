//! Configuration management for EEMP
//!
//! Loads configuration from environment variables following Twelve-Factor App principles.

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    Error(String),

    #[error("Missing required configuration: {0}")]
    MissingRequired(String),
}

/// Application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub access_token_secret: String,
    pub access_token_expiry_seconds: i64,
    pub refresh_token_expiry_seconds: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub argon2_memory_cost: u32,
    pub argon2_time_cost: u32,
    pub argon2_parallelism: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file (development only)
        dotenvy::dotenv().ok();

        Ok(Self {
            server: ServerConfig {
                host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("PORT")
                    .unwrap_or_else(|_| "8000".to_string())
                    .parse()
                    .map_err(|_| ConfigError::Error("Invalid PORT".to_string()))?,
                environment: std::env::var("ENVIRONMENT")
                    .unwrap_or_else(|_| "development".to_string())
                    .parse()
                    .map_err(|_| ConfigError::Error("Invalid ENVIRONMENT".to_string()))?,
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .map_err(|_| ConfigError::MissingRequired("DATABASE_URL".to_string()))?,
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
                min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            redis: RedisConfig {
                url: std::env::var("REDIS_URL")
                    .map_err(|_| ConfigError::MissingRequired("REDIS_URL".to_string()))?,
            },
            jwt: JwtConfig {
                access_token_secret: std::env::var("JWT_SECRET")
                    .map_err(|_| ConfigError::MissingRequired("JWT_SECRET".to_string()))?,
                access_token_expiry_seconds: std::env::var("JWT_ACCESS_EXPIRY_SECONDS")
                    .unwrap_or_else(|_| "900".to_string())  // 15 minutes
                    .parse()
                    .unwrap_or(900),
                refresh_token_expiry_seconds: std::env::var("JWT_REFRESH_EXPIRY_SECONDS")
                    .unwrap_or_else(|_| "604800".to_string())  // 7 days
                    .parse()
                    .unwrap_or(604800),
            },
            security: SecurityConfig {
                // Argon2id parameters from security architecture (OWASP recommended)
                argon2_memory_cost: 19456,  // 19 MiB
                argon2_time_cost: 2,        // 2 iterations
                argon2_parallelism: 1,      // 1 thread
            },
        })
    }

    pub fn is_production(&self) -> bool {
        self.server.environment == Environment::Production
    }

    pub fn is_development(&self) -> bool {
        self.server.environment == Environment::Development
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "staging" | "stage" => Ok(Self::Staging),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(format!("Invalid environment: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_str() {
        assert_eq!("development".parse::<Environment>().unwrap(), Environment::Development);
        assert_eq!("staging".parse::<Environment>().unwrap(), Environment::Staging);
        assert_eq!("production".parse::<Environment>().unwrap(), Environment::Production);
        assert_eq!("dev".parse::<Environment>().unwrap(), Environment::Development);
        assert_eq!("prod".parse::<Environment>().unwrap(), Environment::Production);
    }
}
