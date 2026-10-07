//! Pinned OpenSoldat object, sound, and timer ids from `shared/Constants.pas`.
//!
//! Expected values only, recorded in `tests/fixtures/reference`. Nothing here copies upstream source.

use std::collections::HashSet;

use serde_json::Value;

const SOURCE: &str = "opensoldat/opensoldat shared/Constants.pas";
const COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

fn load(raw: &str) -> Value {
    serde_json::from_str(raw).expect("reference id fixture parses")
}

fn rows<'a>(doc: &'a Value, key: &str) -> &'a [Value] {
    doc[key]
        .as_array()
        .unwrap_or_else(|| panic!("{key} is an array"))
}

fn value_named(entries: &[Value], name: &str) -> i64 {
    entries
        .iter()
        .find(|entry| entry["name"] == name)
        .unwrap_or_else(|| panic!("missing {name}"))["value"]
        .as_i64()
        .unwrap_or_else(|| panic!("{name} value"))
}

#[test]
fn reference_ids_match_the_pinned_constants() {
    let timers = load(include_str!(
        "../../../tests/fixtures/reference/timers.json"
    ));
    let objects = load(include_str!(
        "../../../tests/fixtures/reference/objects.json"
    ));
    let sfx = load(include_str!("../../../tests/fixtures/reference/sfx.json"));

    for doc in [&timers, &objects, &sfx] {
        assert_eq!(doc["source"], SOURCE);
        assert_eq!(doc["commit"], COMMIT);
    }

    let timers = rows(&timers, "timers");
    assert_eq!(timers.len(), 9);
    assert_eq!(value_named(timers, "SECOND"), 60);
    assert_eq!(value_named(timers, "FLAMERBONUSTIME"), 600);
    assert_eq!(value_named(timers, "FLAG_TIMEOUT"), 1500);
    assert_eq!(value_named(timers, "PARA_DISTANCE"), 500);
    assert_eq!(value_named(timers, "SURVIVAL_RESPAWNTIME"), 300);

    let objects = rows(&objects, "objects");
    assert_eq!(objects.len(), 29);
    assert_eq!(value_named(objects, "OBJECT_PARACHUTE"), 23);
    assert_eq!(value_named(objects, "OBJECT_RAMBO_BOW"), 15);
    assert_eq!(value_named(objects, "OBJECT_STATIONARY_GUN"), 27);

    assert_eq!(sfx["unusedIds"], serde_json::json!([4]));
    let sounds = rows(&sfx, "sounds");
    assert_eq!(sounds.len(), 162);
    let values: Vec<i64> = sounds
        .iter()
        .map(|sound| sound["value"].as_i64().expect("sound value"))
        .collect();
    assert!(!values.contains(&4));
    assert_eq!(values[0], 1);
    assert_eq!(sounds.last().expect("a sound")["name"], "SFX_WIND");
    assert_eq!(*values.last().expect("a sound value"), 163);
    assert_eq!(values.iter().collect::<HashSet<_>>().len(), values.len());
}
