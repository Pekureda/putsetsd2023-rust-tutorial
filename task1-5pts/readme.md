# Basic programming concepts

In this task You will have to prevent minor users from accessing blackjack game.

1. If you do not have rust please follow the instructions from official site (https://www.rust-lang.org/learn/get-started) to install it. 
2. Try to build and run project with `cargo run`. You should be able to play blackjack in your command line.
3. Now back to the right problem. Open src/main.rs and in the main function add code that will ask user their age, read their age from standard input and call blackjack function only when they are at least 18 years old. If they are below 18 the program should terminate and if their input can not be interpreted as an integer the program should query user again.

## Tips
- Use `read_line` function of `std::io::Stdin` to get user input.
- Use `match` expression to deal with `Result<T>` values.
- You might want to use `trim` and `parse`.
