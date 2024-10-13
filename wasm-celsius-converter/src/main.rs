use std::fs::read;
use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the Wasm file
    let wasm_bytes = match read("../celsius-converter/build/release.wasm") {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read WebAssembly file: {}", e);
            return Err(Box::new(e));
        }
    };

    println!(
        "WebAssembly file read successfully. Size: {} bytes",
        wasm_bytes.len()
    );

    // Create a store
    let store = Store::default();

    // Compile the Wasm module
    let module = match Module::new(&store, wasm_bytes) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to compile WebAssembly module: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("WebAssembly module compiled successfully");

    // Instantiate the module with empty imports
    let instance = match Instance::new(&module, &imports! {}) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Failed to instantiate WebAssembly module: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("WebAssembly module instantiated successfully");

    // Get the `add` function
    let add = match instance.exports.get_function("add") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to get 'add' function: {}", e);
            return Err(Box::new(e));
        }
    };

    // Call the `add` function
    let result = match add.call(&[Value::I32(5), Value::I32(37)]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to call 'add' function: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("5 + 37 = {}", result[0].unwrap_i32());

    // Get the `celsiusToFahrenheit` function
    let celsius_to_fahrenheit = match instance.exports.get_function("celsiusToFahrenheit") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to get 'celsiusToFahrenheit' function: {}", e);
            return Err(Box::new(e));
        }
    };

    // Call the `celsiusToFahrenheit` function
    let result = match celsius_to_fahrenheit.call(&[Value::F32(100.0)]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to call 'celsiusToFahrenheit' function: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("100°C is {}°F", result[0].unwrap_f32());

    Ok(())
}
