import wasmInit from './pkg/graphics.js'

const runWasm = async () => {
  const rustWams = await wasmInit('./pkg/graphics_bg.wasm')

  const canvasElement = document.querySelector("canvas")

  // Asignando el contexto en el canvas
  const canvasContext = canvasElement.getContext("2d")
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  )

  // Limpiandoo el canvas
  canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height)

  const getDarkValue = () => {
    return Math.floor(Math.random() * 100)
  }

  const getLightValue = () => {
    return Math.floor(Math.random() * 127 ) + 127
  }

  const drawCheckBoard = () => {
    const checkerBoardSize = 20

    // Generando el checkboard en wasm
    rustWams.generate_checker_board(
      getDarkValue(),
      getDarkValue(),
      getDarkValue(),
      getLightValue(),
      getLightValue(),
      getLightValue()
    )

    // Creando un Unit8Array para obtener acceso al wasm memory
    const wasmByteMemoryArray = new Uint8Array(rustWams.memory.buffer)

    const outputPointer = rustWams.get_output_buffer_pointer()

    // Traendo los valores RGBA de salida desde wasm memory
    // Empezando el indice de la salida de memoria desde el puntero
    // 20 * 20 * 4 = checkboard max X * checkerboard max Y * number of pixel properties (R,G.B,A)
    const imageDateArray = wasmByteMemoryArray.slice(
      outputPointer,
      outputPointer + checkerBoardSize * checkerBoardSize * 4
    )

    // Asinando los valores al canvas
    canvasImageData.data.set(imageDateArray)

    //Limpiando el canvas
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height)

    // Colocando el nuevo tablero generado
    canvasContext.putImageData(canvasImageData, 0, 0)
  }

  drawCheckBoard()
  setInterval(() => {
    drawCheckBoard()
  }, 1000)
}

runWasm();