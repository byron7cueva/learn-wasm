const rustWasm = import('./pkg/index')

rustWasm.then(wasm => {
  wasm.create_stuff()
})