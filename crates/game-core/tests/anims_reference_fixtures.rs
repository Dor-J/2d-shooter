//! Frame counts and playback defaults for the OpenSoldat animation catalog.
//!
//! Expected values live in `tests/fixtures/reference/anims.json`. Numeric facts only: no pose
//! coordinates, curves, or `.poa` bodies.

use serde_json::Value;

const FILES_COMMIT: &str = "5b6e5bef23f5c0d58fb1d4d887b9b94ebcf799b4";
const LOADER_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

fn fixture() -> Value {
    let raw = include_str!("../../../tests/fixtures/reference/anims.json");
    serde_json::from_str(raw).expect("animation reference fixture parses")
}

fn files(fixture: &Value) -> &[Value] {
    fixture["files"].as_array().expect("files").as_slice()
}

fn by_object<'a>(files: &'a [Value], name: &str) -> &'a Value {
    files
        .iter()
        .find(|file| file["object"] == name)
        .unwrap_or_else(|| panic!("missing animation {name}"))
}

#[test]
fn opensoldat_animation_catalog_records_frame_counts() {
    let fixture = fixture();
    assert_eq!(fixture["sourceFiles"], "opensoldat/base shared/anims/*.poa");
    assert_eq!(fixture["filesCommit"], FILES_COMMIT);
    assert_eq!(
        fixture["loader"],
        "opensoldat/opensoldat shared/Anims.pas TAnimation.LoadFromFile and LoadAnimObjects"
    );
    assert_eq!(fixture["loaderCommit"], LOADER_COMMIT);
    assert_eq!(
        fixture["formatNotes"]
            .as_array()
            .expect("formatNotes")
            .iter()
            .map(|note| note.as_str().expect("format note"))
            .collect::<Vec<_>>(),
        [
            "one token per line",
            "keywords NEXTFRAME and ENDFILE",
            "NumFrames starts at 1 and increments on NEXTFRAME",
            "MAX_FRAMES_INDEX=40",
            "point index 1..20 (MAX_POS_INDEX=20)",
            "Speed and Loop are NOT in the file (Speed defaults to 1, Loop defaults false)",
            "SCALE=3 applied at load",
            "DoAnimation advances one frame every Speed calls",
            "one cycle is NumFrames*Speed updates",
        ]
    );

    let files = files(&fixture);
    assert_eq!(files.len(), 51, "51 .poa files");

    let loaded: Vec<_> = files
        .iter()
        .filter(|file| file["loaded"].as_bool() == Some(true))
        .collect();
    assert_eq!(loaded.len(), 44, "44 loaded animations");

    let mut ids: Vec<u64> = loaded
        .iter()
        .map(|file| file["id"].as_u64().expect("loaded id"))
        .collect();
    ids.sort_unstable();
    assert_eq!(ids, (0..44).collect::<Vec<_>>(), "ids 0..43 unique");

    let mut unloaded: Vec<&str> = files
        .iter()
        .filter(|file| file["loaded"].as_bool() == Some(false))
        .map(|file| {
            assert!(file["speed"].is_null(), "speed is null when not loaded");
            file["file"].as_str().expect("file name")
        })
        .collect();
    unloaded.sort_unstable();
    assert_eq!(
        unloaded,
        [
            "biegaranny.poa",
            "chat.poa",
            "firemode.poa",
            "pistolet.poa",
            "stunt.poa",
            "template.poa",
            "zmienbron.poa",
        ]
    );

    let run = by_object(files, "Run");
    assert_eq!(run["frames"].as_u64(), Some(38));
    assert_eq!(run["speed"].as_u64(), Some(1));
    assert_eq!(run["loop"].as_bool(), Some(true));

    assert_eq!(by_object(files, "Roll")["frames"].as_u64(), Some(34));
    assert_eq!(by_object(files, "RollBack")["frames"].as_u64(), Some(32));
    assert_eq!(by_object(files, "Prone")["frames"].as_u64(), Some(27));
    assert_eq!(by_object(files, "JumpSide")["frames"].as_u64(), Some(34));

    let stand = by_object(files, "Stand");
    assert_eq!(stand["id"].as_u64(), Some(0));
    assert_eq!(stand["frames"].as_u64(), Some(17));
    assert_eq!(stand["speed"].as_u64(), Some(3));
    assert_eq!(stand["loop"].as_bool(), Some(true));

    let barret = by_object(files, "Barret");
    assert_eq!(barret["speed"].as_u64(), Some(9));
    assert_eq!(barret["frames"].as_u64(), Some(29));

    assert_eq!(by_object(files, "Change")["frames"].as_u64(), Some(40));
    assert_eq!(by_object(files, "Throw")["frames"].as_u64(), Some(40));

    let template = files
        .iter()
        .find(|file| file["file"] == "template.poa")
        .expect("template.poa");
    assert_eq!(template["loaded"].as_bool(), Some(false));
    assert_eq!(template["frames"].as_u64(), Some(2));
}
