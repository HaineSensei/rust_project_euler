# Rust Project Euler

This is my repository for all my Project Euler solutions in Rust. I will gradually add to here. While the UI in [`main.rs`](src/main.rs) and [update tool](`add_problem.rs`) were written by Claude, all individual problem solutions are entirely my work.

## Running

To run a specific problem's solution, simply use cargo run on the full project. This will enter a prompt loop to run solutions by number. This UI in [`main.rs`](src/main.rs) was written almost entirely by Claude.ai since that is not the main purpose of this repository.

Adding a new problem is handled by [`add_problem.rs`](add_problem.rs), compiled directly with rustc:

```sh
rustc add_problem.rs -o add_problem
./add_problem <number> [status]   # status: complete, inefficient, incomplete (default)
```

This creates a new problem file with a stub `main` and a `STATUS` constant, then runs `cargo build`. A [`build.rs`](build.rs) build script picks up any new or removed files in `src/problems/`, keeps the `pub mod` declarations in [`mod.rs`](src/problems/mod.rs) in sync, and regenerates the problem registry automatically. To update an existing problem's status, run `add_problem` with the same number and the new status.
