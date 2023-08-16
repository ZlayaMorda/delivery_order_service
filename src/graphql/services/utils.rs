use std::ops::Add;
use async_graphql::{Context, SelectionField};

/// Convert camel case string to the snake case
/// need to parse graph ql fields
pub fn camel_to_snake(camel: &str) -> String {
    camel.chars().map( |x| match x {
            'A'..='Z' => '_'.to_string().add(&x.to_lowercase().to_string()),
            _ => x.to_string(),
        }
    ).collect()
}

/// Create start of the sql query of the form:
/// from ["product_uuid", "product_name", "product_type"]
/// to "SELECT product_uuid, product_name, product_type"
///
/// It's necessary to create more flexible queries from graphql to the database
pub fn create_sql_select_fields<'a>(fields: impl Iterator<Item = SelectionField<'a>>) -> String {
    let mut merged = fields.fold(String::from("SELECT "),
        |merged, field|
            merged.add(
                &camel_to_snake(field.name())
                    .add(", ")
            )
    );
    merged.pop().expect("Error while delete ', ' in sq; query");
    merged.pop().expect("Error while delete ', ' in sq; query");

    merged
}

pub fn get_fields(ctx: &Context) -> String {
    create_sql_select_fields(ctx.field().selection_set())
}
