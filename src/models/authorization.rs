use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub rol: String,
    pub iat: usize,
    pub exp: usize,
}