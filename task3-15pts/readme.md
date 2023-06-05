# Task 3 - 15pts

In this task you are asked to implement a simple *Russian roulette* like *game*.

Example output:
```
Roulette status:
John is still in game!
Robert has lost the game! Was outed 1 in sequence.
Joe is still in game!
Satoshi is still in game!
Peter has lost the game! Was outed 0 in sequence.
Paul is still in game!
Vladimir has lost the game! Was outed 2 in sequence.
Matthew is still in game!
Chen is still in game!
Isaac is still in game!
Anna has lost the game! Was outed 3 in sequence.
Matilda has lost the game! Was outed 4 in sequence.
Arjun is still in game!
People outed from the game so far: 5
```

The aim of this task is to familiarize you with basic object oriented constructs available in Rust and to make you explore its standard library a bit.

Tasks:
1. Define a `RouletteParticipant` enum which will indicate the state of the players within the game. Enums should contain extra data such as name of the player. If the player was outed then he should have also a field to store the sequence number he was outed. You may use the name of the enum suggested in the `main` function.
2. Implement `new` method to create a new object of struct `Roulette`.
3. Implement `kill` method for the struct `Roulette`. It should randomize `n` roulette participants and change their status from that indicating they're in game to the enum indicating that they're out of the game. 
Use `generate_random_number` function to get a random index. You may need to modify some definitions to make this work (related to borrowing).
4. Implement `Display` trait for `RouletteParticipant` and `Roulette` structs so they could be printed by a macro `println!`.

Rules:
* You're not allowed to implement unsafe code
```rust
unsafe {
...
}
```
* You are not allowed to use `Debug` trait directly or by a macro to implement nice printing
