//! OpenSoldat body integration, jet, and realistic-mode facts.
//!
//! Expected values live in `tests/fixtures/reference/physics.json`. Numeric facts only.

use serde_json::Value;

const SOURCE: &str = "opensoldat/opensoldat";
const COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/reference/physics.json"
    ))
    .expect("physics reference fixture parses")
}

fn constants(doc: &Value) -> &[Value] {
    doc["constants"].as_array().expect("constants")
}

fn unique<'a>(doc: &'a Value, name: &str) -> &'a Value {
    let hits: Vec<_> = constants(doc)
        .iter()
        .filter(|row| row["name"] == name)
        .collect();
    assert_eq!(hits.len(), 1, "{name}");
    hits[0]
}

fn number(doc: &Value, name: &str) -> f64 {
    unique(doc, name)["value"]
        .as_f64()
        .unwrap_or_else(|| panic!("{name} value"))
}

#[test]
fn opensoldat_physics_integration_matches_the_pinned_source() {
    let doc = fixture();
    assert_eq!(doc["source"], SOURCE);
    assert_eq!(doc["commit"], COMMIT);
    assert_eq!(doc["tickRate"].as_u64(), Some(60));
    assert_eq!(constants(&doc).len(), 27);

    assert_eq!(number(&doc, "RUNSPEED"), 0.118);
    assert_eq!(number(&doc, "JETSPEED"), 0.10);
    assert_eq!(number(&doc, "GRAV"), 0.06);
    assert_eq!(number(&doc, "EDamping"), 0.99);
    let live = constants(&doc)
        .iter()
        .find(|row| {
            row["name"] == "VDamping"
                && row["file"] == "shared/mechanics/Sprites.pas"
                && row["line"] == 371
        })
        .expect("live VDamping");
    assert_eq!(live["value"].as_f64().expect("live VDamping"), 0.9945);
    assert_eq!(number(&doc, "MAX_VELOCITY"), 11.0);
    assert_eq!(number(&doc, "PARA_SPEED"), -0.03);

    let run_up = unique(&doc, "RUNSPEEDUP");
    assert_eq!(run_up["value"], "0.118/6");
    assert_eq!(
        run_up["numeric"].as_f64().expect("RUNSPEEDUP numeric"),
        0.019666666666666666
    );
    assert_eq!(unique(&doc, "CROUCHRUNSPEED")["value"], "0.118/0.6");
    assert_eq!(unique(&doc, "PRONESPEED")["value"], "0.118*4");
    assert_eq!(unique(&doc, "ROLLSPEED")["value"], "0.118/1.2");

    assert_eq!(doc["integration"]["body"].as_array().map(Vec::len), Some(6));
    assert_eq!(
        doc["realistic"]["movementConstantsChanged"].as_bool(),
        Some(false)
    );
}
