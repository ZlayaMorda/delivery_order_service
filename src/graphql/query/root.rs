use async_graphql::{MergedObject};
use crate::graphql::query::product::ProductQuery;

#[derive(MergedObject, Default)]
pub struct RootQuery(ProductQuery);