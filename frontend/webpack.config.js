const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: path.resolve(__dirname, 'index.js'),
  mode: 'development',
  plugins: [
    new HtmlWebpackPlugin(),
  ]
}