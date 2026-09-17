use content::manifest::{MapBundle, MapMode};
use content::pms::parse;
use ed25519_dalek::SigningKey;
use map_editor::{original_default_map, EditorProject, EntityKind, Selection, DEFAULT_MAPS};
use std::collections::BTreeSet;

#[test]
fn editor_supports_selection_grid_zoom_history_clipboard_prefabs_and_native_projects() {
    let mut editor = EditorProject::new(original_default_map(DEFAULT_MAPS[0]));
    assert_eq!(editor.snap(14.0), 10.0);
    editor.set_zoom(99.0);
    assert_eq!(editor.zoom, 8.0);
    editor.set_pan(12.0, -5.0);
    assert_eq!(editor.pan, [12.0, -5.0]);
    editor.selection = vec![Selection {
        kind: EntityKind::Polygon,
        index: 0,
    }];
    let clipboard = editor.copy();
    let prefab = editor.prefab().unwrap();
    let before = editor.map.polygons.len();
    editor.paste(&clipboard);
    assert_eq!(editor.map.polygons.len(), before + 1);
    assert!(editor.undo());
    assert_eq!(editor.map.polygons.len(), before);
    assert!(editor.redo());
    editor.paste_prefab(&prefab).unwrap();
    let project = editor.export_project().unwrap();
    let restored = EditorProject::import_project(&project).unwrap();
    assert_eq!(restored.version, 1);
}

#[test]
fn all_97_original_maps_pass_the_admission_pipeline() {
    assert_eq!(DEFAULT_MAPS.len(), 97);
    assert_eq!(
        DEFAULT_MAPS
            .iter()
            .map(|entry| entry.name)
            .collect::<BTreeSet<_>>()
            .len(),
        97
    );
    let key = SigningKey::from_bytes(&[11; 32]);
    for entry in DEFAULT_MAPS {
        let editor = EditorProject::new(original_default_map(*entry));
        editor
            .validate(entry.mode)
            .unwrap_or_else(|error| panic!("{} validation: {error}", entry.name));
        editor
            .offline_test(entry.mode)
            .unwrap_or_else(|error| panic!("{} smoke: {error}", entry.name));
        assert!(
            editor.preview_svg().contains("<polygon"),
            "{} preview",
            entry.name
        );
        let pms = editor.export_pms().unwrap();
        assert_eq!(parse(&pms, &Default::default()).unwrap().name, entry.name);
        let package = editor.package(entry.mode, &key).unwrap();
        package.manifest.verify().unwrap();
        MapBundle::verify(&package.manifest.manifest, &package.pms, &package.assets).unwrap();
    }
}

#[test]
fn mode_validation_rejects_missing_team_spawns() {
    let mut map = original_default_map(DEFAULT_MAPS[29]);
    map.spawnpoints.retain(|spawn| spawn.team != 2);
    let editor = EditorProject::new(map);
    assert!(editor.validate(MapMode::CaptureTheFlag).is_err());
}
