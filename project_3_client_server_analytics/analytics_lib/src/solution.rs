use std::collections::HashMap;
use crate::dataset::{self, ColumnType, Dataset, Row, Value};
use crate::query::{Aggregation, Condition, Query};

pub fn row_match_condition(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    //Helper function to check if a row matches a condition
    match condition {
        Condition::Equal(column, expected_value) => {
            let index = dataset.column_index(column);
            return row.get_value(index) == expected_value;
        }
        Condition::Not(inner_condition) => {
            return !row_match_condition(row, dataset, inner_condition);
        }
        Condition::And(left_condition, right_condition) => {
            return row_match_condition(row, dataset, left_condition) && row_match_condition(row, dataset, right_condition);
        }
        Condition::Or(left_condition, right_condition) => {
            return row_match_condition(row, dataset, left_condition) || row_match_condition(row, dataset, right_condition);
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();
    for (grouped_value, dataset) in dataset {
        match aggregation {
            Aggregation::Count(column_name) => {
                result.insert(grouped_value, Value::Integer(dataset.len() as i32));
            }
            Aggregation::Sum(column_name) => {
                let index = dataset.column_index(column_name);
                let mut sum = 0;
                for row in dataset.iter() {
                    match row.get_value(index) {
                        Value::Integer(value) => sum += value,
                        _ => panic!("Cannot sum non-integer value!"),
                    }
                }
                result.insert(grouped_value, Value::Integer(sum));
            }
            Aggregation::Average(column_name) => {
                let index = dataset.column_index(column_name);
                let mut sum = 0;
                let mut count = 0;
                for row in dataset.iter() {
                    match row.get_value(index) {
                        Value::Integer(value) => {
                            sum += value;
                            count += 1;
                        }
                        _ => panic!("Cannot average non-integer value!"),
                    }
                }
                if count == 0 {
                    result.insert(grouped_value, Value::Integer(0));
                } else {
                    result.insert(grouped_value, Value::Integer(sum / count));
                }
            }
        }
    }
    return result;
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