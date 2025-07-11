## Project Overview

This is a Rust learning project focused on Apache Arrow, a columnar in-memory analytics framework. The project uses the `arrow` crate to explore Arrow's capabilities for data processing and analytics.

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Run the main application
- `cargo check` - Quick syntax and type checking
- `cargo clippy` - Run linter for code quality
- `cargo fmt` - Format code according to Rust standards

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test

## Dependencies

The project uses Apache Arrow (`arrow` crate version 54.2.1) with the following features enabled:
- `default` - Standard Arrow functionality
- `prettyprint` - Human-readable output formatting
- `ffi` - Foreign Function Interface support for C/C++ interop

## Project Structure

- `src/main.rs` - Entry point (currently minimal Hello World)
- `notes/` - Directory for development notes and documentation
- `Cargo.toml` - Project configuration and dependencies
- `target/` - Build artifacts (auto-generated)