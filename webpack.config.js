const webpack = require('webpack');
const path = require('path');
module.exports = {
  entry: "./index.js",
  resolve: {
    extensions: [".js", ".wasm"],
  },
  plugins: [
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  node: {
    fs: "empty"
  }
};
