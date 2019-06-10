const path = require('path')
const CopyWebpackPlugin = require("copy-webpack-plugin")
const { CleanWebpackPlugin } = require('clean-webpack-plugin')

const webpackConfig = {
  entry: './bootstrap.js',
  plugins: [
    new CopyWebpackPlugin(['./index.html']),
    new CleanWebpackPlugin(),
  ],
  output: {
    filename: 'bootstrap.js',
    path: path.resolve(__dirname, "dist"),
  },
}

module.exports = webpackConfig
