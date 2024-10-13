# WASM Test: Celsius to Fahrenheit Converter

This project demonstrates how to create a simple WebAssembly module using AssemblyScript and how to use it from a Rust program. The WebAssembly module contains functions to add two numbers and convert Celsius to Fahrenheit.

## Project Structure

- `celsius-converter/`: AssemblyScript project that compiles to WebAssembly
- `wasm-celsius-converter/`: Rust project that loads and runs the WebAssembly module

## Prerequisites

- Node.js and npm
- Rust and Cargo

## Setup and Running

### 1. Compile WebAssembly Module

First, navigate to the AssemblyScript project directory and compile the WebAssembly module:

```bash
cd celsius-converter
npm install
npm run asbuild
```

This will create the WebAssembly files in the `build` directory.

### 2. Run Rust Program

Next, navigate to the Rust project directory and run the program:

```bash
cd ../wasm-celsius-converter
cargo run
```

This will compile and run the Rust program, which loads the WebAssembly module and executes its functions.

## Expected Output

If everything is set up correctly, you should see output similar to:

```
WebAssembly file read successfully. Size: [size] bytes
WebAssembly module compiled successfully
WebAssembly module instantiated successfully
5 + 37 = 42
100°C is 212°F
```

## Troubleshooting

If you encounter any issues:

1. Ensure that you have the latest versions of Node.js, npm, Rust, and Cargo installed.
2. Check that the path to the WebAssembly file in the Rust code (`src/main.rs`) matches your project structure.
3. Make sure you've run `npm run asbuild` in the `celsius-converter` directory before running the Rust program.

## Further Development

Feel free to modify the AssemblyScript code in `celsius-converter/celsius-to-fahrenheit.ts` to add more functions, and update the Rust code in `wasm-celsius-converter/src/main.rs` to use these new functions.
