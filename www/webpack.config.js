const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: {
    main: "./bootstrap.js",
    index: "./index.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
    publicPath: ''
  },
  mode: "development",
  resolve: {
    extensions: ['.js', '.wasm'],
    fallback: {
      "fs": false,
      "path": false,
      "crypto": false
    }
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async"
      },
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader'
        }
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" },
        { from: "../pkg", to: "pkg" }
      ]
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};