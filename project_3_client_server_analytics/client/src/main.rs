extern crate tarpc;

use std::time::Instant;

use analytics_lib::dataset::Value;
use analytics_lib::query::{Aggregation, Condition, Query};
use client::{start_client, solution};

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    let rpc_client = start_client().await;

    // Read command line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "hello" {
        solution::run_hello(&rpc_client).await;
        return;
    }
    if args.len() < 3 {
        println!("Usage: cargo run -- [query] [method]");
        println!("  [query]: either grades or albums");
        println!("  [method]: either slow_rpc or fast_rpc");
        return;
    }
    let query = &args[1];
    let method = &args[2];

    // Choose query.
    let query = if query == "grades" {
        // Query for grades file.
        Query::new(
            Condition::Equal(String::from("section"), Value::String(String::from("A1"))),
            String::from("grade"),
            Aggregation::Count(String::from("name")),
        )
    } else if query == "albums" {
        // Query for albums file.
        Query::new(
            Condition::Equal(String::from("band"), Value::String(String::from("Meshuggah"))),
            String::from("album"),
            Aggregation::Average(String::from("rating")),
        )
    } else {
        panic!("bad choice for query, choose either grades or albums");
    };

    // Choose method.
    let time = Instant::now();
    let dataset = if method == "slow_rpc" {
        solution::run_slow_rpc(&rpc_client, query).await
    } else if method == "fast_rpc" {
        solution::run_fast_rpc(&rpc_client, query).await
    } else {
        panic!("Bad choice for rpc method, choose either fast_rpc or slow_rpc");
    };
    let duration = time.elapsed();

    println!("{}", dataset);
    println!("Query took {:?} to executed", duration);
}