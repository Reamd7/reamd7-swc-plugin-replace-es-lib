# swc-plugin-replace-es-lib

Auto css modules plugin for swc.

## Install

```bash
  pnpm i -D reamd7-swc-plugin-replace-es-lib
```

You should install the following versions:

|`@swc/core` version|working version|
|:-:|:-:|
| `latest` > version >= `1.4.0`|`swc-plugin-replace-es-lib@1.6.0`|
| `1.3.108` > version >= `1.3.106`|Not support|
| `1.3.106` > version >= `1.3.63`|`swc-plugin-replace-es-lib@1.5.0`|
| `1.3.63` > version >= `1.3.41`|`swc-plugin-replace-es-lib@1.4.0`|
| `1.3.41` > version >= `1.3.24`|`swc-plugin-replace-es-lib@1.3.0`|

## Usage

```diff
// swc config
{
  jsc: {
    experimental: {
      plugins: [
+       ['reamd7-swc-plugin-replace-es-lib', {}]
      ],
    },
  }
}
```

### Auto css modules

This plugin will auto add the `?modules` suffix.

```ts
import styles from './index.less'
// to
import styles from './index.less?modules'
```

### Lock `core-js` import

Lock `core-js` import by config `lock_core_js_pkg_path`.

```ts
plugins: [
  [
    'swc-plugin-replace-es-lib',
    { lock_core_js_pkg_path: dirname(require.resolve('core-js/package.json')) },
  ],
]
```

```ts
import 'core-js/es/modules'
// to
import '/node_modules/**/core-js/es/modules'
```

## Config

See [types](./index.d.ts) file

```ts
import type { ISwcPluginAutoCssModulesConfig } from 'swc-plugin-replace-es-lib'
```

## License

MIT
