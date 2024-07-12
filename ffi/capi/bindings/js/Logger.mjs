import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const Logger_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLogger_destroy(underlying);
});

export class Logger {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Logger_box_destroy_registry.register(this, underlying);
    }
  }

  static init_simple_logger() {
    return wasm.ICU4XLogger_init_simple_logger();
  }
}