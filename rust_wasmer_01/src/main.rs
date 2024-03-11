use std::io::Read;

use rand::prelude::*;
use wasmer::{
    imports, sys::EngineBuilder, sys::Features, Function, FunctionEnv, FunctionEnvMut, Instance,
    Module, Store, Value,
};
use wasmer_compiler_singlepass::Singlepass;

fn main() -> anyhow::Result<()> {
    // Use StdRng here as we want a Rng that impl Send + Sync
    let rng = StdRng::from_entropy();

    // Read wasm file
    let wasm_file =
        "../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_01.wasm".to_string();
    let mut file = std::fs::File::open(wasm_file)?;
    let mut wasm_content = Vec::new();
    let _wasm_file_size = file.read_to_end(&mut wasm_content)?;

    // Use the Singlepass compiler here
    // The engine is a system that uses a compiler to make a WebAssembly module executable.
    // Wasmer can use several compiler
    // Singlepass provides a fast compilation-time but an unoptimized runtime speed
    let compiler = Singlepass::default();
    // Let's declare the features.
    let mut features = Features::new();
    // Enable the bulk memory feature.
    features.bulk_memory(true);
    // Set up the engine. That's where we define the features!
    let engine = EngineBuilder::new(compiler).set_features(Some(features));

    let mut store = Store::new(engine);
    let module = Module::new(&store, &wasm_content)?;

    // Our wasm requires an external function (a Rust function): Math.random
    // (see rust_wasm_the_hard_way_01 lib.rs)
    // From: https://github.com/wasmerio/wasmer/blob/master/examples/imports_function_env.rs
    // Create a struct FunctionEnv that wraps our rng then it will be passed to our import function
    let env = FunctionEnv::new(&mut store, Env { rng });
    let import_object = imports! {
        "Math" => {
            "random" => Function::new_typed_with_env(&mut store, &env, random),
        }
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_1")?;
    println!("Calling add_1 function (defined in wasm file) ...");
    let param_1 = Value::I32(42);
    let param_2 = Value::I32(42);
    let result = add_one.call(&mut store, &[param_1.clone(), param_2.clone()])?;
    println!("add_1({:?}, {:?}) == {:?}", param_1, param_2, result);
    assert_eq!(result[0].unwrap_i32(), Value::I32(42 + 42).unwrap_i32());

    let param_3 = Value::F64(42.0);
    let param_4 = Value::F64(42.0);
    let add_two = instance.exports.get_function("add_2")?;
    println!("Calling add_2 function (defined in wasm file) ...");

    let result2 = add_two.call(&mut store, &[param_3.clone(), param_4.clone()])?;
    println!("add_2({:?}, {:?}) == {:?}", param_3, param_4, result2);
    assert!(result2[0].unwrap_f64() > Value::F64(42.0 + 42.0).unwrap_f64());

    Ok(())
}

#[derive(Clone)]
struct Env {
    rng: StdRng,
}

fn random(mut env: FunctionEnvMut<Env>) -> f64 {
    env.as_mut().data_mut().rng.gen()
}
