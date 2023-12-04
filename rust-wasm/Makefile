build:
	wasm-pack build --release --out-name gameplay.wasm --target web --out-dir pkg 
	wasm-opt -Oz -o pkg/opt-gameplay.wasm pkg/gameplay.wasm

	cp pkg/opt-gameplay.wasm ../nextjs-version/public/js/gameplay.wasm
	cp pkg/gameplay.d.ts ../nextjs-version/public/js/
	cp pkg/gameplay.js ../nextjs-version/public/js/
	cp pkg/opt-gameplay.wasm ../nextjs-version/public/gameplay.wasm_bg.wasm

	cp pkg/opt-gameplay.wasm ../react-version/src/js/gameplay.wasm
	cp pkg/gameplay.d.ts ../react-version/src/js/
	cp pkg/gameplay.js ../react-version/src/js/

trace:
	wasm-interp pkg/output.wasm --run-all-exports  --trace > trace.log
	wc -l trace.log

clean:
	rm -rf pkg


