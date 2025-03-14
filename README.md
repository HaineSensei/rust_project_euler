# Rust Project Euler

This is my repository for all my Project Euler solutions in Rust. I will gradually add to here. While the UI in [`main.rs`](src/main.rs) and [update tool](`add_problem.rs`) were written by Claude, all individual problem solutions are entirely my work.

## Running

To run a specific problem's solution, simply use cargo run on the full project. This will enter a prompt loop to run solutions by number. This UI in [`main.rs`](src/main.rs) was written almost entirely by Claude.ai since that is not the main purpose of this repository.

For now, the update system for when I add a new problem solution is by adding the appropriate lines to the various files. This process is automated by [`add_problem.rs`](add_problem.rs) which has been compiled with rustc for my Mac to [`add_problem`](add_problem). Again, I let Claude.ai do most of that work for me and just had to correct a few small things.