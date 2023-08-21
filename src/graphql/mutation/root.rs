use async_graphql::MergedObject;
use crate::graphql::mutation::product::ProductMutation;

#[derive(MergedObject, Default)]
pub struct RootMutation(ProductMutation);