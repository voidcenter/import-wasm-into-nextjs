import { useEffect } from "react";
import initGameInstance from "./js/game";
import * as Gameplay from "./js/gameplay";

export type WasmInstance = typeof Gameplay;



export function App() {

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
    <div></div>
  );
}

