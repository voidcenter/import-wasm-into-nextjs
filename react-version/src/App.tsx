import { useEffect } from "react";
import initGameInstance from "./js/game";

import wasmInit, {
  // add_wasm_by_example_to_string,
  // initSync
    create_game
} from "./js/gameplay.js";


export function App() {

    useEffect(() => {

        const run = async () => {

            console.log('test');

            // Instantiate our wasm module
            const rustWasm = await wasmInit("js/gameplay.wasm");
            // initSync();

            // // Call our exported function
            // const helloString = add_wasm_by_example_to_string("Hello from ");

            // // Log the result to the console
            // console.log(helloString);

            const r = create_game();
            console.log(r);
        };

        run();

    });


  // useEffect(() => {
  //   initGameInstance().then((ins) => {
  //     console.log("setting instance", ins);
  //     console.log(ins.add(1, 2));
  //     console.log(ins.get_position());
  //     console.log(ins.perform_command(0));
  //     console.log(ins.get_position());
  //     console.log(ins.hello2("human"));
  //     console.log(ins.hello3("human"));

  //     const x = Number(ins.hello3("human"));
  //     const a = Math.floor(x / 100000);
  //     const l = x % 100000;
  //     console.log(a, l);

  //     const linearMemory = (ins as any).memory;
  //     // const stringBuffer = new Uint8Array(linearMemory.buffer, a, l);
  //     const stringBuffer = new Uint8Array(linearMemory.buffer, a, l);
      
  //     let str = new TextDecoder().decode(stringBuffer);
  //     console.log(str);


  //   });
  // }, []);

  return (
    <div></div>
  );
}




// import * as Gameplay from "./js/gameplay";
// export type WasmInstance = typeof Gameplay;

