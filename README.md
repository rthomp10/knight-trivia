# Knight Trivia Game

Welcome to the Knight Trivia Game! Test your knowledge with a variety of trivia questions.

```verilog (verilog has the best colors)
         *_   _   _   _   _   _ *
 ^       | `_' `-' `_' `-' `_' `|       ^
 |       |    Knight Trivia     |       |
 |  (*)  |_   _   _   _   _   _ |  \\^/  |
 | _<\">_ | `_' `-' `_' `-' `_' `| _(#)_ |
o+o \\ / \\0                      0/ \\ / (=)
 0'\\ ^ /\\/                      \\/\\ ^ /`0
   /_^_\\ |                      | /_^_\\
   || || |                      | || ||
   d|_|b_T______________________T_d|_|b
```

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have Rust and Cargo installed on your machine. If you don't have Rust set up, please follow the official guide to install Rust and Cargo at https://www.rust-lang.org/learn/get-started.

### Installing

First, clone the repository to your local machine:

```bash
git clone https://github.com/rthomp10/knight-trivia.git
```

Navigate to the project directory:

```bash
cd knight_trivia
```

Build the project using Cargo:

```bash
cargo build
```

This command compiles the project and generates an executable in the `target/debug` directory.

## Running the Tests

To ensure everything is working as expected, you can run the automated tests:

```bash
cargo test
```

This command will run the tests defined in your Rust files and output the results.

## Playing the Game

To start the game, run the following command:

```bash
cargo run
```

This will compile the project (if not already compiled) and execute the game. Follow the on-screen instructions to play.

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.
