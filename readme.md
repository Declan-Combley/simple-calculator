# Simple Calculator
This calculator takes only floats, integers and operators, no algebra yet. Use cargo to run it with what you want to calculate as an argument, just be sure not to use * for multiplication (it will panic).

<br>

```bash
cargo run <first-number> <operator> <second-number> <etc...>

# If you want to run a release version first run 
cargo build --release

./target/release/calculator <math>
```
