const fs = require("fs");

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

const lTextDecoder =
    typeof TextDecoder === "undefined"
        ? (0, module.require)("util").TextDecoder
        : TextDecoder;

let cachedTextDecoder = new lTextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true,
});

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const imports = {
    env: {
        abort(_msg, _file, line, column) {
            console.error(
                "abort called at main_wasm.ts:" + line + ":" + column
            );
        },
    },
    __wbindgen_placeholder__: {
        __wbindgen_throw(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
    },
};

async function loadWasm(path: string) {
    const buffer = fs.readFileSync(path);
    const wasmModule = await WebAssembly.instantiate(buffer, imports);
    console.log("Wasm module loaded");
    return wasmModule.instance.exports;
}

// import { greet } from "./../rust/provable_game_logic/pkg";
// console.log(greet());

console.log("Loading Wasm module...");
loadWasm("../wasm/gameplay_bg.wasm")
    .then((exports: any) => {
        console.log(
            "Sanity check",
            exports.create_spin_game_init_args(BigInt(0))
        );
        // function print_game_state(): void {
        //     console.log(`Total steps: ${exports.get_total_steps()}`);
        //     console.log(`Current position: ${exports.get_current_position()}`);
        // }

        // let rl = readline.createInterface({
        //     input: process.stdin,
        //     output: process.stdout,
        // });

        // exports.init_game(BigInt(0), BigInt(0));
        // print_game_state();

        // function gameplay() {
        //     rl.question(
        //         "Use a (left) and d (right) to control the game, q to quit. ",
        //         (answer) => {
        //             switch (answer.toLowerCase()) {
        //                 case "a":
        //                     exports.step(BigInt(0));
        //                     break;
        //                 case "d":
        //                     exports.step(BigInt(1));
        //                     break;
        //                 case "q":
        //                     rl.close();
        //                     process.exit();
        //                     break;
        //                 default:
        //                     console.log("Invalid command!");
        //                     break;
        //             }

        //             print_game_state();

        //             gameplay();
        //         }
        //     );
        // }

        // gameplay();
    })
    .catch((err) => {
        console.error("Failed to load Wasm module:", err);
    });
