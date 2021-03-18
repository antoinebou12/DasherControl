var path = require('path');

//plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { VueLoaderPlugin } = require('vue-loader')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const ImageMinimizerPlugin = require('image-minimizer-webpack-plugin');

const NODE_ENV = process.env.NODE_ENV;
const IS_PRODUCTION = NODE_ENV === "production";

module.exports = (env, argv) => ({
  mode: argv && argv.mode || 'development',
  entry: './main.js',
  node: false,
  output: {
    path: path.resolve(__dirname, 'dist/public'),
    filename: '[name]-[fullhash].js',
    publicPath: '/public'
  },
  module: {
    rules: [
      { test: /\.js$/, exclude: /node_modules/,  loader: 'babel-loader', options: { cacheDirectory: true } },
      { test: /\.vue$/, use: ['vue-loader'] },
      { test: /\.css$/, use: ['vue-style-loader', 'css-loader'] },
      { test: /\.scss$/, use: ['vue-style-loader', 'css-loader', 'sass-loader'] },
      {
        test: /\.(png|jpe?g|gif|svg|ico)(\?.*)?$/,
        use: {
          loader: 'file-loader',
          options: {
            name: `public/imgs/[name].[ext]`,
          },
        },
      },
      {
        test: /\.(woff2?|eot|ttf|otf)(\?.*)?$/,
        use: {
          loader: 'file-loader',
          options: {
            name: `public/fonts/[name].[ext]`,
          },
        },
      },
    ]
  },
  resolve: {
    extensions: ['.js', '.scss', '.vue'],
    alias: {
      'vue$': 'vue/dist/vue.esm.js',
      '@': path.resolve(__dirname, './'),
      modules: path.resolve(__dirname, 'node_modules'),
    },
    modules: [
      path.resolve(__dirname, './'),
      path.resolve(__dirname, 'node_modules'),
    ],
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
  
  devtool: (argv && argv.mode || 'development') === 'production' ? 'source-map' : 'eval-cheap-source-map',
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
    new ImageMinimizerPlugin({
      minimizerOptions: {
        // Lossless optimization with custom option
        // Feel free to experiment with options for better result for you
        plugins: [
          ['gifsicle', { interlaced: true }],
          ['jpegtran', { progressive: true }],
          ['optipng', { optimizationLevel: 5 }],
          [
            'svgo',
            {
              prefix: 'icon--',
              plugins: [
                // { cleanupIDs: false },
                // { collapseGroups: false },
                // { removeTitle: true },
              ],
            },
          ],
        ],
      },
    }),
  ],
   // splitting out the vendor
   optimization: {
    splitChunks: {
    },
  }
});
