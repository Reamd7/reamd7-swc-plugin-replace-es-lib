import assert from 'assert'
import { transform } from './_do'

const result = transform(
  `
import { a } from '@tarslib/utils/es/xxx';
import { b } from '@tarslib/utils/es/xxx';
import * as c from '@tarslib/utils/es/xxx';
import("@tarslib/utils/es/xxx")
require('@tarslib/utils/es/xxx')
console.log(a, b, c)
`,
  {
    target_module: ['@tarslib/utils'],
    direction: 'es2lib',
  }
)
console.log(result.code)

assert(
  result.code.trim() ===
    `
import { a } from "@tarslib/utils/lib/xxx";
import { b } from "@tarslib/utils/lib/xxx";
import * as c from "@tarslib/utils/lib/xxx";
import("@tarslib/utils/lib/xxx");
require("@tarslib/utils/lib/xxx");
console.log(a, b, c);
  `.trim()
)
