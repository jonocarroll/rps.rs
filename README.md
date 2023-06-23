# rps

Play rock 🪨, paper 🧻, scissors ✂️ against the computer

Demonstrates uses of `enum`

## Examples
```rust
cargo run -q
Let's play! 🪨🧻✂️
What do you throw?
rock
You threw... Rock 🪨
Computer throws Rock 🪨
Result: It's a tie! 👔
Press ENTER to play again, or anything else to quit

Let's play! 🪨🧻✂️
What do you throw?
paper
You threw... Paper 🧻
Computer throws Scissors ✂️
Result: Sorry, you lose 😿
Press ENTER to play again, or anything else to quit

Let's play! 🪨🧻✂️
What do you throw?
scissors
You threw... Scissors ✂️
Computer throws Paper 🧻
Result: You win, congrats! 🎉
Press ENTER to play again, or anything else to quit
```

A single argument can also be passed to play just once

```rust
cargo run -q -- rock
Let's play 🪨🧻✂️!
You threw... Rock 🪨
Computer throws Rock 🪨
Result: It's a tie! 👔
```

