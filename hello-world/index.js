// Importando el wasm module
import init from './pkg/hello_world.js'

const runWasm = async () => {
  // Inicializando el modulo wasm
  const helloWWord = await init('./pkg/hello_world_bg.wasm')

  // Llamando a la funcion add exportada desde wasm y guardando el resultado
  const addResult = helloWWord.add(24, 24)

  // Añadiendo el resultado dentro del body
  document.body.textContent = `Hola mundo addResult: ${addResult}`
}

runWasm()