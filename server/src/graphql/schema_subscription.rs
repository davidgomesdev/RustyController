use std::pin::Pin;

use async_stream::stream;
use juniper::futures::Stream;
use juniper::graphql_subscription;

use crate::tasks::models::{ButtonChange, ControllerChange};

use super::schema::Context;

pub struct SubscriptionRoot;

type ButtonChangeStream = Pin<Box<dyn Stream<Item=ButtonChange> + Send>>;

#[graphql_subscription(Context = Context)]
impl SubscriptionRoot {
    async fn button_change(context: &Context) -> ButtonChangeStream {
        let mut rx = { context.ctrl_rx.clone().lock().unwrap().clone() };

        let stream = stream! {
            while rx.changed().await.is_ok() {
                let data = rx.borrow().to_owned();

                match data {
                    ControllerChange::ButtonChange(btn) => yield btn
                }
            }
        };

        Box::pin(stream)
    }
}
