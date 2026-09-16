use super::CollisionPolygon;
use crate::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min: Vec2 {
                x: min_x.min(max_x),
                y: min_y.min(max_y),
            },
            max: Vec2 {
                x: min_x.max(max_x),
                y: min_y.max(max_y),
            },
        }
    }

    pub(super) fn overlaps(self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

pub(super) fn polygon_bounds(polygon: &CollisionPolygon) -> Aabb {
    polygon.vertices.iter().skip(1).fold(
        Aabb::new(
            polygon.vertices[0].x,
            polygon.vertices[0].y,
            polygon.vertices[0].x,
            polygon.vertices[0].y,
        ),
        |bounds, vertex| {
            Aabb::new(
                bounds.min.x.min(vertex.x),
                bounds.min.y.min(vertex.y),
                bounds.max.x.max(vertex.x),
                bounds.max.y.max(vertex.y),
            )
        },
    )
}
