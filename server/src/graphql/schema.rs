use std::sync::Arc;

use juniper::{EmptySubscription, RootNode};
use tokio::sync::Mutex;
use tokio::sync::watch::Sender;

use crate::graphql::schema_mutation::MutationRoot;
use crate::graphql::schema_query::QueryRoot;
use crate::ps_move::models::LedEffect;
use crate::PsMoveController;

pub struct Context {
    pub tx: Arc<Sender<LedEffect>>,
    pub controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
