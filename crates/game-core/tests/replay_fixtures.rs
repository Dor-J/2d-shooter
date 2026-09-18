//! Acceptance evidence:
//!   rust:replay:record
//!   rust:replay:play
//!   rust:replay:seek
//!   rust:replay:compat
//!   rust:replay:repair
//!   rust:capacity:16

use game_core::{Input, Replay, ReplayError, ReplayHeader, World};
use std::collections::BTreeMap;

fn header(world: &World) -> ReplayHeader {
    ReplayHeader::new(15, "Aero", world, 7)
}

fn right(seq: u32) -> Input {
    Input {
        seq,
        right: true,
        ..Input::default()
    }
}

#[test]
fn recording_the_same_inputs_twice_replays_to_the_same_digest() {
    let mut live = World::new("deathmatch");
    live.add_player(1, "Ace".into());
    let mut replay = Replay::new(header(&live));
    let start = live.clone();
    for seq in 1..=8 {
        let inputs = BTreeMap::from([(1, right(seq))]);
        live.step_with_bots(&inputs);
        replay.record(&live, &inputs).unwrap();
    }
    let played = replay.play(start).unwrap();
    assert_eq!(played.digest().as_str(), live.digest().as_str());
    assert_eq!(
        played.digest().as_str(),
        replay.last_digest().unwrap().as_str()
    );
}

#[test]
fn seek_and_fast_forward_stop_on_a_recorded_tick() {
    let mut live = World::new("deathmatch");
    live.add_player(1, "Ace".into());
    let mut replay = Replay::new(header(&live));
    let start = live.clone();
    for seq in 1..=6 {
        let inputs = BTreeMap::from([(1, right(seq))]);
        live.step_with_bots(&inputs);
        replay.record(&live, &inputs).unwrap();
    }
    let mid = replay.seek(start.clone(), replay.chunks[2].tick).unwrap();
    assert_eq!(mid.tick, replay.chunks[2].tick);
    let fast = replay.fast_forward(start, 2).unwrap();
    assert_eq!(fast.tick, replay.chunks[1].tick);
}

#[test]
fn a_wrong_protocol_or_a_truncated_tail_is_refused_or_repaired() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Ace".into());
    let mut header = header(&world);
    header.protocol = 1;
    assert_eq!(
        header.compatible(15, world.weapons.canonical_hash()),
        Err(ReplayError::Protocol)
    );
    let mut replay = Replay::new(header);
    replay.header.protocol = 15;
    let inputs = BTreeMap::from([(1, right(1))]);
    world.step_with_bots(&inputs);
    replay.record(&world, &inputs).unwrap();
    replay.chunks.push(game_core::ReplayChunk {
        tick: world.tick + 1,
        inputs: BTreeMap::new(),
        checksum: String::new(),
    });
    let repaired = replay.repair_truncated().unwrap();
    assert_eq!(repaired.chunks.len(), 1);
    let text = repaired.export().unwrap();
    assert_eq!(Replay::import(&text).unwrap().chunks.len(), 1);
}

#[test]
fn sixteen_players_fit_in_one_world() {
    let mut world = World::new("deathmatch");
    for id in 1..=16 {
        world.add_player(id, format!("P{id}"));
    }
    assert_eq!(world.players.len(), 16);
    world.step_with_bots(&BTreeMap::new());
    assert_eq!(world.players.len(), 16);
}

#[test]
// Acceptance evidence: rust:replay:soak
fn a_short_soak_replays_without_drift() {
    let mut live = World::new("deathmatch");
    live.add_player(1, "Ace".into());
    let mut replay = Replay::new(header(&live));
    let start = live.clone();
    for seq in 1..=180 {
        let inputs = BTreeMap::from([(1, right(seq))]);
        live.step_with_bots(&inputs);
        replay.record(&live, &inputs).unwrap();
    }
    assert_eq!(
        replay.play(start).unwrap().digest().as_str(),
        live.digest().as_str()
    );
}
