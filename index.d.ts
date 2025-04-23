export interface ISwcPluginReplaceEsLibConfig {
  /**
   * target module
   */
  target_module: string[]

  /**
   * es2lib | lib2es
   * @default 'es2lib'
   */
  direction: string
}
