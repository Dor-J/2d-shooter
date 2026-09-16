#![forbid(unsafe_code)]

mod catalog;
pub use catalog::{original_default_map, DefaultMap, DEFAULT_MAPS};

use content::manifest::{sign_manifest, AssetEntry, MapManifest, MapMode, SignedMapManifest};
use content::pms::{encode, MapAsset, Polygon, Prop, Spawnpoint, Waypoint};
use ed25519_dalek::SigningKey;
use game_core::{ValidatedMap, World};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub const PROJECT_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityKind { Polygon, Prop, Spawn, Waypoint }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection { pub kind: EntityKind, pub index: usize }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClipboardItem { Polygon(Polygon), Prop(Prop), Spawn(Spawnpoint), Waypoint(Waypoint) }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorProject {
    pub version: u32,
    pub map: MapAsset,
    pub selection: Vec<Selection>,
    pub grid: f32,
    pub zoom: f32,
    #[serde(skip)]
    undo: Vec<MapAsset>,
    #[serde(skip)]
    redo: Vec<MapAsset>,
}

impl EditorProject {
    pub fn new(map: MapAsset) -> Self { Self { version: PROJECT_VERSION, map, selection: Vec::new(), grid: 10.0, zoom: 1.0, undo: Vec::new(), redo: Vec::new() } }
    pub fn import_pms(bytes: &[u8]) -> Result<Self, String> { content::pms::parse(bytes, &Default::default()).map(Self::new).map_err(|error| error.to_string()) }
    pub fn export_pms(&self) -> Result<Vec<u8>, String> { encode(&self.map).map_err(|error| error.to_string()) }
    pub fn import_project(bytes: &[u8]) -> Result<Self, String> { let project: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?; if project.version != PROJECT_VERSION { return Err("unsupported project version".into()); } Ok(project) }
    pub fn export_project(&self) -> Result<Vec<u8>, String> { serde_json::to_vec_pretty(self).map_err(|error| error.to_string()) }
    pub fn edit(&mut self, operation: impl FnOnce(&mut MapAsset)) { self.undo.push(self.map.clone()); self.redo.clear(); operation(&mut self.map); }
    pub fn undo(&mut self) -> bool { let Some(previous) = self.undo.pop() else { return false }; self.redo.push(std::mem::replace(&mut self.map, previous)); true }
    pub fn redo(&mut self) -> bool { let Some(next) = self.redo.pop() else { return false }; self.undo.push(std::mem::replace(&mut self.map, next)); true }
    pub fn snap(&self, value: f32) -> f32 { if self.grid > 0.0 { (value / self.grid).round() * self.grid } else { value } }
    pub fn set_zoom(&mut self, zoom: f32) { self.zoom = zoom.clamp(0.1, 8.0); }
    pub fn copy(&self) -> Vec<ClipboardItem> { self.selection.iter().filter_map(|selection| match selection.kind { EntityKind::Polygon => self.map.polygons.get(selection.index).cloned().map(ClipboardItem::Polygon), EntityKind::Prop => self.map.props.get(selection.index).cloned().map(ClipboardItem::Prop), EntityKind::Spawn => self.map.spawnpoints.get(selection.index).cloned().map(ClipboardItem::Spawn), EntityKind::Waypoint => self.map.waypoints.get(selection.index).cloned().map(ClipboardItem::Waypoint) }).collect() }
    pub fn paste(&mut self, items: &[ClipboardItem]) { self.edit(|map| for item in items { match item { ClipboardItem::Polygon(value) => map.polygons.push(value.clone()), ClipboardItem::Prop(value) => map.props.push(value.clone()), ClipboardItem::Spawn(value) => map.spawnpoints.push(value.clone()), ClipboardItem::Waypoint(value) => map.waypoints.push(value.clone()) } }); }
    pub fn prefab(&self) -> Result<Vec<u8>, String> { serde_json::to_vec(&self.copy()).map_err(|error| error.to_string()) }
    pub fn paste_prefab(&mut self, bytes: &[u8]) -> Result<(), String> { let items: Vec<ClipboardItem> = serde_json::from_slice(bytes).map_err(|error| error.to_string())?; self.paste(&items); Ok(()) }
    pub fn validate(&self, mode: MapMode) -> Result<(), String> { ValidatedMap::try_from(self.map.clone()).map_err(|error| format!("{error:?}"))?; validate_spawns(&self.map, mode) }
    pub fn preview_svg(&self) -> String { preview_svg(&self.map) }
    pub fn offline_test(&self, mode: MapMode) -> Result<(), String> { self.validate(mode)?; let map = ValidatedMap::try_from(self.map.clone()).map_err(|error| format!("{error:?}"))?; let mut world = World::with_map("deathmatch", &map); world.add_player(1, "Offline test".into()); for _ in 0..120 { world.step(&BTreeMap::new()); } if world.players[&1].pos.x.is_finite() && world.players[&1].pos.y.is_finite() { Ok(()) } else { Err("simulation became non-finite".into()) } }
    pub fn package(&self, mode: MapMode, key: &SigningKey) -> Result<MapPackage, String> { self.validate(mode)?; let pms = self.export_pms()?; let preview = self.preview_svg().into_bytes(); let texture = procedural_texture(); let assets = vec![AssetEntry::from_bytes("textures/generated.ppm", &texture), AssetEntry::from_bytes("previews/map.svg", &preview)]; let manifest = MapManifest::new(&self.map.name, content::pms::pms_hash(&pms), &pms, assets, vec![mode], Some("previews/map.svg".into())).map_err(|error| format!("{error:?}"))?; let signed = sign_manifest(manifest, key).map_err(|error| format!("{error:?}"))?; Ok(MapPackage { manifest: signed, pms, assets: BTreeMap::from([("textures/generated.ppm".into(), texture), ("previews/map.svg".into(), preview)]) }) }
}

pub struct MapPackage { pub manifest: SignedMapManifest, pub pms: Vec<u8>, pub assets: BTreeMap<String, Vec<u8>> }

impl MapPackage {
    pub fn deploy(&self, root: &Path) -> Result<(), String> { let package = root.join(&self.manifest.manifest.name); std::fs::create_dir_all(package.join("maps")).map_err(|error| error.to_string())?; for path in self.assets.keys() { if let Some(parent) = package.join(path).parent() { std::fs::create_dir_all(parent).map_err(|error| error.to_string())?; } } std::fs::write(package.join("manifest.json"), serde_json::to_vec_pretty(&self.manifest).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?; std::fs::write(package.join("maps").join(format!("{}.pms", self.manifest.manifest.name)), &self.pms).map_err(|error| error.to_string())?; for (path, bytes) in &self.assets { std::fs::write(package.join(path), bytes).map_err(|error| error.to_string())?; } Ok(()) }
}

fn validate_spawns(map: &MapAsset, mode: MapMode) -> Result<(), String> { let active = |team| map.spawnpoints.iter().any(|spawn| spawn.active && spawn.team == team); match mode { MapMode::CaptureTheFlag | MapMode::Infiltration if !(active(1) && active(2)) => Err("team spawns required".into()), _ if !map.spawnpoints.iter().any(|spawn| spawn.active) => Err("spawn required".into()), _ => Ok(()) } }

fn preview_svg(map: &MapAsset) -> String { let polygons = map.polygons.iter().map(|polygon| format!("<polygon points=\"{},{} {},{} {},{}\"/>", polygon.vertices[0].x, polygon.vertices[0].y, polygon.vertices[1].x, polygon.vertices[1].y, polygon.vertices[2].x, polygon.vertices[2].y)).collect::<String>(); format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1200 700\"><rect width=\"1200\" height=\"700\" fill=\"rgb({},{},{})\"/><g fill=\"#596568\">{polygons}</g></svg>", map.background_top[0], map.background_top[1], map.background_top[2]) }

fn procedural_texture() -> Vec<u8> { b"P3\n2 2\n255\n70 80 82 95 105 107 95 105 107 70 80 82\n".to_vec() }
