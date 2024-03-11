use std::io::Read;

use wasmer::{imports, sys::EngineBuilder, sys::Features, Instance, Module, Store, Value};
use wasmer_compiler_singlepass::Singlepass;

fn main() -> anyhow::Result<()> {
    // Read wasm file
    let wasm_file = "../target/wasm32-unknown-unknown/release/rust_wasmer_02.wasm".to_string();

    println!("Reading wasm file: {}...", wasm_file);
    let mut file = std::fs::File::open(wasm_file)?;
    let mut wasm_content = Vec::new();
    let _wasm_file_size = file.read_to_end(&mut wasm_content)?;
    println!("Wasm file size: {}", _wasm_file_size);

    let compiler = Singlepass::default();
    // Let's declare the features.
    let mut features = Features::new();
    features.bulk_memory(true);
    let engine = EngineBuilder::new(compiler).set_features(Some(features));

    let mut store = Store::new(engine);
    let module = Module::new(&store, &wasm_content)?;

    let import_object = imports! {};

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let param_1_: [u8; 4] = [1, 2, 3, 4];
    let param_3_ = b"hello";
    let mut offset = 0;
    {
        // Write param_1_ to shared memory
        let memory = instance.exports.get_memory("memory")?;
        let memory_view = memory.view(&store);
        memory_view.write(offset, &param_1_[..])?;
    }

    let param_1 = Value::I32(offset as i32); // Memory offset is 0
    let param_2 = Value::I32(i32::try_from(param_1_.len())?);

    offset = param_1_.len() as u64;

    // Write param_3_ to shared memory
    {
        let memory = instance.exports.get_memory("memory")?;
        let memory_view = memory.view(&store);
        memory_view.write(offset, &param_3_[..])?;
    }
    let param_3 = Value::I32(offset as i32);
    let param_4 = Value::I32(i32::try_from(param_3_.len())?);

    let function_sum = instance.exports.get_function("sum")?;
    println!("Calling function: sum...");
    let result = function_sum.call(&mut store, &[param_1.clone(), param_2.clone()])?;

    println!("Sum of {:?} - result: {:?}", param_1_, result);
    assert_eq!(
        result[0].unwrap_i64(),
        i64::from(param_1_.iter().sum::<u8>())
    );

    let function_upper = instance.exports.get_function("uppercase")?;
    println!("Calling function: uppercase...");
    let result2 = function_upper.call(&mut store, &[param_3.clone(), param_4.clone()])?;

    // Note: result2 is just an offset in memory
    println!("result 2: {:?}", result2);

    // let mut buffer = Vec::with_capacity(4);
    let mut buffer = vec![];
    buffer.resize(param_3_.len(), 0);
    let memory = instance.exports.get_memory("memory")?;
    let memory_view = memory.view(&store);
    memory_view.read(result2[0].unwrap_i32() as u64, &mut buffer)?;

    let message_up = String::from_utf8(buffer)?;
    println!(
        "result 2 - original: {:?} --> new: {}",
        String::from_utf8_lossy(&param_3_[..]),
        message_up
    );

    Ok(())
}
