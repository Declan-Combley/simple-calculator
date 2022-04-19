# Simple Calculator
This simple calculator takes only three arguments, two numbers and an operator. To run it either ./ into the target binary or by using cargo.

<br>

```bash
cargo run <first-number> <operator> <second-number> <...>

# If you want to run a release version first run 
cargo build --release

./target/release/calculator <first-number> <operator> <second-number>
```
