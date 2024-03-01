
async function main() {

    /*
    const importObj = {};

    const data = require("fs")
        .readFileSync("../../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_04.wasm");
    const {instance} = await WebAssembly.instantiate(data, importObj);
    */

    const data = require("../pkg");
    // console.log("data: {}", data);

    let res_0 = data.add(2, 4);
    //let res_0 = instance.exports.add(2, 4);
    console.log("res:", res_0);
}

main();
