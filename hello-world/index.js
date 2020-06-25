// Importando el wasm module
import init from './pkg/hello_world.js'

const runWasm = async () => {
  // Inicializando el modulo wasm
  const helloWorld = await init('./pkg/hello_world_bg.wasm')

  // Llamando a la funcion add exportada desde wasm y guardando el resultado
  const addResult = helloWorld.add(24, 24)

  const result = helloWorld.call_me_from_javascript(24, 24)

  // Añadiendo el resultado dentro del body
  document.body.textContent = `Hola mundo addResult: ${addResult}`

  console.log(result)


  // Primera parte escribiendo desde wasm y leendo desde javascript
  // Escribiendo en el buffer de wasm
  console.log('Write in wasm and read from js, index 0')
  helloWorld.store_value_in_wasm_memory_buffer_index_zero(24)

  // Creando un Unit8Array del wasm buffer
  let wasmMemory = new Uint8Array(helloWorld.memory.buffer)

  // Obtener el puntero del buffer que esta dentro de helloWorld
  let bufferPointer = helloWorld.get_wasm_memory_buffer_pointer()

  // Leendo el valor escrito en el indice 0 del buffer
  console.log(wasmMemory[bufferPointer + 0])

  // Segunda parte escribiendo en js y leendo desde wasm
  console.log('Write in js, read in wasm, index 1')
  // Escribiendo en el indice 1 en el buffer
  wasmMemory[bufferPointer + 1] = 25

  // Leendo desde wasm el buffer en la posicion 1 y retornando el resultado
  console.log(helloWorld.read_wasm_memory_buffer_and_return_index_one())

  /**
  * NOTA: si tuviéramos que seguir leyendo y escribiendo memoria,
  * Dependiendo de cómo crece la memoria en rust, es posible que tenga
  * que volver a crear el Uint8Array ya que el diseño de la memoria podría cambiar.
  * Por ejemplo, `let wasmMemory = new Uint8Array (rustWasm.memory.buffer);`
  * En este ejemplo, no lo hicimos, pero tenga en cuenta que esto puede suceder
   */

   helloWorld.console_log_from_wasm()
}

runWasm()