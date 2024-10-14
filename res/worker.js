const WASM_PATH = '../generated/';

importScripts( WASM_PATH + 'web.js');

/*
const { read_file } = wasm_bindgen;

async function run() {
    await wasm_bindgen( WASM_PATH + 'web_bg.wasm');
    console.log('Wasm loaded');
}

run(); */

onmessage = async function(e) {
    postMessage('Worker received message');
    /* let file = e.data;
    let gameboy = await new_gameboy_from_file(file);
    postMessage(gameboy); */
}