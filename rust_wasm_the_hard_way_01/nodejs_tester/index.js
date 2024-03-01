
async function main() {

    const importObj = {
        Math: {
            random: () => Math.random(),
        }
    };
    // Note: this works too
    // const importObj = { Math };

    const data = require("fs")
        .readFileSync("../../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_01.wasm");
    const {instance} = await WebAssembly.instantiate(data, importObj);

    console.log("instance exports", instance.exports);

    let res_1 = instance.exports.add_1(40, 2); // returns 42
    console.log("wasm res 1:", res_1);

    let res_2 = instance.exports.add_2(40.0, 2.0);
    console.log("wasm res 2:", res_2);

    let res_3 = instance.exports.add_3(40.0, 2.0);
    console.log("wasm res 3:", res_3);
}

main();
