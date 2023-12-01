build:
	wasm-pack build --release --out-name gameplay.wasm --target web --out-dir pkg 
	wasm-opt -Oz -o pkg/opt-gameplay.wasm pkg/gameplay.wasm
	cp pkg/opt-gameplay.wasm ../nextjs-version/js/gameplay.wasm
	cp pkg/gameplay.d.ts ../nextjs-version/js/
	cp pkg/opt-gameplay.wasm ../react-version/src/js/gameplay.wasm
	cp pkg/gameplay.d.ts ../react-version/src/js/

trace:
	wasm-interp pkg/output.wasm --run-all-exports  --trace > trace.log
	wc -l trace.log

clean:
	rm -rf pkg

