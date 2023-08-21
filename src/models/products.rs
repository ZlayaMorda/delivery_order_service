use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;
use crate::utils::permissions::{Role, RoleGuard};
#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
pub struct Product {
    #[graphql(guard = "RoleGuard::new(Role::Authorized)")]
    #[sqlx(default)]
    pub product_uuid: Uuid,
    #[graphql(guard = "RoleGuard::new(Role::Authorized)")]
    #[sqlx(default)]
    pub product_type: String,
    #[graphql(guard = "RoleGuard::new(Role::Authorized)")]
    #[sqlx(default)]
    pub product_name: String,
    #[graphql(guard = "RoleGuard::new(Role::Authorized)")]
    #[sqlx(default)]
    pub restaurant: String,
    #[graphql(guard = "RoleGuard::new(Role::Authorized)")]
    #[sqlx(default)]
    pub price: f64,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    #[sqlx(default)]
    pub created_at: NaiveDateTime,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    #[sqlx(default)]
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
pub struct ProductType {
    pub product_type: String,
}

#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
pub struct ChangeProduct {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub product_uuid: Uuid,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub product_type: String,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub product_name: String,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub restaurant: String,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub price: f64,
}