import init, { add } from '../dist/wasm-demo/index.js';

const start = async () => {
  await init();
  console.log(add(1, 2));
}
start()