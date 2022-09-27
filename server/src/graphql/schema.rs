use std::sync::Arc;

use juniper::{EmptySubscription, RootNode};
use tokio::sync::watch::Sender;

use crate::graphql::schema_mutation::MutationRoot;
use crate::graphql::schema_query::QueryRoot;
use crate::ps_move::models::LedEffect;

pub struct Context {
    pub tx: Arc<Sender<LedEffect>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
