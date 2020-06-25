const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = (env, args) => {
  const isProductionMode = (args.mode === 'production')

  return {
    entry: './index.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: isProductionMode ? '[name].[contenthash].js' : '[name].[hash].js'
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.')
      })
    ]
  }
}