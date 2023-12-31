import init from "./wasm/website.js";

async function run() {
  console.log("Initializing WASM");
  await init();
  console.log("WASM initialized");
}

run();
