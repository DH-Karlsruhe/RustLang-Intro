const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    // this confi-partern has changed and needs a wrapper..
    new CopyWebpackPlugin({
      patterns: ['index.html']
    })
  ],
  experiments: {
    // WebAssembly on Webpack is still experimental..
    asyncWebAssembly: true
  }
};
