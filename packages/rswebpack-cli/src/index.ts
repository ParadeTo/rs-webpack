import {Compiler} from '@rswebpack/core/src'
import path from 'path'
import YargsParser from 'yargs-parser'


const argv = YargsParser(process.argv.slice(2))

const config = require(path.resolve(
  process.cwd(),
  argv.config || 'js_code/rswebpack.config.js'
))
console.log(config)
const rsWebpack = new Compiler(config)
rsWebpack.run().then(() => {
    console.log('finish')
})
