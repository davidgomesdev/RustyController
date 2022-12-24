use std::sync::Arc;
use std::sync::Mutex;

use juniper::RootNode;
use tokio::sync::broadcast::Sender;
use tokio::sync::watch::Receiver;

use crate::{ControllerChange, EffectChange, PsMoveController};
use crate::graphql::schema_mutation::MutationRoot;
use crate::graphql::schema_query::QueryRoot;
use crate::graphql::schema_subscription::SubscriptionRoot;

pub struct Context {
    pub effect_tx: Arc<Sender<EffectChange>>,
    pub ctrl_rx: Arc<Mutex<Receiver<ControllerChange>>>,
    pub controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
}

impl juniper::Context for Context {}

impl Clone for Context {
    fn clone(&self) -> Context {
        Context {
            effect_tx: self.effect_tx.clone(),
            ctrl_rx: self.ctrl_rx.clone(),
            controllers: self.controllers.clone(),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, SubscriptionRoot)
}
