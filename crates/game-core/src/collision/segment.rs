use crate::Vec2;

pub(crate) fn segment_point_distance_squared(start: Vec2, end: Vec2, point: Vec2) -> f32 {
    let segment = Vec2 {
        x: end.x - start.x,
        y: end.y - start.y,
    };
    let length_squared = segment.x * segment.x + segment.y * segment.y;
    if length_squared == 0.0 {
        return (point.x - start.x).powi(2) + (point.y - start.y).powi(2);
    }
    let projection = (((point.x - start.x) * segment.x + (point.y - start.y) * segment.y)
        / length_squared)
        .clamp(0.0, 1.0);
    let closest = Vec2 {
        x: start.x + segment.x * projection,
        y: start.y + segment.y * projection,
    };
    (point.x - closest.x).powi(2) + (point.y - closest.y).powi(2)
}

pub(crate) fn segment_circle_t(start: Vec2, end: Vec2, center: Vec2, radius: f32) -> Option<f32> {
    let segment = Vec2 {
        x: end.x - start.x,
        y: end.y - start.y,
    };
    let length_squared = segment.x * segment.x + segment.y * segment.y;
    if length_squared == 0.0 {
        return (segment_point_distance_squared(start, end, center) <= radius * radius)
            .then_some(0.0);
    }
    let t = (((center.x - start.x) * segment.x + (center.y - start.y) * segment.y)
        / length_squared)
        .clamp(0.0, 1.0);
    (segment_point_distance_squared(start, end, center) <= radius * radius).then_some(t)
}

/// Lifts a projectile clear of the surface it just struck so the next tick does not re-hit it.
pub(crate) fn nudge(pos: Vec2, normal: Vec2) -> Vec2 {
    Vec2 {
        x: pos.x + normal.x * 0.5,
        y: pos.y + normal.y * 0.5,
    }
}
