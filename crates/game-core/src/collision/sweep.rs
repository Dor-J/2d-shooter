use super::PolygonKind;
use crate::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct RayHit {
    pub time: f32,
    pub position: Vec2,
    pub polygon: usize,
    /// Unit normal of the edge that was struck, pointing back towards where the ray came from.
    /// Projectiles that bounce or skip off terrain need it; those that stop can ignore it.
    pub normal: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct SweepHit {
    pub time: f32,
    pub position: Vec2,
    pub normal: Vec2,
    pub polygon: usize,
    pub kind: PolygonKind,
}

#[derive(Clone, Copy, Debug)]
pub struct Contact {
    pub position: Vec2,
    pub normal: Vec2,
    pub penetration: f32,
    pub polygon: usize,
    pub kind: PolygonKind,
}

#[derive(Clone, Debug)]
pub struct ContactManifold {
    pub contacts: Vec<Contact>,
}

impl SweepHit {
    pub fn manifold(self) -> ContactManifold {
        ContactManifold {
            contacts: vec![Contact {
                position: self.position,
                normal: self.normal,
                penetration: 0.0,
                polygon: self.polygon,
                kind: self.kind,
            }],
        }
    }
}
