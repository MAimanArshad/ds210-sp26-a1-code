use analytics_lib::{dataset::Dataset, query::Query};


#[tarpc::service]
pub trait RPCInterface {
    async fn hello() -> String;
    async fn slow_rpc() -> Dataset;
    // async fn fast_rpc(query: Query) -> Dataset;
}
