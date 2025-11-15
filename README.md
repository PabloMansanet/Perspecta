# Perspecta

A Bevy-based tool to help artists learn to draw with perspective exercises.

## About

Perspecta is an interactive learning tool that offers various exercises to help artists improve their drawing skills:

- **Memory games**: Remember a 3D figure for one minute, then draw it
- **Perspective exercises**: Different ways to view 3D models in perspective
- **Lighting studies**: Learn how light affects form and shadow

## Requirements

- [Rust](https://rustup.rs/) (latest stable version)
- For WASM builds:
  - `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
  - `wasm-bindgen-cli`: `cargo install wasm-bindgen-cli`

## Running Locally

To run the application natively:

```bash
cargo run
```

For faster development builds with better performance:

```bash
cargo run --release
```

## Building for WASM

To build for WebAssembly:

```bash
./build_wasm.sh
```

Then serve the `wasm` directory:

```bash
python3 -m http.server --directory wasm 8080
```

Open your browser to `http://localhost:8080`

## Project Structure

```
perspecta/
├── src/
│   └── main.rs          # Main application entry point
├── wasm/
│   └── index.html       # HTML wrapper for WASM build
├── Cargo.toml           # Project dependencies
└── build_wasm.sh        # WASM build script
```

## Development

This project uses Bevy 0.15, a data-driven game engine built in Rust. The current version includes a simple 3D scene with a spinning quad as a starting point.

## License

TBD
