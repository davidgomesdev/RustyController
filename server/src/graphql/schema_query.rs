use juniper::FieldResult;

use crate::graphql::schema::Context;
use crate::graphql::schema_response::HealthStatus;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Check the service health")]
    fn health(_ctx: &Context) -> FieldResult<HealthStatus> {
        Ok(HealthStatus::Ok)
    }
}
