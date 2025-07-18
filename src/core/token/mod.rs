//! This module manages the creation, validation, and refreshing of authentication tokens.

use crate::error::AuthError;

/// Represents a token pair (access and refresh).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// Trait to abstract token operations.
#[async_trait::async_trait]
pub trait TokenService: Send + Sync {
    /// Generates a new token pair for a given user.
    async fn generate_token_pair(&self, user_id: &str) -> Result<TokenPair, AuthError>;

    /// Validates an access token and extracts its claims.
    async fn validate_access_token(
        &self,
        token: &str,
    ) -> Result<Box<dyn crate::core::token::claims::Claims + Send + Sync>, AuthError>;

    /// Refreshes an access token using a refresh token.
    async fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenPair, AuthError>;
}

/// The default claims for JWTs.
pub mod claims;
pub mod jwt;
