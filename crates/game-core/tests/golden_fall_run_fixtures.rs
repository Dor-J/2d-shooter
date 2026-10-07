//! OpenSoldat dedicated-server fall-and-run pose trace on ctf_Ash.
//!
//! Expected values live in `tests/fixtures/reference/golden-fall-run.json`.
//! Each sample is the pose at the start of that tick. Numeric facts only.

use serde_json::Value;

const SOURCE: &str = "opensoldat/opensoldat dedicated server";
const COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/reference/golden-fall-run.json"
    ))
    .expect("fall-and-run reference fixture parses")
}

fn samples(doc: &Value) -> &[Value] {
    doc["samples"].as_array().expect("samples").as_slice()
}

fn at(rows: &[Value], tick: u64) -> &Value {
    let row = &rows[tick as usize];
    assert_eq!(row["tick"].as_u64(), Some(tick));
    row
}

fn num(row: &Value, key: &str) -> f64 {
    row[key].as_f64().unwrap_or_else(|| panic!("{key} number"))
}

#[test]
fn opensoldat_fall_and_run_trace_matches_the_pinned_server() {
    let doc = fixture();
    assert_eq!(doc["source"], SOURCE);
    assert_eq!(doc["commit"], COMMIT);
    assert_eq!(doc["map"], "ctf_Ash");
    assert_eq!(
        doc["script"],
        "30 ticks still, 60 ticks right, 30 ticks jump (KeyUp)"
    );
    assert_eq!(
        doc["note"],
        "Values are at the start of the tick. Keys apply on the following integration. Positive Y is down."
    );

    let rows = samples(&doc);
    assert_eq!(rows.len(), 120);

    let first = at(rows, 0);
    assert_eq!(num(first, "x"), -419.0);
    assert_eq!(num(first, "y"), -203.0);
    assert_eq!(num(first, "vx"), 0.0);
    assert_eq!(num(first, "vy"), 0.0);

    let last = at(rows, 119);
    assert_eq!(num(last, "x"), -268.065);
    assert_eq!(num(last, "y"), -143.421);
    assert_eq!(num(last, "vx"), 1.906);
    assert_eq!(num(last, "vy"), 1.751);

    // Tick 29 is still the idle fall: horizontal speed has not started.
    assert_eq!(num(at(rows, 29), "vx"), 0.0);
    // Tick 40 is after landing, with the right key already integrated.
    assert!(num(at(rows, 40), "vx") > 0.0);
}
