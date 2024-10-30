const {RsWebpack} = require('@rs-webpack/binding')
const config = require('./rswebpack.config')

const rsWebpack = new RsWebpack(config)

rsWebpack.run()
