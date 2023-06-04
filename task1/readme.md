## Basic programming concepts

In this task You will have to prevent minor users from accessing blackjack game.

1. Try to build project with `cargo run`. Unfortunately It looks like the other team responsible for developing the game forgot to implement function for randomly drawing a card. Open src/secret/draw.rs and implement a public `draw_card` function that would return a random integer value between 1 and 11 (including 1 and 11). You will need to add rand crate https://docs.rs/rand/latest/rand/ to the project for rng utilities. After this You should be able to run project successfully.
2. Now back to the right problem. Open src/main.rs and in the main function add code that will ask user their age, read their age from standard input (io::stdin) and call blackjack function only when they are at least 18 years old. If they are below 18 the program should terminate and if their input can not be interpreted as an integer the program should query user again.
