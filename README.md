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

You can run a Befunge program by providing the path to your Befunge program file. The file path argument is required:

```bash
cargo run -- path/to/your/befunge/program  # Required: program file path
```

If no file path is provided, the program will exit with an error.

## Web Usage
This project can also be used in web browsers through WebAssembly (Wasm).

### Setup
1. Make sure you have Node.js and npm installed
2. Build the Wasm package:
   ```bash
   wasm-pack build
   ```
3. Set up the web application:
   ```bash
   cd www
   npm install
   npm run start
   ```
4. Open http://localhost:8080 in your browser

### Using in Your Web Project
You can import and use the Befunge interpreter in your JavaScript code:

```javascript
import { WebBefunge } from "befunge-rust";

// Create a new interpreter instance with your Befunge program
const interpreter = new WebBefunge(program);

// Set input if needed
interpreter.set_input(input);

// Run the program
const output = await interpreter.run();
```

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
