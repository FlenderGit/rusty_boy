/* tslint:disable */
/* eslint-disable */
/**
*/
export function start(): void;
/**
*/
export class RustyBoy {
  free(): void;
/**
* @param {Uint8Array} rom
* @param {boolean} skip_checksum
*/
  constructor(rom: Uint8Array, skip_checksum: boolean);
/**
*/
  run_frame(): void;
/**
* @returns {Uint8Array}
*/
  get_screen_data(): Uint8Array;
/**
* @param {number} i
*/
  press_key(i: number): void;
/**
* @param {number} i
*/
  release_key(i: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly start: () => void;
  readonly __wbg_rustyboy_free: (a: number, b: number) => void;
  readonly rustyboy_new: (a: number, b: number, c: number, d: number) => void;
  readonly rustyboy_run_frame: (a: number) => void;
  readonly rustyboy_get_screen_data: (a: number, b: number) => void;
  readonly rustyboy_press_key: (a: number, b: number) => void;
  readonly rustyboy_release_key: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
