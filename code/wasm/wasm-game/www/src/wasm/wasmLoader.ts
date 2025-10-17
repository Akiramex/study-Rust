import init, { World } from './wasm_game';

export async function loadWasm() {
    await init();

    return { World };
}