window.__arenaWasmReady = import('/wasm/game_core.js')
  .then(async (module) => {
    await module.default()
    return module.predict_player
  })
  .catch(() => null)
