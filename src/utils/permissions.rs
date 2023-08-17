use std::str::FromStr;
use async_graphql::{async_trait, Context, Error, Guard};
use crate::graphql::extension::authentication::AuthenticatedUser;
use crate::utils::errors::AppError;
use crate::utils::errors::AppError::Unauthorized;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    Authorized,
    Admin,
    User,
    Courier
}

impl FromStr for Role {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Role::User),
            "admin" => Ok(Role::Admin),
            "courier" => Ok(Role::Courier),
            _ => Err(Unauthorized())
        }
    }
}

pub(crate) struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub(crate) fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        let auth_user = ctx.data_opt::<AuthenticatedUser>();
        match auth_user {
            Some(user) => {
                if user.role == self.role || self.role == Role::Authorized{
                    Ok(())
                }
                else {
                    Err(Error::from("Permission denied".to_string()))
                }
            }
            None => { Err(Error::from("Unauthorized".to_string())) }
        }
    }
}
