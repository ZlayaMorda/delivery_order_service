use std::str::FromStr;
use axum::{async_trait, Extension, RequestPartsExt};
use axum::extract::{FromRequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use crate::AppState;
use crate::models::authorization::TokenClaims;
use crate::utils::permissions::Role;
use crate::utils::errors::AppError;
use crate::utils::errors::AppError::{BadRequest, Unauthorized};

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_uuid: Uuid,
    pub role: Role,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        let Extension(state) = parts
            .extract::<Extension<AppState>>()
            .await
            .map_err(|_| AppError::AppError("Cant get App state".to_string()))?;
        println!("After state");

        let token = match parts.headers.get(AUTHORIZATION) {
            Some(token) => { Ok(token.to_str().expect("Cant convert token to string")) }
            None => { Err(BadRequest("AUTHORIZATION header is missing".to_string())) }
        }?;
        // let token = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
        //     .await
        //     .map(|token| token.token().to_string())
        //     .map_err(|_| BadRequest("AUTHORIZATION header is missing".to_string()))?;

        return match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(state.env.secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => {
                let iat = Utc::now().timestamp() as usize;
                if iat >= c.claims.exp {
                    return Err(Unauthorized());
                }

                let temp = c.claims;
                let user_info: AuthenticatedUser = AuthenticatedUser {
                    user_uuid: temp.sub.parse().expect("Cant convert string to UUID"),
                    role: Role::from_str(&temp.rol)?,
                };

                Ok(user_info)
            }
            Err(_) => {
                Err(Unauthorized())
            }
        }
    }

}