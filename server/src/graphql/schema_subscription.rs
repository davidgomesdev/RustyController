use std::pin::Pin;

use juniper::{FieldError, futures};
use juniper::futures::Stream;
use juniper::graphql_subscription;

use super::schema::Context;

pub struct SubscriptionRoot;

type StringStream = Pin<Box<dyn Stream<Item=Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl SubscriptionRoot {
    async fn hello_world(context: &Context) -> StringStream {
        let stream = futures::stream::iter(vec![
            Ok(String::from("Hello")),
            Ok(String::from("World!")),
        ]);
        Box::pin(stream)
    }
}
