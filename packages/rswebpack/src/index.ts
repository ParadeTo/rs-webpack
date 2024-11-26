import {JsTap, RegisterJsTapKind, RegisterJsTaps, RsWebpack as BindingRsWebpack} from "@rswebpack/binding"
import * as liteTapable from '@rspack/lite-tapable'

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
        beforeRun: liteTapable.SyncHook<[string]>;
        beforeRunSync: liteTapable.SyncHook<[string]>;
    }
    registers?: RegisterJsTaps;

    constructor(props: RawConfig) {
        this.hooks = {
            beforeRun: new liteTapable.SyncHook(['root']),
            beforeRunSync: new liteTapable.SyncHook(['root']),
        }
        const {plugins} = props
        plugins.forEach(plugin => {
            plugin.apply(this)
        })
        this.registers = {
            registerBeforeRunTaps: this.#createHookRegisterTaps(
                RegisterJsTapKind.BeforeRun,
                () => this.hooks.beforeRun,
                queried => (native: string) => {
                    console.log('before run', )
                    queried.call(native);
                }
            ),
            registerBeforeRunSyncTaps: this.#createHookRegisterTaps(
                RegisterJsTapKind.BeforeRunSync,
                () => this.hooks.beforeRunSync,
                queried => (native: string) => {
                    console.log('before run sync', native)
                    queried.call(native);
                }
            )
        }
        this.bindingRsWebpack = new BindingRsWebpack(props, this.registers)
        this.bindingRsWebpack.setNonSkippableRegisters([RegisterJsTapKind.BeforeRunSync]);

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

    async run() {
        // this.hooks.beforeRun.call(this)
        console.log(222)
        await this.bindingRsWebpack.run()
    }

    #createHookRegisterTaps<T, R, A>(
        registerKind: RegisterJsTapKind,
        getHook: () => liteTapable.Hook<T, R, A>,
        createTap: (queried: liteTapable.QueriedHook<T, R, A>) => any
    ): (stages: number[]) => JsTap[] {
        const getTaps = (stages: number[]) => {
            const hook = getHook();
            if (!hook.isUsed()) return [];
            const breakpoints = [
                liteTapable.minStage,
                ...stages,
                liteTapable.maxStage
            ];
            const jsTaps: JsTap[] = [];
            for (let i = 0; i < breakpoints.length - 1; i++) {
                const from = breakpoints[i];
                const to = breakpoints[i + 1];
                const stageRange = [from, to] as const;
                const queried = hook.queryStageRange(stageRange);
                if (!queried.isUsed()) continue;
                jsTaps.push({
                    function: createTap(queried),
                    stage: liteTapable.safeStage(from + 1)
                });
            }
            // this.#decorateJsTaps(jsTaps);
            return jsTaps;
        };
        getTaps.registerKind = registerKind;
        getTaps.getHook = getHook;
        return getTaps;
    }


}