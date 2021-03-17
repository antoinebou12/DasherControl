var path = require('path');

//plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { VueLoaderPlugin } = require('vue-loader')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const NODE_ENV = process.env.NODE_ENV;
const IS_PRODUCTION = NODE_ENV === "production";

module.exports = (env, argv) => ({
  mode: argv && argv.mode || 'development',
  entry: './main.js',
  node: false,
  output: {
    path: path.resolve(__dirname, 'dist/public'),
    filename: '[name]-[fullhash].js'
  },
  module: {
    rules: [
      { test: /\.js$/, exclude: /node_modules/,  loader: 'babel-loader', options: { cacheDirectory: true } },
      { test: /\.vue$/, use: ['vue-loader'] },
      { test: /\.css$/, use: ['vue-style-loader', 'css-loader'] },
      { test: /\.scss$/, use: ['vue-style-loader', 'css-loader', 'sass-loader'] }
    ]
  },
  resolve: {
    extensions: [
      '.js',
      '.vue',
      '.json'
    ],
    alias: {
      'vue$': 'vue/dist/vue.esm.js',
      '@': path.resolve(__dirname, './')
    }
  },
  devServer: {
      host: '0.0.0.0',
      port: '8080',
      open: true,
      hot: true,
      compress: true,
      overlay: true,
      https: true,
  },
  
  devtool: (argv && argv.mode || 'development') === 'production' ? 'source-map' : 'eval',
  plugins: [
    new CleanWebpackPlugin({
        cleanAfterEveryBuildPatterns: ['dist']
    }),
    new CopyWebpackPlugin({
      patterns: [{
          from: path.resolve(__dirname, 'public'),
          to: path.resolve(__dirname, 'dist/public'),
          globOptions: {
              ignore: [path.resolve(__dirname, 'public/index*')]
          },
          toType: 'dir'
      },
      {
        from: path.resolve(__dirname, 'templates'),
        to: path.resolve(__dirname, 'dist/templates'),
        globOptions: {
          ignore: [path.resolve(__dirname, 'templates/base*')]
      },
        toType: 'dir'
    }]
  }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'templates', 'base.html.tera'),
      filename: path.resolve(__dirname, 'dist/templates', 'base.html.tera'),
      inject: true
    }),
    new VueLoaderPlugin(),
  ],
  optimization: {
    runtimeChunk: false,
    splitChunks: {
      cacheGroups: {
    }
  }
}
});
