# Befunge-Rust ![GitHub Actions](https://github.com/tomingtoming/Befunge-Rust/actions/workflows/rust.yml/badge.svg)

A Befunge-93 interpreter implementation in Rust. This project provides a fast and reliable way to execute Befunge programs while leveraging Rust's safety and performance features.

## About Befunge

Befunge is an esoteric programming language created in 1993 by Chris Pressey. It features a two-dimensional grid where program execution can move in any direction (up, down, left, right). This makes it unique among programming languages and particularly interesting for exploring unconventional programming concepts.

## Features

- Full Befunge-93 specification support
- 2D program space navigation
- Stack-based operations
- Random direction execution
- Input/Output operations

## Installation

Make sure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/tomingtoming/Befunge-Rust.git
cd Befunge-Rust

# Build the project
cargo build --release
```

## Usage

You can run a Befunge program by providing the path to your Befunge program file:

```bash
cargo run -- path/to/your/befunge/program         # Run program normally
cargo run -- path/to/your/befunge/program -d      # Run with debug mode
cargo run -- path/to/your/befunge/program --debug # Run with debug mode
```

If no file path is provided, the program will exit with an error.

### Debug Mode

When running with the `-d` or `--debug` flag, the interpreter will run in step-by-step debug mode. In this mode:
- Each instruction step is displayed with detailed information
- Shows current position, instruction, direction, and stack state
- Displays the program grid with current position highlighted
- Execution pauses after each step (press Enter to continue)

This mode is useful for:
- Learning how Befunge programs work
- Debugging complex Befunge code
- Understanding program flow and stack operations

## Example Befunge Programs

### Hello World
```befunge
>              v
v  ,,,,,\"Hello\"<
>48*,          v
v,,,,,,\"World!\"<
>25*,@
```

### Factorial Calculator
```befunge
5 100p:v
v *g00:_00g.@
>00p1-:^
```

## Development

This project uses GitHub Actions for CI/CD with the following checks:
- Cross-platform testing (Linux, Windows, macOS)
- Code formatting with `rustfmt`
- Linting with `clippy`

To set up the development environment:

1. Install Rust toolchain
2. Install development dependencies:
   ```bash
   rustup component add rustfmt clippy
   ```
3. Run tests:
   ```bash
   cargo test
   ```

The primary development branch is `master`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

- [toming](https://github.com/tomingtoming)
