use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn row_matches_condition(dataset: &Dataset, row: &Row, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(column_name, expected_value) => {
            let column_index = dataset.column_index(column_name);
            row.get_value(column_index) == expected_value
        }
        Condition::Not(inner_condition) => {
            !row_matches_condition(dataset, row, inner_condition)
        }
        Condition::And(left_condition, right_condition) => {
            row_matches_condition(dataset, row, left_condition)
                && row_matches_condition(dataset, row, right_condition)
        }
        Condition::Or(left_condition, right_condition) => {
            row_matches_condition(dataset, row, left_condition)
                || row_matches_condition(dataset, row, right_condition)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered_dataset = Dataset::new(dataset.columns().clone());

    for row in dataset.iter() {
        if row_matches_condition(dataset, row, filter) {
            filtered_dataset.add_row(row.clone());
        }
    }

    return filtered_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let group_by_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();

    let mut grouped: HashMap<Value, Dataset> = HashMap::new();

    for row in dataset.into_iter() {
        let group_value = row.get_value(group_by_index).clone();

        grouped
            .entry(group_value)
            .or_insert_with(|| Dataset::new(columns.clone()))
            .add_row(row);
    }

    return grouped;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}