use content::anim::{encode, parse, AnimError};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[test]
fn round_trips_an_authored_two_point_clip() {
    let bytes = br#"{"version":1,"id":9,"name":"scratch","frames":1,"speed":4,"loop":false,"points":[[{"x":1.25,"y":-4.5},{"x":8.0,"y":0.25}]]}"#;
    let clip = parse(bytes).unwrap();
    assert_eq!(clip.id, 9);
    assert_eq!(clip.name, "scratch");
    assert_eq!(clip.frames, 1);
    assert_eq!(clip.speed, 4);
    assert!(!clip.looping);
    assert_eq!(clip.points.len(), 1);
    assert_eq!(clip.points[0].len(), 2);
    assert_eq!(clip.points[0][0].x, 1.25);
    assert_eq!(clip.points[0][0].y, -4.5);
    assert_eq!(clip.points[0][1].x, 8.0);
    assert_eq!(clip.points[0][1].y, 0.25);
    assert_eq!(parse(&encode(&clip).unwrap()).unwrap(), clip);
}

#[test]
fn rejects_a_bad_version() {
    let bytes =
        br#"{"version":2,"id":0,"name":"nope","frames":1,"speed":1,"loop":false,"points":[[]]}"#;
    let error = parse(bytes).unwrap_err();
    assert_eq!(error, AnimError::UnsupportedVersion { value: 2 });
}

#[test]
fn rejects_forty_one_frames() {
    let bytes =
        br#"{"version":1,"id":0,"name":"long","frames":41,"speed":1,"loop":false,"points":[]}"#;
    let error = parse(bytes).unwrap_err();
    assert_eq!(
        error,
        AnimError::CountOutOfRange {
            field: "frames",
            value: 41,
            max: 40,
        }
    );
}

#[test]
fn rejects_twenty_one_points() {
    let points = (0..21)
        .map(|index| format!(r#"{{"x":{index}.0,"y":1.0}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let bytes = format!(
        r#"{{"version":1,"id":0,"name":"wide","frames":1,"speed":1,"loop":false,"points":[[{points}]]}}"#
    );
    let error = parse(bytes.as_bytes()).unwrap_err();
    assert_eq!(
        error,
        AnimError::CountOutOfRange {
            field: "points",
            value: 21,
            max: 20,
        }
    );
}

#[test]
fn rejects_a_truncated_document() {
    let bytes = br#"{"version":1,"id":0,"name":"cut","frames":1,"speed":1,"loop":false,"points":[[{"x":1.0,"y":2.0}"#;
    let error = parse(bytes).unwrap_err();
    assert!(matches!(error, AnimError::Truncated { .. }));
}

#[test]
fn loaded_catalog_matches_reference_headers() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/reference/anims.json"))
            .expect("animation reference fixture");
    let loaded: Vec<&Value> = fixture["files"]
        .as_array()
        .expect("files")
        .iter()
        .filter(|file| file["loaded"].as_bool() == Some(true))
        .collect();
    assert_eq!(loaded.len(), 44);

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("animations");
    let mut files: Vec<_> = fs::read_dir(&dir)
        .expect("animations directory")
        .map(|entry| entry.expect("entry").path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    files.sort();
    assert_eq!(files.len(), 44);

    let mut names = Vec::new();
    for file in &loaded {
        let name = file["object"].as_str().expect("object");
        names.push(name.to_owned());
        let path = dir.join(format!("{name}.json"));
        let raw = fs::read_to_string(&path).unwrap_or_else(|_| panic!("missing {name}"));
        let clip = parse(raw.as_bytes()).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(clip.version, 1);
        assert_eq!(clip.id, file["id"].as_u64().expect("id") as u32);
        assert_eq!(clip.name, name);
        assert_eq!(clip.frames, file["frames"].as_u64().expect("frames") as u32);
        assert_eq!(clip.speed, file["speed"].as_u64().expect("speed") as u32);
        assert_eq!(clip.looping, file["loop"].as_bool().expect("loop"));
        assert_eq!(clip.points.len(), clip.frames as usize);
        for frame in &clip.points {
            assert!(frame.len() <= 20, "{name} has more than 20 points");
            for point in frame {
                assert!(
                    point.x.is_finite()
                        && point.y.is_finite()
                        && (-40.0..=40.0).contains(&point.x)
                        && (-40.0..=40.0).contains(&point.y),
                    "{name} point is out of range"
                );
            }
        }
    }
    names.sort();
    let mut stems: Vec<_> = files
        .iter()
        .map(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .expect("stem")
                .to_owned()
        })
        .collect();
    stems.sort();
    assert_eq!(stems, names);

    for banned in [
        "biegaranny",
        "chat",
        "firemode",
        "pistolet",
        "stunt",
        "template",
        "zmienbron",
    ] {
        assert!(!dir.join(format!("{banned}.json")).exists(), "{banned}");
        assert!(!dir.join(format!("{banned}.poa")).exists(), "{banned}");
    }
}
