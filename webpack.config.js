const HtmlWebpackPlugin = require('html-webpack-plugin');
const VueLoaderPlugin = require('vue-loader/lib/plugin');

module.exports = {
  entry: './frontend/main.js',
  module: {
    rules: [
      { test: /\.js$/, use: 'babel-loader' },
      { test: /\.vue$/, use: 'vue-loader' },
      { test: /\.css$/, use: ['vue-style-loader', 'css-loader']},
    ]
  },
  devServer: {
      host: '0.0.0.0',
      port: '8080',
      open: true,
      hot: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './frontend/index.html',
    }),
    new VueLoaderPlugin(),
  ]
};