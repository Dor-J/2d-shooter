use crate::{ValidatedMap, Vec2};
use serde::{Deserialize, Serialize};

mod broadphase;
mod dynamic;
mod hitbox;
mod materials;
mod sweep;
pub use broadphase::Aabb;
pub use dynamic::{DynamicBody, DynamicBodyKind};
pub use hitbox::{BodyPart, BodyRegion, BodyShape, ShapeHit};
pub use materials::{resolve_material_velocity, MaterialResponse};
pub use sweep::{Contact, ContactManifold, RayHit, SweepHit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionMask {
    Player,
    Bullet,
}

impl CollisionMask {
    pub const PLAYER: Self = Self::Player;
    pub const BULLET: Self = Self::Bullet;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolygonKind {
    Normal,
    OneWay,
    Bouncy,
    Ice,
    Deadly,
    OnlyPlayers,
    OnlyBullets,
}

impl PolygonKind {
    fn allows(self, mask: CollisionMask) -> bool {
        !matches!(
            (self, mask),
            (Self::OnlyPlayers, CollisionMask::Bullet) | (Self::OnlyBullets, CollisionMask::Player)
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CollisionPolygon {
    pub vertices: [Vec2; 3],
    pub kind: PolygonKind,
}

impl CollisionPolygon {
    pub fn triangle(vertices: [Vec2; 3], kind: PolygonKind) -> Self {
        Self { vertices, kind }
    }
}

#[derive(Clone, Debug)]
pub struct CollisionWorld {
    polygons: Vec<CollisionPolygon>,
    bounds: Vec<Aabb>,
}

impl CollisionWorld {
    pub fn new(polygons: Vec<CollisionPolygon>) -> Self {
        let bounds = polygons.iter().map(broadphase::polygon_bounds).collect();
        Self { polygons, bounds }
    }

    pub fn polygons(&self) -> &[CollisionPolygon] {
        &self.polygons
    }

    pub fn from_map(map: &ValidatedMap) -> Self {
        let polygons = map
            .asset()
            .polygons
            .iter()
            .filter_map(|polygon| {
                let kind = match polygon.kind {
                    0 => PolygonKind::Normal,
                    1 => PolygonKind::OnlyBullets,
                    2 => PolygonKind::OnlyPlayers,
                    3 | 24 | 25 => return None,
                    4 => PolygonKind::Ice,
                    5..=7 | 9 | 19 | 20 => PolygonKind::Deadly,
                    18 => PolygonKind::Bouncy,
                    _ => PolygonKind::Normal,
                };
                Some(CollisionPolygon::triangle(
                    polygon.vertices.map(|vertex| Vec2 {
                        x: vertex.x,
                        y: vertex.y,
                    }),
                    kind,
                ))
            })
            .collect();
        Self::new(polygons)
    }

    pub fn query_aabb(&self, bounds: Aabb) -> Vec<usize> {
        self.bounds
            .iter()
            .enumerate()
            .filter_map(|(index, polygon_bounds)| bounds.overlaps(*polygon_bounds).then_some(index))
            .collect()
    }

    pub fn raycast(&self, start: Vec2, end: Vec2, mask: CollisionMask) -> Option<RayHit> {
        self.query_aabb(Aabb::new(start.x, start.y, end.x, end.y))
            .into_iter()
            .map(|index| (index, &self.polygons[index]))
            .filter(|(_, polygon)| polygon.kind.allows(mask))
            .filter(|(_, polygon)| polygon.kind != PolygonKind::OneWay || end.y > start.y)
            .filter_map(|(polygon_index, polygon)| {
                polygon
                    .vertices
                    .iter()
                    .copied()
                    .zip(polygon.vertices.iter().copied().cycle().skip(1))
                    .take(3)
                    .filter_map(|(a, b)| segment_intersection(start, end, a, b))
                    .min_by(|left, right| left.total_cmp(right))
                    .map(|time| RayHit {
                        time,
                        position: Vec2 {
                            x: start.x + (end.x - start.x) * time,
                            y: start.y + (end.y - start.y) * time,
                        },
                        polygon: polygon_index,
                    })
            })
            .min_by(|left, right| left.time.total_cmp(&right.time))
    }

    pub fn sweep_circle(
        &self,
        start: Vec2,
        end: Vec2,
        radius: f32,
        mask: CollisionMask,
    ) -> Option<SweepHit> {
        if !radius.is_finite() || radius < 0.0 {
            return None;
        }
        self.query_aabb(Aabb::new(
            start.x.min(end.x) - radius,
            start.y.min(end.y) - radius,
            start.x.max(end.x) + radius,
            start.y.max(end.y) + radius,
        ))
        .into_iter()
        .map(|index| (index, &self.polygons[index]))
        .filter(|(_, polygon)| polygon.kind.allows(mask))
        .filter(|(_, polygon)| polygon.kind != PolygonKind::OneWay || end.y > start.y)
        .filter_map(|(polygon_index, polygon)| {
            polygon
                .vertices
                .iter()
                .copied()
                .zip(polygon.vertices.iter().copied().cycle().skip(1))
                .take(3)
                .filter_map(|(a, b)| sweep_circle_edge(start, end, radius, a, b))
                .filter(|(_, normal)| polygon.kind != PolygonKind::OneWay || normal.y < -0.5)
                .min_by(|left, right| left.0.total_cmp(&right.0))
                .map(|(time, normal)| SweepHit {
                    time,
                    position: lerp(start, end, time),
                    normal,
                    polygon: polygon_index,
                    kind: polygon.kind,
                })
        })
        .min_by(|left, right| left.time.total_cmp(&right.time))
    }
}

fn sweep_circle_edge(start: Vec2, end: Vec2, radius: f32, a: Vec2, b: Vec2) -> Option<(f32, Vec2)> {
    let edge = subtract(b, a);
    let edge_length = length(edge);
    if edge_length <= f32::EPSILON {
        return sweep_point(start, end, radius, a);
    }
    let tangent = scale(edge, 1.0 / edge_length);
    let line_normal = Vec2 {
        x: -tangent.y,
        y: tangent.x,
    };
    let start_distance = dot(subtract(start, a), line_normal);
    let end_distance = dot(subtract(end, a), line_normal);
    let target_distance = if start_distance >= 0.0 {
        radius
    } else {
        -radius
    };
    let distance_delta = end_distance - start_distance;
    let line_hit = if distance_delta.abs() <= f32::EPSILON {
        None
    } else {
        let time = (target_distance - start_distance) / distance_delta;
        let center = lerp(start, end, time);
        let contact = subtract(center, scale(line_normal, target_distance));
        let projection = dot(subtract(contact, a), tangent);
        ((0.0..=1.0).contains(&time) && (0.0..=edge_length).contains(&projection)).then(|| {
            (
                time,
                scale(line_normal, if target_distance >= 0.0 { 1.0 } else { -1.0 }),
            )
        })
    };
    [
        line_hit,
        sweep_point(start, end, radius, a),
        sweep_point(start, end, radius, b),
    ]
    .into_iter()
    .flatten()
    .min_by(|left, right| left.0.total_cmp(&right.0))
}

fn sweep_point(start: Vec2, end: Vec2, radius: f32, point: Vec2) -> Option<(f32, Vec2)> {
    let motion = subtract(end, start);
    let offset = subtract(start, point);
    let a = dot(motion, motion);
    if a <= f32::EPSILON {
        return None;
    }
    let b = 2.0 * dot(offset, motion);
    let c = dot(offset, offset) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let time = (-b - discriminant.sqrt()) / (2.0 * a);
    if !(0.0..=1.0).contains(&time) {
        return None;
    }
    let center = lerp(start, end, time);
    let normal = normalize(subtract(center, point))?;
    Some((time, normal))
}

fn segment_intersection(start: Vec2, end: Vec2, a: Vec2, b: Vec2) -> Option<f32> {
    let ray = Vec2 {
        x: end.x - start.x,
        y: end.y - start.y,
    };
    let edge = Vec2 {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let denominator = cross(ray, edge);
    if denominator.abs() <= f32::EPSILON {
        return None;
    }
    let offset = Vec2 {
        x: a.x - start.x,
        y: a.y - start.y,
    };
    let time = cross(offset, edge) / denominator;
    let edge_time = cross(offset, ray) / denominator;
    (0.0..=1.0)
        .contains(&time)
        .then_some(())
        .filter(|_| (0.0..=1.0).contains(&edge_time))
        .map(|_| time)
}

fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

fn subtract(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

fn scale(value: Vec2, scalar: f32) -> Vec2 {
    Vec2 {
        x: value.x * scalar,
        y: value.y * scalar,
    }
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

fn length(value: Vec2) -> f32 {
    dot(value, value).sqrt()
}

fn normalize(value: Vec2) -> Option<Vec2> {
    let magnitude = length(value);
    (magnitude > f32::EPSILON).then(|| scale(value, 1.0 / magnitude))
}

fn lerp(start: Vec2, end: Vec2, time: f32) -> Vec2 {
    Vec2 {
        x: start.x + (end.x - start.x) * time,
        y: start.y + (end.y - start.y) * time,
    }
}
