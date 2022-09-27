use juniper::GraphQLEnum;

#[derive(GraphQLEnum)]
pub(super) enum HealthStatus {
    Ok,
    Error,
}

#[derive(GraphQLEnum)]
pub(super) enum MutationResponse {
    Success,
    ServerError,
}
