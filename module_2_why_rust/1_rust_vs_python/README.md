# Rust vs Python Performance Comparison

This example creates a list/vector with 10,000,000 numbers, sums them up in a loop, and measures the elapsed time.

The Python and Rust code are at `src/example.py` and `src/example.rs`. In addition `src/helpers.py` and `src/helpers.rs` contain the code to generate the large list of numbers.

### Running the Python code

```bash
cd src/
python3 example.py
```


### Running the Rust code

For regular/debug mode:
```bash
cargo run
```

For optimized/release mode (faster code):
```bash
cargo run --release
```

### Looking at the compiled Rust binary

You can compile the code using:
```bash
cargo build
```

Then, you can find the compiled code under `target/debug/` in a file called `example`:
```bash
cd target/debug  # change directory to where the compiled file is
./example            # run compiled file directly
cat example        # print content of compiled file (unintelligible)
```
