#!/usr/bin/env node
const path = require('path')
const {RsWebpack} = require('@rswebpack/binding')

const argv = require('yargs-parser')(process.argv.slice(2))

const config = require(path.resolve(
  process.cwd(),
  argv.config || 'rswebpack.config.js'
))

const rsWebpack = new RsWebpack(config)
rsWebpack.run()
