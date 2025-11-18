import init, { deserialize_payload, serialize_payload } from '../../pkg/parse';

let wasmInitialized = false;
let wasmInitPromise: Promise<void> | null = null;

export async function initWasm(): Promise<void> {
  if (wasmInitialized) {
    return;
  }

  if (wasmInitPromise) {
    return wasmInitPromise;
  }

  wasmInitPromise = (async () => {
    try {
      await init();
      wasmInitialized = true;
    } catch (error) {
      wasmInitPromise = null;
      throw new Error(`Failed to initialize WASM module: ${error}`);
    }
  })();

  return wasmInitPromise;
}

export function serializeMessage(value: unknown): Uint8Array {
  if (!wasmInitialized) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }
  return serialize_payload(value);
}

export function deserializeMessage(bytes: Uint8Array): unknown {
  if (!wasmInitialized) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }
  return deserialize_payload(bytes);
}

export function isWasmInitialized(): boolean {
  return wasmInitialized;
}
