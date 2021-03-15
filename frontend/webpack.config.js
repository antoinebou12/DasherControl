var path = require('path');

//plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { VueLoaderPlugin } = require('vue-loader')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

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
      { test: /\.js$/, use: 'babel-loader' },
      { test: /\.vue$/, use: 'vue-loader' },
      { test: /\.css$/, use: ['vue-style-loader', 'css-loader']},
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
    // splitChunks: {
    //   chunks: 'all',
    //   minSize: 30000,
    //   maxSize: 0,
    //   cacheGroups: {
    //     vendors: {
    //       test: /[\\/]node_modules[\\/]/,
    //       priority: -10
    //     },
    //     default: {
    //       minChunks: 2,
    //       priority: -20,
    //       reuseExistingChunk: true
    //     }
    //   }
    // },
    // runtimeChunk: {
    //   name: entrypoint => `runtime~${entrypoint.name}`
    // },
    // mangleWasmImports: true,
    // removeAvailableModules: true,
    // removeEmptyChunks: true,
    // mergeDuplicateChunks: true
  },
});
