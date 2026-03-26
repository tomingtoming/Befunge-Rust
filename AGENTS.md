# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` is the CLI entry point: it validates the required file path, loads the Befunge source, and wires stdin/stdout. `src/befunge.rs` contains the interpreter core, instruction handling, and most behavior tests. `src/world.rs` models the 2D playfield and includes parsing and grid-level tests. There is no separate `tests/` or assets directory today, so keep new tests close to the module they cover.

## Build, Test, and Development Commands
Use Cargo for all local work:

- `cargo build`: debug build for normal development.
- `cargo build --release`: optimized binary build.
- `cargo run -- path/to/program.bf`: run a Befunge-93 program; the file argument is required.
- `cargo test`: run the full unit test suite.
- `cargo fmt --all`: apply standard Rust formatting.
- `cargo fmt --all -- --check`: verify formatting without editing files.
- `cargo clippy -- -D warnings`: enforce the same zero-warning lint gate used in CI.

GitHub Actions in `.github/workflows/rust.yml` runs `test`, `fmt --check`, and `clippy` on pushes and pull requests to `master`.

## Coding Style & Naming Conventions
Follow standard Rust style and let `rustfmt` decide formatting: 4-space indentation, formatter-managed line breaks, and grouped imports where appropriate. Use `snake_case` for functions, modules, locals, and tests, and `CamelCase` for structs and enums such as `World`, `Befunge`, and `Direction`. Keep interpreter behavior in `src/befunge.rs` and grid/storage behavior in `src/world.rs`. Prefer `Result`-based error propagation and do not introduce new Clippy warnings.

## Testing Guidelines
Tests live inline under `#[cfg(test)]`. Add coverage in the same file as the code you change. Test names should describe the behavior under test, for example `factorial_of_5` or `memory_operation_commands`. For interpreter changes, use compact Befunge program strings and assert on output, stack contents, or world state.

## Commit & Pull Request Guidelines
Recent commits use short, imperative subjects such as `Fix stdin handling by using BufReader` and `Update README badge from CircleCI to GitHub Actions`. Keep commit titles concise, capitalized, and without a trailing period. Pull requests should target `master`, summarize the behavioral change, mention added or updated tests, and link the related issue when applicable. If output changes, include a minimal Befunge example or expected output in the PR description.
