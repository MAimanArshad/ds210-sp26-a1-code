# Summing up a Matrix in two ways in Rust

This code contains two examples, `rows` and `columns`. Both generate a 10,000 by 10,000 matrix of numbers (10,000,000 elements), sums up all of the matrix content, and measure how long it takes to compute that sum.

Both examples are identical, except `rows` traverese the matrix one row at a time, from top to bottom, going left to right inside every row, while `columns` goes one column at a time, from left to right.

For example, consider this matrix:
```
1   2   3 
4   5   6 
```

`rows` will traverse it as 1 -> 2 -> 3 -> 4 -> 5 -> 6, while `columns` will do 1 -> 4 -> 2 -> 5 -> 3 -> 6.


### Running the code

You can run the example you desire using the corresponding command:
```bash
# to run rows
cargo run --bin rows
# to run columns
cargo run --bin columns
```
