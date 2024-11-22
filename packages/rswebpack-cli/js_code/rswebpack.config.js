const path = require('path')
const MyPlugin = require('./my_plugin')

module.exports = {
  root: path.resolve(__dirname),
  entry: 'index.ts',
  output: {
    path: path.resolve(__dirname, 'out'),
    filename: 'bundle.js',
  },
  plugins: [new MyPlugin()]
}
