const path = require('path')

module.exports = {
  root: path.resolve(__dirname, 'js_code'),
  entry: 'index.js',
  output: {
    path: path.resolve(__dirname, 'out'),
    filename: 'bundle.js',
  },
}
