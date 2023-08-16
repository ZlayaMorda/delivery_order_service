use async_graphql::{Context, MergedObject};
use crate::graphql::extension::authentication::AuthenticatedUser;
use crate::graphql::query::product::ProductQuery;
use crate::utils::errors::AppError;

#[derive(MergedObject, Default)]
pub struct RootQuery(ProductQuery);