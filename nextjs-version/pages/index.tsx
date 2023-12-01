import { useEffect } from "react"

import initGameInstance from "../js/game";
import * as Gameplay from "../js/gameplay";

export type WasmInstance = typeof Gameplay;


export default function Home() {

  useEffect(() => {
    initGameInstance().then((ins: WasmInstance) => {
      console.log("setting instance", ins);
      console.log(ins.add(1, 2));
      console.log(ins.get_position());
      console.log(ins.perform_command(0));
      console.log(ins.get_position());
    });
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
    </main>
  )
}
