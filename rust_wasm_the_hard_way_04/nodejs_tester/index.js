
async function main() {

    const data = require("../pkg");
    // console.log("data: {}", data);

    let res_0 = data.add(2, 4);
    console.log("res:", res_0);

    let res_1 = data.upper("let's go!");
    console.log("res:", res_1);
}

main();
