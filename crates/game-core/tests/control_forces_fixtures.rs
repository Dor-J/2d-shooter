//! Force and velocity writes recorded from OpenSoldat `ControlSprite`.
//!
//! The table lives in `tests/fixtures/reference/control-forces.json` and names the pinned
//! upstream revision. Nothing here copies upstream implementation code.

use serde_json::Value;

fn fixture() -> Value {
    let raw = include_str!("../../../tests/fixtures/reference/control-forces.json");
    serde_json::from_str(raw).expect("control force reference fixture parses")
}

#[test]
fn control_force_table_records_the_pinned_writes() {
    let fixture = fixture();
    let entries = fixture["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 26);
    for id in [
        "backflip-entry",
        "roll-ground",
        "jump",
        "run-right",
        "parachute-right",
    ] {
        assert!(
            entries.iter().any(|entry| entry["id"] == id),
            "missing {id}"
        );
    }
    let assignment = entries
        .iter()
        .find(|entry| entry["id"] == "backflip-entry")
        .expect("backflip-entry")["assignment"]
        .as_str()
        .expect("assignment");
    assert!(assignment.contains("no force"));
}
