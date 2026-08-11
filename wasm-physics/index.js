import init, { ParticleSystem } from './pkg/physics_wasm.js';

async function runWasmPhysics() {
  // Inicializa o binário WASM
  await init();

  const PARTICLE_COUNT = 100_000;
  const system = new ParticleSystem(PARTICLE_COUNT);

  let lastTime = performance.now();

  function loop(currentTime) {
    const dt = (currentTime - lastTime) / 1000;
    lastTime = currentTime;

    // Executa a física em Rust
    system.update(dt);

    requestAnimationFrame(loop);
  }

  requestAnimationFrame(loop);
}

runWasmPhysics();
