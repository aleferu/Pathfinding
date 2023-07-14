# Pathfinding

Visualization of different pathfinding algorithms using Rust + Macroquad library.

## Dependencies
- [Rust](https://www.rust-lang.org/)
- [Macroquad crate](https://macroquad.rs/)
- [Rand crate](https://docs.rs/rand/latest/rand/)

## How to build
Normal Rust compilation. Release is better, but it's fast either way.
```bash
$ cargo build --release
```

Run with the following or just run the binary.
```bash
$ cargo run --release
```

## How to use

- **Left Click** to place a wall in the grid.
- **Right Click** to place the goal in the grid.
- **Middle Click** to place the start in the grid.
- **Left Shift + Left Click** to reset a square in the grid.
- **A Key** to run the *A\* Algorithm* (Goal and Start must be placed).
- **D Key** to run the *Dijkstra Algorithm* (Goal and Start must be placed).
- **G Key** to run the *Greedy Best Algorithm* (Goal and Start must be placed).
- **C Key** to clear the grid.
- **M Key** to generate a random maze.
- **Left and Right Arrow Keys** to visualizate how the algorithm worked (deletes walls placed after the algorithm worked).
