use std::ops::Add;
use async_graphql::{Context, Object};
use uuid::Uuid;
use crate::AppState;
use crate::models::products::{Product, ProductType};
use crate::utils::errors::AppError;
use crate::graphql::services::utils::{get_fields};

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn products<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Product>, AppError> {
        let app_state = ctx.data::<AppState>()
            .expect("Error while get app state from the context");

        let row = sqlx::query_as::<_, Product>(
            &get_fields(ctx).add(" FROM products"))
            .fetch_all(&app_state.db)
            .await?;
        Ok(row)
    }

    async fn products_type<'ctx>(&self, ctx: &Context<'ctx>) -> Result<ProductType, AppError> {
        let app_state = ctx.data::<AppState>()
            .expect("Error while get app state from the context");

        let row = sqlx::query_as::<_, ProductType>(
            "SELECT DISTINCT product_type FROM products")
            .fetch_one(&app_state.db)
            .await?;
        Ok(row)
    }

    async fn products_from_restaurant<'ctx>(&self, ctx: &Context<'ctx>, restaurant: String) -> Result<Vec<Product>, AppError> {
        let app_state = ctx.data::<AppState>()
            .expect("Error while get app state from the context");

        let row = sqlx::query_as::<_, Product>(
            &get_fields(ctx).add(" FROM products WHERE restaurant = $1"))
            .bind(restaurant)
            .fetch_all(&app_state.db)
            .await?;
        Ok(row)
    }

    async fn products_by_id<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Result<Product, AppError> {
        let app_state = ctx.data::<AppState>()
            .expect("Error while get app state from the context");

        let row = sqlx::query_as::<_, Product>(
            "SELECT DISTINCT * FROM products WHERE product_uuid = $1")
            .bind(Uuid::parse_str(&id)?)
            .fetch_one(&app_state.db)
            .await?;
        Ok(row)
    }
}