use async_graphql::{Context, Object};
use uuid::Uuid;
use crate::AppState;
use crate::graphql::services::utils::create_field_for_where_case;
use crate::models::products::ChangeProduct;
use crate::utils::errors::AppError;

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {

    /// resolver for adding product
    pub async fn add_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        product_type: String,
        product_name: String,
        restaurant: String,
        price: f64
    ) -> Result<ChangeProduct, AppError> {
        let app_state = ctx.data::<AppState>()
            .expect("Error while get app state from the context");

        let product = sqlx::query_as::<_, ChangeProduct>(
            "INSERT INTO products (product_type, product_name, restaurant, price)\
            VALUES($1, $2, $3, $4)\
            RETURNING product_uuid, product_type, product_name, restaurant, price;"
        )
            .bind(product_type)
            .bind(product_name)
            .bind(restaurant)
            .bind(price)
            .fetch_one(&app_state.db)
            .await?;
        Ok(product)
    }

    /// resolver for deleting rows by any column
    pub async fn delete_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        product_uuid: Option<Uuid>,
        product_type: Option<String>,
        product_name: Option<String>,
        restaurant: Option<String>,
        price: Option<f64>
    ) -> Result<Vec<ChangeProduct>, AppError> {

        let app_state = ctx.data::<AppState>()
            .expect("Error while get app state from the context");

        let mut first_add = false;

        let query = "DELETE FROM products WHERE".to_string()
        + &create_field_for_where_case::<Uuid>(
            &mut first_add,
            "",
            " product_uuid = ",
            "'",
            product_uuid
        )
        + &create_field_for_where_case::<String>(
            &mut first_add,
            " AND product_type = ",
            " product_type = ",
            "'",
            product_type
        )
        + &create_field_for_where_case::<String>(
            &mut first_add,
            " AND product_name = ",
            " product_name = ",
            "'",
            product_name
        )
        + &create_field_for_where_case::<String>(
            &mut first_add,
            " AND restaurant = ",
            " restaurant = ",
            "'",
            restaurant
        )
        + &create_field_for_where_case::<f64>(
            &mut first_add,
            " AND price = ",
            " price = ",
            "",
            price
        )
        + " RETURNING product_uuid, product_type, product_name, restaurant, price;";

        let products = sqlx::query_as::<_, ChangeProduct>(
            &query
        )
            .fetch_all(&app_state.db)
            .await?;
        Ok(products)
    }
}