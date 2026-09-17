# Original-map editor pipeline

`map-editor` imports and exports PMS plus the versioned JSON project format. Its project model exposes polygon vertices/types, texture UVs, scenery and layers, colliders, all spawn/objective/bonus/gun types, waypoints, weather, backgrounds, footsteps, and jet fuel through `MapAsset`.

The editor model provides multi-selection, configurable grid snapping and zoom, undo/redo, copy/paste, JSON prefabs, validation, SVG previews, signed packaging, deployment, and authoritative offline test-play. Run `map-editor <output-directory>` with `MAP_SIGNING_KEY_HEX` to generate and deploy all 97 original compatible maps.

The maps intentionally reuse only the historical reference names and mode prefixes. Geometry, colors, metadata, previews, and procedural texture content are generated in this repository and do not reproduce upstream layouts or assets.
