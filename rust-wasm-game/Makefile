build:
	wasm-pack build --release --out-name server.wasm --target web --out-dir pkg 
	wasm-opt -Oz -o pkg/opt-server.wasm pkg/server.wasm
	cp pkg/opt-server.wasm ../../public/wasm-server/server.wasm
	cp pkg/server.d.ts ../../public/wasm-server/
	cp pkg/opt-server.wasm ../../../playground/zkwasm/cloud/wasm/server.wasm

trace:
	wasm-interp pkg/output.wasm --run-all-exports  --trace > trace.log
	wc -l trace.log

clean:
	rm -rf pkg

