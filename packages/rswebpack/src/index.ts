import {JsCompiler, RegisterJsTapKind, RegisterJsTaps, RsWebpack as BindingRsWebpack} from "@rswebpack/binding"
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
        beforeRun: SyncHook<[JsCompiler]>;
    }
    registers?: RegisterJsTaps;

    constructor(props: RawConfig) {
        this.hooks = {
            beforeRun: new SyncHook(['compiler'])
        }
        const {plugins} = props
        plugins.forEach(plugin => {
            plugin.apply(this)
        })
        this.registers = {
            registerBeforeRunTaps: (stages: Array<number>) => stages.map(stage => {
                    return {
                        function: (compiler: JsCompiler) => {
                            console.log(11111)
                            this.hooks.beforeRun.call(compiler);
                        },
                        stage
                    }
                }
            )
        }
        this.bindingRsWebpack = new BindingRsWebpack(props, this.registers)
        this.bindingRsWebpack.setNonSkippableRegisters([RegisterJsTapKind.BeforeRun]);
        // for (const { getHook, getHookMap, registerKind } of Object.values(
        //     this.registers!
        // )) {
        //     const get = getHook ?? getHookMap;
        //     const hookOrMap = get();
        //     if (hookOrMap.isUsed()) {
        //         kinds.push(registerKind);
        //     }
        // }
        // if (this.#nonSkippableRegisters.join() !== kinds.join()) {
        //     this.#getInstance((_error, instance) => {
        //         instance!.setNonSkippableRegisters(kinds);
        //         this.#nonSkippableRegisters = kinds;
        //     });
        // }
        // this.bindingRsWebpack.setNonSkippableRegisters()
    }

    run() {
        // this.hooks.beforeRun.call(this)
        console.log(222)
        this.bindingRsWebpack.run()
    }
}