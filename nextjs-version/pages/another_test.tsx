// import { useEffect } from "react"
// import initGameInstance from "../public/js/gameplay";
// import dynamic from 'next/dynamic'


// // const RustComponent = dynamic({
// //     loader: async () => {
// //         // Import the wasm module
// //         // @ts-ignore
// //         const exports = (await import('../public/js/gameplay.wasm'))
// //         const { add } = exports
    
// //         console.log(add(1, 2));

// //         return <div></div>;
// //     },
// // })



// export interface AddModuleExports {
//     add(a: Number, b: Number): Number
//   }
  
//   interface RustComponentProps {
//     number: Number
//   }
  
//   const RustComponent = dynamic({
//     loader: async () => {
//       // Import the wasm module
//       // @ts-ignore
//       const exports = (await import('../public/js/gameplay.wasm')) as AddModuleExports
//       const { add } = exports
  
//       // Return a React component that calls the add_one method on the wasm module
//       return ({ a, b }: RustComponentProps) => (
//         <div>
//           <>{add(a, b)}</>
//         </div>
//       )
//     },
//   })


// export default function Home() {

//     // useEffect(() => {
//     //     initGameInstance().then((ins) => {
//     //       console.log("setting instance", ins);
//     //       console.log(ins.add(1, 2));
//     //       console.log(ins.get_position());
//     //       console.log(ins.perform_command(0));
//     //       console.log(ins.get_position());
//     //     });
//     // }, []);


//     return (
//         <main className="flex min-h-screen flex-col items-center justify-between p-24">
//             <RustComponent number={4}/>
//         </main>
//     )
// }

