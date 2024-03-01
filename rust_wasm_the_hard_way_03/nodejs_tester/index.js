
async function main() {

    const importObj = {};

    const data = require("fs")
        .readFileSync("../../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_03.wasm");
    const {instance} = await WebAssembly.instantiate(data, importObj);

    // console.log("instance exports", instance.exports);

    let res_0 = instance.exports.nth_prime(0);
    console.log("1st prime number is:", res_0);
    let res_1 = instance.exports.nth_prime(1);
    console.log("2nd prime number is:", res_1);
    let res_2 = instance.exports.nth_prime(2);
    console.log("3rd prime number is:", res_2);
    let res_3 = instance.exports.nth_prime(3);
    console.log("4th prime number is:", res_3);
    let res_4 = instance.exports.nth_prime(4);
    console.log("5th prime number is:", res_4);
}

main();
