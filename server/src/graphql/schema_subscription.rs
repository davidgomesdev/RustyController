use std::pin::Pin;

use async_stream::stream;
use juniper::futures::Stream;
use juniper::graphql_subscription;

use crate::tasks::models::{Button, ControllerChange::ButtonPressed};

use super::schema::Context;

pub struct SubscriptionRoot;

type StringStream = Pin<Box<dyn Stream<Item=Button> + Send>>;

#[graphql_subscription(Context = Context)]
impl SubscriptionRoot {
    async fn hello_world(context: &Context) -> StringStream {
        let mut rx = { context.ctrl_rx.clone().lock().unwrap().clone() };

        let stream = stream! {
            while rx.changed().await.is_ok() {
                let data = rx.borrow().to_owned();

                match data {
                    ButtonPressed(btn) => yield btn
                }
            }
        };

        Box::pin(stream)
    }
}
