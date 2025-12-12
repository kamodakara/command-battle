/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__h46aa9f72f6c05a8e: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h7e6eb12389f23c6c: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h2ff4f1994c6612fc: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h1cb5b95d151286bf: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h1d5577f107f771af: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h9aaa6c46cefaa00f: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h4daf360047df690d: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__he93315c97fac8ae5: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hdaf4da541cbd5c9c: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
