const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: './index.js',
  output: {
    path:path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  mode: 'development',
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.')
    })
  ]
}