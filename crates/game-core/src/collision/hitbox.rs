use super::{CollisionMask, CollisionWorld, SweepHit};
use crate::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyRegion {
    Head,
    Chest,
    Legs,
}

#[derive(Clone, Copy, Debug)]
pub struct BodyPart {
    pub offset: Vec2,
    pub radius: f32,
    pub region: BodyRegion,
}

#[derive(Clone, Debug)]
pub struct BodyShape {
    parts: Vec<BodyPart>,
}

impl BodyShape {
    pub fn standing() -> Self {
        Self {
            parts: vec![
                BodyPart {
                    offset: Vec2 { x: 0.0, y: -11.0 },
                    radius: 5.0,
                    region: BodyRegion::Head,
                },
                BodyPart {
                    offset: Vec2 { x: 0.0, y: 0.0 },
                    radius: 10.0,
                    region: BodyRegion::Chest,
                },
                BodyPart {
                    offset: Vec2 { x: 0.0, y: 8.0 },
                    radius: 8.0,
                    region: BodyRegion::Legs,
                },
            ],
        }
    }

    pub fn circle(radius: f32) -> Self {
        Self {
            parts: vec![BodyPart {
                offset: Vec2::default(),
                radius,
                region: BodyRegion::Chest,
            }],
        }
    }

    /// Lowering a stance drops the head and torso; the feet stay on the same line as the standing
    /// pose so that crouching or going prone never lifts the body off the ground it rests on.
    pub fn crouching() -> Self {
        Self::pose([-5.0, 1.0, 9.0], [4.5, 8.0, 7.0])
    }

    pub fn prone() -> Self {
        Self {
            parts: vec![
                BodyPart {
                    offset: Vec2 { x: 8.0, y: 7.0 },
                    radius: 4.5,
                    region: BodyRegion::Head,
                },
                BodyPart {
                    offset: Vec2 { x: 0.0, y: 9.0 },
                    radius: 7.0,
                    region: BodyRegion::Chest,
                },
                BodyPart {
                    offset: Vec2 { x: -9.0, y: 11.0 },
                    radius: 5.0,
                    region: BodyRegion::Legs,
                },
            ],
        }
    }

    fn pose(offsets: [f32; 3], radii: [f32; 3]) -> Self {
        Self {
            parts: vec![
                BodyPart {
                    offset: Vec2 {
                        x: 0.0,
                        y: offsets[0],
                    },
                    radius: radii[0],
                    region: BodyRegion::Head,
                },
                BodyPart {
                    offset: Vec2 {
                        x: 0.0,
                        y: offsets[1],
                    },
                    radius: radii[1],
                    region: BodyRegion::Chest,
                },
                BodyPart {
                    offset: Vec2 {
                        x: 0.0,
                        y: offsets[2],
                    },
                    radius: radii[2],
                    region: BodyRegion::Legs,
                },
            ],
        }
    }

    pub fn height(&self) -> f32 {
        let top = self
            .parts
            .iter()
            .map(|part| part.offset.y - part.radius)
            .fold(f32::INFINITY, f32::min);
        let bottom = self
            .parts
            .iter()
            .map(|part| part.offset.y + part.radius)
            .fold(f32::NEG_INFINITY, f32::max);
        bottom - top
    }

    pub fn parts(&self) -> impl Iterator<Item = &BodyPart> {
        self.parts.iter()
    }

    /// The head part of this pose, falling back to the first part for single-circle shapes.
    pub fn head(&self) -> BodyPart {
        self.parts
            .iter()
            .find(|part| part.region == BodyRegion::Head)
            .or_else(|| self.parts.first())
            .copied()
            .unwrap_or(BodyPart {
                offset: Vec2::default(),
                radius: 0.0,
                region: BodyRegion::Head,
            })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShapeHit {
    pub hit: SweepHit,
    pub region: BodyRegion,
}

impl CollisionWorld {
    /// Whether a low stance has room to stand up. Only the volume between the crouched head and
    /// the standing head is tested: a crouched body already rests against the floor, so sweeping
    /// the whole shape would report that floor as a blocking ceiling and pin the player down.
    pub fn has_standing_clearance(&self, position: Vec2, mask: CollisionMask) -> bool {
        let crouched = BodyShape::crouching().head();
        let standing = BodyShape::standing().head();
        self.sweep_circle(
            Vec2 {
                x: position.x,
                y: position.y + crouched.offset.y,
            },
            Vec2 {
                x: position.x,
                y: position.y + standing.offset.y,
            },
            standing.radius,
            mask,
        )
        .is_none()
    }

    pub fn sweep_shape(
        &self,
        start: Vec2,
        end: Vec2,
        shape: &BodyShape,
        mask: CollisionMask,
    ) -> Option<ShapeHit> {
        shape
            .parts()
            .filter_map(|part| {
                let offset_start = add(start, part.offset);
                let offset_end = add(end, part.offset);
                self.sweep_circle(offset_start, offset_end, part.radius, mask)
                    .map(|mut hit| {
                        hit.position = subtract(hit.position, part.offset);
                        ShapeHit {
                            hit,
                            region: part.region,
                        }
                    })
            })
            .min_by(|left, right| left.hit.time.total_cmp(&right.hit.time))
    }
}

fn add(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn subtract(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}
