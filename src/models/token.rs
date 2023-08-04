
#[derive(Debug)]
pub struct TokenClaims {
    pub sub: String,
    pub rol: String,
    pub iat: usize,
    pub exp: usize,
}