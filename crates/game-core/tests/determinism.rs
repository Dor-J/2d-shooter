use game_core::{replay_fixture_json, Fixture, FrameInput, Input, World};
use std::collections::BTreeMap;

#[test]
fn fixture_replay_produces_stable_per_frame_digests() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Fixture Player".into());
    let frames = vec![
        FrameInput {
            inputs: BTreeMap::from([(
                1,
                Input {
                    seq: 1,
                    right: true,
                    ..Input::default()
                },
            )]),
        },
        FrameInput {
            inputs: BTreeMap::from([(
                1,
                Input {
                    seq: 2,
                    right: true,
                    jump: true,
                    ..Input::default()
                },
            )]),
        },
    ];
    let fixture = Fixture {
        source_revision: "c7596cdca32416cb66339b339105eb1e07e7fbbf".into(),
        source_path: "tests/fixture-contract".into(),
        seed: 7,
        initial: world,
        frames,
        expected: Vec::new(),
    };

    let first = fixture.replay();
    let second = fixture.replay();

    assert_eq!(first, second);
    assert_eq!(
        first
            .iter()
            .map(|digest| digest.as_str())
            .collect::<Vec<_>>(),
        ["5f557107f804d991", "b12a16e8d2e36bd3"]
    );
}

#[test]
fn replay_json_returns_the_same_digests_used_by_wasm() {
    let fixture_json = r#"{
      "source_revision":"c7596cdca32416cb66339b339105eb1e07e7fbbf",
      "source_path":"tests/empty-fixture",
      "seed":99,
      "initial":{"tick":0,"mode":"deathmatch","players":{},"projectiles":[],"scores":[0,0],"events":[]},
      "frames":[{"inputs":{}}],
      "expected":[]
    }"#;

    assert_eq!(
        replay_fixture_json(fixture_json).unwrap(),
        "[\"59f65e2ae86448fc\"]"
    );
    assert!(replay_fixture_json("not json").is_err());
}

#[test]
fn fixture_json_round_trip_preserves_replay_results() {
    let fixture_json = r#"{
      "source_revision":"c7596cdca32416cb66339b339105eb1e07e7fbbf",
      "source_path":"tests/empty-fixture",
      "seed":99,
      "initial":{"tick":0,"mode":"deathmatch","players":{},"projectiles":[],"scores":[0,0],"events":[]},
      "frames":[],
      "expected":[]
    }"#;

    let fixture: Fixture = serde_json::from_str(fixture_json).unwrap();
    let encoded = serde_json::to_string(&fixture).unwrap();
    let decoded: Fixture = serde_json::from_str(&encoded).unwrap();

    assert_eq!(fixture.replay(), decoded.replay());
    assert_eq!(decoded.initial.digest(), fixture.initial.digest());
}
