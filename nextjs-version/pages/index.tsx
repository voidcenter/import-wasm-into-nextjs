import { useEffect } from "react"
// import initGameInstance from "../public/js/gameplay";

import wasmInit, {
  add_wasm_by_example_to_string,
} from "../public/js/gameplay.js";


export default function Home() {

  useEffect(() => {

      const run = async () => {

          console.log('test');

          // Instantiate our wasm module
          const rustWasm = await wasmInit("js/gameplay.wasm");
          // initSync();

          // Call our exported function
          const helloString = add_wasm_by_example_to_string("Hello from ");

          // Log the result to the console
          console.log(helloString);
      };

      run();

  });


    // useEffect(() => {
    //     initGameInstance().then((ins) => {
    //       console.log("setting instance", ins);
    //       console.log(ins.add(1, 2));
    //       console.log(ins.get_position());
    //       console.log(ins.perform_command(0));
    //       console.log(ins.get_position());
    //     });
    // }, []);

    // useEffect(() => {
    //     const run = async () => {
    //         const wasm = await getWasm();

    //         // console.log(wasm.add(10, 20));
    //         // console.log(wasm.get_position());
    //         // wasm.perform_command(1);
    //         // console.log(wasm.get_position());
    //         console.log(wasm.get_position_int());
    //         // console.log(wasm.get_position_str());

    //         console.log(wasm.get_hello());
    //         console.log(wasm.get_hello_len());

    //         const linearMemory = (wasm as any).memory;
    //         console.log(linearMemory);

    //         // create a buffer starting at the reference to the exported string
    //         const offset = wasm.get_hello();
    //         const stringBuffer = new Uint8Array(linearMemory.buffer, offset, wasm.get_hello_len());

    //         // create a string from this buffer
    //         // let str = '';
    //         // for (let i=0; i<stringBuffer.length; i++) {
    //         //   str += String.fromCharCode(stringBuffer[i]);
    //         // }

    //         let str = new TextDecoder().decode(stringBuffer);

    //         console.log(str);

    //     }

    //     run();
    // });

    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
        </main>
    )
}


// async function getWasm() {
//   try {
//     const res = await fetch("js/gameplay.wasm");
//     // bytes from memory
//     const buffer = await res.arrayBuffer();
//     // this will create an object
//     // WebAssembly is part of window api. so make sure you are on client side. 

//     const imports = { };
//     const wasm = await WebAssembly.instantiate(buffer, imports);
//     console.log(wasm);
//     return wasm.instance.exports as WasmInstance;

//   } catch (e) {
//     console.log(e);
//   }
// }

