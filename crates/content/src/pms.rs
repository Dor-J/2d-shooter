use crate::is_safe_content_path;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct ContentLimits {
    pub max_polygons: usize,
    pub max_sector_radius: usize,
    pub max_props: usize,
    pub max_colliders: usize,
    pub max_spawnpoints: usize,
    pub max_waypoints: usize,
}

impl Default for ContentLimits {
    fn default() -> Self {
        Self {
            max_polygons: 5_000,
            max_sector_radius: 25,
            max_props: 500,
            max_colliders: 128,
            max_spawnpoints: 255,
            max_waypoints: 5_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentError {
    Truncated {
        offset: usize,
        needed: usize,
        remaining: usize,
    },
    CountOutOfRange {
        field: &'static str,
        value: i32,
        max: usize,
    },
    InvalidStringLength {
        field: &'static str,
        value: usize,
        max: usize,
    },
    InvalidUtf8 {
        field: &'static str,
    },
    UnsafePath {
        field: &'static str,
    },
    InvalidReference {
        field: &'static str,
        value: usize,
        count: usize,
    },
    NonFinite {
        field: &'static str,
    },
    ArithmeticOverflow {
        field: &'static str,
    },
    UnsupportedVersion {
        value: i32,
    },
}

impl fmt::Display for ContentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ContentError {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rhw: f32,
    pub color: [u8; 4],
    pub u: f32,
    pub v: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub vertices: [Vertex; 3],
    pub normals: [[f32; 3]; 3],
    pub kind: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sector {
    pub polygons: Vec<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prop {
    pub active: bool,
    pub style: u16,
    pub width: i32,
    pub height: i32,
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
    pub alpha: u8,
    pub color: [u8; 4],
    pub level: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scenery {
    pub filename: String,
    pub date: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collider {
    pub active: bool,
    pub position: [f32; 2],
    pub radius: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spawnpoint {
    pub active: bool,
    pub x: i32,
    pub y: i32,
    pub team: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Waypoint {
    pub active: bool,
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub jetpack: bool,
    pub path_num: u8,
    pub action: u8,
    pub connections_num: i32,
    pub connections: [i32; 20],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapAsset {
    pub hash: u32,
    pub version: i32,
    pub name: String,
    pub texture: String,
    pub background_top: [u8; 4],
    pub background_bottom: [u8; 4],
    pub start_jet: i32,
    pub grenade_packs: u8,
    pub medikits: u8,
    pub weather: u8,
    pub steps: u8,
    pub random_id: i32,
    pub polygons: Vec<Polygon>,
    pub sectors_division: i32,
    pub sectors_num: i32,
    pub sectors: Vec<Sector>,
    pub props: Vec<Prop>,
    pub scenery: Vec<Scenery>,
    pub colliders: Vec<Collider>,
    pub spawnpoints: Vec<Spawnpoint>,
    pub waypoints: Vec<Waypoint>,
}

pub fn parse(bytes: &[u8], limits: &ContentLimits) -> Result<MapAsset, ContentError> {
    let hash = pms_hash(bytes);
    let mut cursor = Cursor::new(bytes);
    let version = cursor.i32()?;
    if version != 1 {
        return Err(ContentError::UnsupportedVersion { value: version });
    }
    let name = cursor.fixed_string("name", 38)?;
    let texture = cursor.fixed_string("texture", 24)?;
    validate_relative_path("texture", &texture)?;
    let background_top = cursor.color()?;
    let background_bottom = cursor.color()?;
    let start_jet = cursor.i32()?;
    let grenade_packs = cursor.u8()?;
    let medikits = cursor.u8()?;
    let weather = cursor.u8()?;
    let steps = cursor.u8()?;
    let random_id = cursor.i32()?;

    let polygon_count = cursor.count("polygons", limits.max_polygons)?;
    let mut polygons = Vec::with_capacity(polygon_count);
    for _ in 0..polygon_count {
        polygons.push(cursor.polygon()?);
    }

    let sectors_division = cursor.i32()?;
    let sectors_num = cursor.i32()?;
    if sectors_num < 0 || sectors_num as usize > limits.max_sector_radius {
        return Err(ContentError::CountOutOfRange {
            field: "sector radius",
            value: sectors_num,
            max: limits.max_sector_radius,
        });
    }
    let width = (sectors_num as usize)
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(ContentError::ArithmeticOverflow { field: "sectors" })?;
    let sector_count = width
        .checked_mul(width)
        .ok_or(ContentError::ArithmeticOverflow { field: "sectors" })?;
    let mut sectors = Vec::with_capacity(sector_count);
    for _ in 0..sector_count {
        let count = usize::from(cursor.u16()?);
        if count > limits.max_polygons {
            return Err(ContentError::CountOutOfRange {
                field: "sector polygons",
                value: count as i32,
                max: limits.max_polygons,
            });
        }
        let mut indices = Vec::with_capacity(count);
        for _ in 0..count {
            let index = cursor.u16()?;
            if usize::from(index) > polygon_count {
                return Err(ContentError::InvalidReference {
                    field: "sector polygon",
                    value: usize::from(index),
                    count: polygon_count,
                });
            }
            indices.push(index);
        }
        sectors.push(Sector { polygons: indices });
    }

    let prop_count = cursor.count("props", limits.max_props)?;
    let mut props = Vec::with_capacity(prop_count);
    for _ in 0..prop_count {
        props.push(cursor.prop()?);
    }
    let scenery_count = cursor.count("scenery", limits.max_props)?;
    let mut scenery = Vec::with_capacity(scenery_count);
    for _ in 0..scenery_count {
        let filename = cursor.fixed_string("scenery filename", 50)?;
        validate_relative_path("scenery filename", &filename)?;
        scenery.push(Scenery {
            filename,
            date: cursor.i32()?,
        });
    }
    let collider_count = cursor.count("colliders", limits.max_colliders)?;
    let mut colliders = Vec::with_capacity(collider_count);
    for _ in 0..collider_count {
        colliders.push(cursor.collider()?);
    }
    let spawnpoint_count = cursor.count("spawnpoints", limits.max_spawnpoints)?;
    let mut spawnpoints = Vec::with_capacity(spawnpoint_count);
    for _ in 0..spawnpoint_count {
        spawnpoints.push(cursor.spawnpoint()?);
    }
    let waypoint_count = cursor.count("waypoints", limits.max_waypoints)?;
    let mut waypoints = Vec::with_capacity(waypoint_count);
    for _ in 0..waypoint_count {
        waypoints.push(cursor.waypoint()?);
    }

    Ok(MapAsset {
        hash,
        version,
        name,
        texture,
        background_top,
        background_bottom,
        start_jet,
        grenade_packs,
        medikits,
        weather,
        steps,
        random_id,
        polygons,
        sectors_division,
        sectors_num,
        sectors,
        props,
        scenery,
        colliders,
        spawnpoints,
        waypoints,
    })
}

pub fn pms_hash(bytes: &[u8]) -> u32 {
    bytes.iter().fold(5_381_u32, |crc, byte| {
        let mut table_value = u32::from(*byte ^ ((crc >> 24) as u8)) << 24;
        for _ in 0..8 {
            table_value = if table_value & 0x8000_0000 != 0 {
                (table_value << 1) ^ 0x04c1_1db7
            } else {
                table_value << 1
            };
        }
        table_value ^ (crc << 8)
    })
}

pub fn encode(map: &MapAsset) -> Result<Vec<u8>, ContentError> {
    if map.version != 1 {
        return Err(ContentError::UnsupportedVersion { value: map.version });
    }
    validate_relative_path("texture", &map.texture)?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&map.version.to_le_bytes());
    push_fixed_string(&mut bytes, "name", &map.name, 38)?;
    push_fixed_string(&mut bytes, "texture", &map.texture, 24)?;
    push_color(&mut bytes, map.background_top);
    push_color(&mut bytes, map.background_bottom);
    bytes.extend_from_slice(&map.start_jet.to_le_bytes());
    bytes.extend_from_slice(&[map.grenade_packs, map.medikits, map.weather, map.steps]);
    bytes.extend_from_slice(&map.random_id.to_le_bytes());
    push_count(&mut bytes, "polygons", map.polygons.len())?;
    for polygon in &map.polygons {
        for vertex in polygon.vertices {
            for value in [vertex.x, vertex.y, vertex.z, vertex.rhw] {
                push_f32(&mut bytes, "vertex", value)?;
            }
            push_color(&mut bytes, vertex.color);
            push_f32(&mut bytes, "texture u", vertex.u)?;
            push_f32(&mut bytes, "texture v", vertex.v)?;
        }
        for normal in polygon.normals {
            for value in normal {
                push_f32(&mut bytes, "normal", value)?;
            }
        }
        bytes.push(polygon.kind);
    }
    bytes.extend_from_slice(&map.sectors_division.to_le_bytes());
    bytes.extend_from_slice(&map.sectors_num.to_le_bytes());
    for sector in &map.sectors {
        let count =
            u16::try_from(sector.polygons.len()).map_err(|_| ContentError::CountOutOfRange {
                field: "sector polygons",
                value: i32::MAX,
                max: u16::MAX as usize,
            })?;
        bytes.extend_from_slice(&count.to_le_bytes());
        for polygon in &sector.polygons {
            bytes.extend_from_slice(&polygon.to_le_bytes());
        }
    }
    push_count(&mut bytes, "props", map.props.len())?;
    for prop in &map.props {
        bytes.extend_from_slice(&[u8::from(prop.active), 0]);
        bytes.extend_from_slice(&prop.style.to_le_bytes());
        bytes.extend_from_slice(&prop.width.to_le_bytes());
        bytes.extend_from_slice(&prop.height.to_le_bytes());
        for value in [
            prop.position[0],
            prop.position[1],
            prop.rotation,
            prop.scale[0],
            prop.scale[1],
        ] {
            push_f32(&mut bytes, "prop", value)?;
        }
        bytes.extend_from_slice(&[prop.alpha, 0, 0, 0]);
        push_color(&mut bytes, prop.color);
        bytes.extend_from_slice(&[prop.level, 0, 0, 0]);
    }
    push_count(&mut bytes, "scenery", map.scenery.len())?;
    for scenery in &map.scenery {
        validate_relative_path("scenery filename", &scenery.filename)?;
        push_fixed_string(&mut bytes, "scenery filename", &scenery.filename, 50)?;
        bytes.extend_from_slice(&scenery.date.to_le_bytes());
    }
    push_count(&mut bytes, "colliders", map.colliders.len())?;
    for collider in &map.colliders {
        bytes.extend_from_slice(&[u8::from(collider.active), 0, 0, 0]);
        for value in [collider.position[0], collider.position[1], collider.radius] {
            push_f32(&mut bytes, "collider", value)?;
        }
    }
    push_count(&mut bytes, "spawnpoints", map.spawnpoints.len())?;
    for spawn in &map.spawnpoints {
        bytes.extend_from_slice(&[u8::from(spawn.active), 0, 0, 0]);
        for value in [spawn.x, spawn.y, spawn.team] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
    }
    push_count(&mut bytes, "waypoints", map.waypoints.len())?;
    for waypoint in &map.waypoints {
        bytes.extend_from_slice(&[u8::from(waypoint.active), 0, 0, 0]);
        for value in [waypoint.id, waypoint.x, waypoint.y] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&[
            u8::from(waypoint.left),
            u8::from(waypoint.right),
            u8::from(waypoint.up),
            u8::from(waypoint.down),
            u8::from(waypoint.jetpack),
            waypoint.path_num,
            waypoint.action,
            0,
            0,
            0,
            0,
            0,
        ]);
        bytes.extend_from_slice(&waypoint.connections_num.to_le_bytes());
        for connection in waypoint.connections {
            bytes.extend_from_slice(&connection.to_le_bytes());
        }
    }
    Ok(bytes)
}

fn push_count(bytes: &mut Vec<u8>, field: &'static str, count: usize) -> Result<(), ContentError> {
    let value = i32::try_from(count).map_err(|_| ContentError::CountOutOfRange {
        field,
        value: i32::MAX,
        max: i32::MAX as usize,
    })?;
    bytes.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

fn push_fixed_string(
    bytes: &mut Vec<u8>,
    field: &'static str,
    value: &str,
    size: usize,
) -> Result<(), ContentError> {
    if value.len() > size || value.len() > u8::MAX as usize {
        return Err(ContentError::InvalidStringLength {
            field,
            value: value.len(),
            max: size,
        });
    }
    bytes.push(value.len() as u8);
    bytes.extend_from_slice(value.as_bytes());
    bytes.resize(bytes.len() + size - value.len(), 0);
    Ok(())
}

fn push_color(bytes: &mut Vec<u8>, color: [u8; 4]) {
    bytes.extend_from_slice(&[color[2], color[1], color[0], color[3]]);
}

fn push_f32(bytes: &mut Vec<u8>, field: &'static str, value: f32) -> Result<(), ContentError> {
    if !value.is_finite() {
        return Err(ContentError::NonFinite { field });
    }
    bytes.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

fn validate_relative_path(field: &'static str, value: &str) -> Result<(), ContentError> {
    if !is_safe_content_path(value) {
        Err(ContentError::UnsafePath { field })
    } else {
        Ok(())
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], ContentError> {
        let remaining = self.bytes.len().saturating_sub(self.offset);
        if count > remaining {
            return Err(ContentError::Truncated {
                offset: self.offset,
                needed: count,
                remaining,
            });
        }
        let start = self.offset;
        self.offset += count;
        Ok(&self.bytes[start..self.offset])
    }

    fn skip(&mut self, count: usize) -> Result<(), ContentError> {
        self.take(count).map(|_| ())
    }

    fn u8(&mut self) -> Result<u8, ContentError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, ContentError> {
        let bytes: [u8; 2] = self.take(2)?.try_into().expect("slice length checked");
        Ok(u16::from_le_bytes(bytes))
    }

    fn i32(&mut self) -> Result<i32, ContentError> {
        let bytes: [u8; 4] = self.take(4)?.try_into().expect("slice length checked");
        Ok(i32::from_le_bytes(bytes))
    }

    fn f32(&mut self, field: &'static str) -> Result<f32, ContentError> {
        let bytes: [u8; 4] = self.take(4)?.try_into().expect("slice length checked");
        let value = f32::from_le_bytes(bytes);
        if value.is_finite() {
            Ok(value)
        } else {
            Err(ContentError::NonFinite { field })
        }
    }

    fn count(&mut self, field: &'static str, max: usize) -> Result<usize, ContentError> {
        let value = self.i32()?;
        if value < 0 || value as usize > max {
            Err(ContentError::CountOutOfRange { field, value, max })
        } else {
            Ok(value as usize)
        }
    }

    fn fixed_string(
        &mut self,
        field: &'static str,
        field_size: usize,
    ) -> Result<String, ContentError> {
        let length = usize::from(self.u8()?);
        let bytes = self.take(field_size)?;
        if length > field_size {
            return Err(ContentError::InvalidStringLength {
                field,
                value: length,
                max: field_size,
            });
        }
        std::str::from_utf8(&bytes[..length])
            .map(str::to_owned)
            .map_err(|_| ContentError::InvalidUtf8 { field })
    }

    fn color(&mut self) -> Result<[u8; 4], ContentError> {
        let bytes = self.take(4)?;
        Ok([bytes[2], bytes[1], bytes[0], bytes[3]])
    }

    fn vertex(&mut self) -> Result<Vertex, ContentError> {
        Ok(Vertex {
            x: self.f32("vertex x")?,
            y: self.f32("vertex y")?,
            z: self.f32("vertex z")?,
            rhw: self.f32("vertex rhw")?,
            color: self.color()?,
            u: self.f32("texture u")?,
            v: self.f32("texture v")?,
        })
    }

    fn vector(&mut self) -> Result<[f32; 3], ContentError> {
        Ok([
            self.f32("normal x")?,
            self.f32("normal y")?,
            self.f32("normal z")?,
        ])
    }

    fn polygon(&mut self) -> Result<Polygon, ContentError> {
        Ok(Polygon {
            vertices: [self.vertex()?, self.vertex()?, self.vertex()?],
            normals: [self.vector()?, self.vector()?, self.vector()?],
            kind: self.u8()?,
        })
    }

    fn prop(&mut self) -> Result<Prop, ContentError> {
        let active = self.u8()? != 0;
        self.skip(1)?;
        let style = self.u16()?;
        let width = self.i32()?;
        let height = self.i32()?;
        let position = [self.f32("prop x")?, self.f32("prop y")?];
        let rotation = self.f32("prop rotation")?;
        let scale = [self.f32("prop scale x")?, self.f32("prop scale y")?];
        let alpha = self.u8()?;
        self.skip(3)?;
        let color = self.color()?;
        let level = self.u8()?;
        self.skip(3)?;
        Ok(Prop {
            active,
            style,
            width,
            height,
            position,
            rotation,
            scale,
            alpha,
            color,
            level,
        })
    }

    fn collider(&mut self) -> Result<Collider, ContentError> {
        let active = self.u8()? != 0;
        self.skip(3)?;
        Ok(Collider {
            active,
            position: [self.f32("collider x")?, self.f32("collider y")?],
            radius: self.f32("collider radius")?,
        })
    }

    fn spawnpoint(&mut self) -> Result<Spawnpoint, ContentError> {
        let active = self.u8()? != 0;
        self.skip(3)?;
        Ok(Spawnpoint {
            active,
            x: self.i32()?,
            y: self.i32()?,
            team: self.i32()?,
        })
    }

    fn waypoint(&mut self) -> Result<Waypoint, ContentError> {
        let active = self.u8()? != 0;
        self.skip(3)?;
        let id = self.i32()?;
        let x = self.i32()?;
        let y = self.i32()?;
        let left = self.u8()? != 0;
        let right = self.u8()? != 0;
        let up = self.u8()? != 0;
        let down = self.u8()? != 0;
        let jetpack = self.u8()? != 0;
        let path_num = self.u8()?;
        let action = self.u8()?;
        self.skip(5)?;
        let connections_num = self.i32()?;
        let mut connections = [0; 20];
        for connection in &mut connections {
            *connection = self.i32()?;
        }
        Ok(Waypoint {
            active,
            id,
            x,
            y,
            left,
            right,
            up,
            down,
            jetpack,
            path_num,
            action,
            connections_num,
            connections,
        })
    }
}
