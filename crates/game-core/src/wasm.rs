use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn predict_player(player_json: &str, input_json: &str) -> String {
    let Ok(player) = serde_json::from_str::<Player>(player_json) else {
        return String::new();
    };
    let Ok(input) = serde_json::from_str::<Input>(input_json) else {
        return String::new();
    };
    let mut world = World::new("deathmatch");
    let id = player.id;
    world.players.insert(id, player);
    world.step(&BTreeMap::from([(id, input)]));
    serde_json::to_string(&world.players[&id]).unwrap_or_default()
}

#[wasm_bindgen]
pub fn replay_fixture(fixture_json: &str) -> String {
    replay_fixture_json(fixture_json).unwrap_or_default()
}
