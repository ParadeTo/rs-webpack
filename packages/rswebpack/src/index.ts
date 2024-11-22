import {RsWebpack as BindingRsWebpack} from "@rswebpack/binding"
import {SyncHook} from '@rspack/lite-tapable'

abstract class Plugin {
     abstract apply(compiler: Compiler): void
}

export interface Output {
    path: string
    filename: string
}


export interface RawConfig {
    root: string
    entry: string
    output: Output
    plugins: Plugin[]
}

export class Compiler {
    bindingRsWebpack: BindingRsWebpack
    hooks: {
        beforeRun: SyncHook<[Compiler]>;
    }

    constructor(props: RawConfig) {
        this.hooks = {
            beforeRun: new SyncHook(['compiler'])
        }
        const {plugins} = props
        plugins.forEach(plugin => {
            plugin.apply(this)
        })
        this.bindingRsWebpack = new BindingRsWebpack(props)
    }

    run() {
        this.hooks.beforeRun.call(this)
        this.bindingRsWebpack.run()
    }
}