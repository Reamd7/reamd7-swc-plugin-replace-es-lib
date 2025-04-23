import { transformSync } from '@swc/core'
import type { ISwcPluginReplaceEsLibConfig } from '../'

export const transform = (
  code: string,
  opts: ISwcPluginReplaceEsLibConfig
) => {
  return transformSync(code, {
    jsc: {
      experimental: {
        plugins: [
          [
            require.resolve(
              '../target/wasm32-wasip1/release/swc_plugin_replace_es_lib.wasm'
            ),
            opts,
          ],
        ],
      },
      parser: {
        syntax: 'typescript',
        dynamicImport: true,
        tsx: true,
      },
      target: 'es2015',
      minify: {
        compress: false,
        mangle: false,
      },
      transform: {
        react: {
          runtime: 'automatic',
          pragma: 'React.createElement',
          pragmaFrag: 'React.Fragment',
          throwIfNamespace: true,
          development: true,
          useBuiltins: true,
        },
      },
    },
    module: {
      type: 'es6',
      ignoreDynamic: true,
    },
    minify: false,
    isModule: true,
    sourceMaps: false,
    filename: 'index.tsx',
  })
}
