extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::Query;
use client::{start_client, solution};

use analytics_lib::query::Condition;
use analytics_lib::dataset::Value;
use analytics_lib::query::Aggregation;

pub fn strip_parens(s: &str) -> &str {
    //Helper function to strip parenthese from the query
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        let mut depth = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            if depth == 0 && i != s.len() - 1 {
                return s;
            }
        }
        return &s[1..s.len() - 1];
    }
    return s;
}

pub fn split_top_level(s: &str, op: &str) -> Option<(String, String)> {
    //Helper function to split AND/OR
    let mut depth = 0;
    let tokens: Vec<&str> = s.split_whitespace().collect();
    for i in 0..tokens.len() {
        for c in tokens[i].chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
        }
        if depth == 0 && tokens[i] == op {
            let left = tokens[..i].join(" ");
            let right = tokens[i + 1..].join(" ");
            return Some((left, right));
        }
    }
    return None;
}

pub fn parse_condition(s: &str) -> Condition {
    //Helper function for filter
    let s = strip_parens(s.trim());
    // 1. OR
    if let Some((left, right)) = split_top_level(s, "OR") {
        return Condition::Or(
            Box::new(parse_condition(&left)),
            Box::new(parse_condition(&right)),
        );
    }
    // 2. AND
    if let Some((left, right)) = split_top_level(s, "AND") {
        return Condition::And(
            Box::new(parse_condition(&left)),
            Box::new(parse_condition(&right)),
        );
    }
    // 3. NOT
    if s.starts_with("!") {
        return Condition::Not(Box::new(parse_condition(&s[1..])));
    }
    // 4. EQUAL
    let parts: Vec<&str> = s.split("==").collect();
    if parts.len() != 2 {
        panic!("Invalid condition format");
    }
    let column = parts[0].trim().to_string();
    let raw_value = parts[1].trim();
    let value = if raw_value.starts_with('"') && raw_value.ends_with('"') {
        Value::String(raw_value[1..raw_value.len() - 1].to_string())
    } else if let Ok(int_value) = raw_value.parse::<i32>() {
        Value::Integer(int_value)
    } else {
        panic!("Invalid value format");
    };
    return Condition::Equal(column, value);
}

// Your solution goes here.
pub fn parse_query_from_string(input: String) -> Query {
    let parts: Vec<&str> = input.split("GROUP BY").collect();
    if parts.len() != 2 {
        panic!("Invalid query format");
    }
    // FILTER
    let filter_part = parts[0].trim();
    let filter_str = filter_part
        .strip_prefix("FILTER")
        .expect("Missing FILTER")
        .trim();
    let filter = parse_condition(filter_str);
    // GROUP BY + AGG
    let tokens: Vec<&str> = parts[1].trim().split_whitespace().collect();
    if tokens.len() != 3 {
        panic!("Invalid GROUP BY format");
    }
    let group_by = tokens[0].to_string();
    let aggregate = match tokens[1] {
        "COUNT" => Aggregation::Count(tokens[2].to_string()),
        "SUM" => Aggregation::Sum(tokens[2].to_string()),
        "AVERAGE" => Aggregation::Average(tokens[2].to_string()),
        _ => panic!("Invalid aggregation"),
    };
    return Query::new(filter, group_by, aggregate);
}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}