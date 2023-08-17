use crate::graphql::extension::authentication::AuthenticatedUser;

pub mod authorization;
pub mod buckets;
pub mod orders;
pub mod products;

#[derive(Clone)]
pub(crate) struct State {
    user: AuthenticatedUser,
}