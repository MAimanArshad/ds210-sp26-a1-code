# Homework 3: Guessing Game

### Getting started

Run the simplest version of this game using the following command, then follow the instructions on the screen.

```bash
cargo run --bin game -- --strategy random
```

For example, here is a transcript from when I ran the game. In this run, I chose `6` as my number!
```
Choose a number between 0 (inclusive) and 16 (exclusive).
Write down your number on a piece of paper so you remember what it is.
Do not tell me what your number is. I will try to guess it.
Did you choose a number? Hit enter when you have! 

Commencing game!
Is your number equal to 5 [y/n]? 
n
Is your number equal to 9 [y/n]? 
n
Is your number equal to 4 [y/n]? 
n
Is your number equal to 4 [y/n]? 
n
Is your number equal to 10 [y/n]? 
n
Is your number equal to 12 [y/n]? 
n
Is your number equal to 7 [y/n]? 
n
Is your number equal to 13 [y/n]? 
n
Is your number equal to 4 [y/n]? 
n
Is your number equal to 7 [y/n]? 
n
Is your number equal to 2 [y/n]? 
n
Is your number equal to 2 [y/n]? 
n
Is your number equal to 6 [y/n]? 
y
Final answer: 6
Took 13 steps
```

### Adding your solution

You should add your solution code to `src/part1.rs`, `src/part2.rs`, and `src/part3.rs`.

**Do not change any other files!**. Our auto grader is configured to only use your solution files and will not look at or use any changes you make to other files.

### Running and testing your code

**Parts 1 and 2:** You can run your solutions to part1 and part2 using these commands:
```bash
cargo run --bin game -- --strategy part1
cargo run --bin game -- --strategy part2
```

**Part 3:** After adding your code to part 3, you can run your tests using:
```bash
cargo test --bin game -- --test-threads 1
```

This will run all the tests. You may want to run separate test groups at a time to make it easier to understand what is going on.
In that case, we suggest using these commands one at a time.
```bash
cargo test --bin game bad_strategy -- --test-threads=1
cargo test --bin game part1 -- --test-threads=1
cargo test --bin game part2 -- --test-threads=1
```

**Part 4:** After adding your code to part 3, you can run the experiment using this command. The output plot will be in `plot.png`.
```bash
cargo run --bin experiment
```

### Submitting your solution

Follow the instructions on gradescope. **Make sure you submitted the correct file for each part!**
