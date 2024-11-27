module.exports = class MyPlugin {
    apply(compiler) {
        compiler.hooks.beforeRun.tap('myplugin', (compiler) => {
            console.log("before run", compiler)
        })
    }
}