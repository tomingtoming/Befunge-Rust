# Befunge-Rust ![GitHub Actions](https://github.com/tomingtoming/Befunge-Rust/actions/workflows/rust.yml/badge.svg)

A Befunge-93 interpreter implementation in Rust. This project provides a fast and reliable way to execute Befunge programs while leveraging Rust's safety and performance features.

## About Befunge

Befunge is an esoteric programming language created in 1993 by Chris Pressey. It features a two-dimensional grid where program execution can move in any direction (up, down, left, right). This makes it unique among programming languages and particularly interesting for exploring unconventional programming concepts.

## Features

- Core Befunge-93 instruction support
- Fixed 80x25 toroidal program space
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

You can run a Befunge program by providing the path to your Befunge program file. The file path argument is required:

```bash
cargo run -- path/to/your/befunge/program  # Required: program file path
```

If no file path is provided, or if the program file is unreadable or empty, the interpreter will exit with an error. Division and modulo by zero are reported as runtime errors.

## Compatibility Notes

- Source files are loaded into a fixed 80x25 torus. Programs exceeding 80 columns or 25 rows are rejected.
- This implementation treats division and modulo by zero as runtime errors instead of using interactive or implementation-defined behavior.

## Example Befunge Programs

### Hello World
```befunge
>              v
v  ,,,,,"Hello"<
>48*,          v
v,,,,,,"World!"<
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
4. Run the same checks used in CI:
   ```bash
   cargo fmt --all -- --check
   cargo clippy -- -D warnings
   ```

The primary development branch is `master`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

- [toming](https://github.com/tomingtoming)
