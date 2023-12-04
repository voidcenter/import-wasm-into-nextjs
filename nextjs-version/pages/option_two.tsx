
import { useEffect } from "react"
// import initGameInstance from "../public/js/gameplay";


async function getWasm() {
  try {
    const res = await fetch("js/gameplay.wasm");
    // bytes from memory
    const buffer = await res.arrayBuffer();
    // this will create an object
    // WebAssembly is part of window api. so make sure you are on client side. 

    const imports = { };
    const wasm = await WebAssembly.instantiate(buffer, imports);
    console.log(wasm);
    return wasm.instance.exports;

  } catch (e) {
    console.log(e);
  }
}


export default function Home() {

    useEffect(() => {
        const run = async () => {
            const wasm = await getWasm();

            console.log(wasm);
            // console.log(wasm.add(10, 20));
            // console.log(wasm.get_position());
        }

        run();
    });


    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
        </main>
    )
}

