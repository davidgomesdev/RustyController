use std::sync::Arc;

use juniper::RootNode;
use tokio::sync::Mutex;
use tokio::sync::watch::Sender;

use crate::{EffectChange, PsMoveController};
use crate::graphql::schema_mutation::MutationRoot;
use crate::graphql::schema_query::QueryRoot;
use crate::graphql::schema_subscription::SubscriptionRoot;

#[derive(Clone)]
pub struct Context {
    pub tx: Arc<Sender<EffectChange>>,
    pub controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, SubscriptionRoot)
}
