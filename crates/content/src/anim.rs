//! Original animation clips. Version 1 JSON, not the upstream pose text format.
//!
//! Frame and point counts are checked before the typed buffers are allocated.

use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt;

pub const CLIP_VERSION: u32 = 1;
pub const MAX_FRAMES: usize = 40;
pub const MAX_POINTS: usize = 20;
pub const MAX_NAME_BYTES: usize = 64;
pub const MAX_CLIP_BYTES: usize = 256 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimError {
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
    UnsupportedVersion {
        value: i32,
    },
    InvalidStringLength {
        field: &'static str,
        value: usize,
        max: usize,
    },
    Invalid {
        field: &'static str,
    },
    NonFinite {
        field: &'static str,
    },
}

impl fmt::Display for AnimError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for AnimError {}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Clip {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub frames: u32,
    pub speed: u32,
    #[serde(rename = "loop")]
    pub looping: bool,
    pub points: Vec<Vec<Point>>,
}

pub fn parse(bytes: &[u8]) -> Result<Clip, AnimError> {
    if bytes.len() > MAX_CLIP_BYTES {
        return Err(AnimError::CountOutOfRange {
            field: "document",
            value: i32::try_from(bytes.len()).unwrap_or(i32::MAX),
            max: MAX_CLIP_BYTES,
        });
    }
    let value: Value = serde_json::from_slice(bytes).map_err(|error| {
        if error.is_eof() {
            AnimError::Truncated {
                offset: bytes.len(),
                needed: 1,
                remaining: 0,
            }
        } else {
            AnimError::Invalid { field: "document" }
        }
    })?;
    let Some(object) = value.as_object() else {
        return Err(AnimError::Invalid { field: "document" });
    };

    let version = required_i32(object, "version")?;
    if version != i32::from(CLIP_VERSION as u8) {
        return Err(AnimError::UnsupportedVersion { value: version });
    }
    let id = required_u32(object, "id")?;
    let name = required_name(object)?;
    let frames = frame_count(object)?;
    let speed = required_u32(object, "speed")?;
    if speed == 0 {
        return Err(AnimError::CountOutOfRange {
            field: "speed",
            value: 0,
            max: u32::MAX as usize,
        });
    }
    let Some(looping) = object.get("loop").and_then(Value::as_bool) else {
        return Err(AnimError::Invalid { field: "loop" });
    };
    let Some(rows) = object.get("points").and_then(Value::as_array) else {
        return Err(AnimError::Invalid { field: "points" });
    };
    if rows.len() != frames {
        return Err(AnimError::CountOutOfRange {
            field: "points",
            value: i32::try_from(rows.len()).unwrap_or(i32::MAX),
            max: frames,
        });
    }

    let mut widths = Vec::with_capacity(frames);
    for row in rows {
        let Some(coords) = row.as_array() else {
            return Err(AnimError::Invalid { field: "points" });
        };
        if coords.len() > MAX_POINTS {
            return Err(AnimError::CountOutOfRange {
                field: "points",
                value: i32::try_from(coords.len()).unwrap_or(i32::MAX),
                max: MAX_POINTS,
            });
        }
        widths.push(coords.len());
    }

    let mut points = Vec::with_capacity(frames);
    for (row, width) in rows.iter().zip(widths) {
        let Some(coords) = row.as_array() else {
            return Err(AnimError::Invalid { field: "points" });
        };
        let mut frame = Vec::with_capacity(width);
        for coord in coords {
            frame.push(read_point(coord)?);
        }
        points.push(frame);
    }

    Ok(Clip {
        version: CLIP_VERSION,
        id,
        name,
        frames: u32::try_from(frames).expect("frames max is 40"),
        speed,
        looping,
        points,
    })
}

pub fn encode(clip: &Clip) -> Result<Vec<u8>, AnimError> {
    validate(clip)?;
    serde_json::to_vec_pretty(clip).map_err(|_| AnimError::Invalid { field: "document" })
}

fn validate(clip: &Clip) -> Result<(), AnimError> {
    if clip.version != CLIP_VERSION {
        return Err(AnimError::UnsupportedVersion {
            value: i32::try_from(clip.version).unwrap_or(i32::MAX),
        });
    }
    if clip.name.len() > MAX_NAME_BYTES {
        return Err(AnimError::InvalidStringLength {
            field: "name",
            value: clip.name.len(),
            max: MAX_NAME_BYTES,
        });
    }
    if clip.frames == 0 || clip.frames as usize > MAX_FRAMES {
        return Err(AnimError::CountOutOfRange {
            field: "frames",
            value: i32::try_from(clip.frames).unwrap_or(i32::MAX),
            max: MAX_FRAMES,
        });
    }
    if clip.speed == 0 {
        return Err(AnimError::CountOutOfRange {
            field: "speed",
            value: 0,
            max: u32::MAX as usize,
        });
    }
    if clip.points.len() != clip.frames as usize {
        return Err(AnimError::CountOutOfRange {
            field: "points",
            value: i32::try_from(clip.points.len()).unwrap_or(i32::MAX),
            max: clip.frames as usize,
        });
    }
    for frame in &clip.points {
        if frame.len() > MAX_POINTS {
            return Err(AnimError::CountOutOfRange {
                field: "points",
                value: i32::try_from(frame.len()).unwrap_or(i32::MAX),
                max: MAX_POINTS,
            });
        }
        for point in frame {
            if !point.x.is_finite() || !point.y.is_finite() {
                return Err(AnimError::NonFinite { field: "points" });
            }
        }
    }
    Ok(())
}

fn frame_count(object: &Map<String, Value>) -> Result<usize, AnimError> {
    let frames = required_i32(object, "frames")?;
    if frames < 1 || frames as usize > MAX_FRAMES {
        return Err(AnimError::CountOutOfRange {
            field: "frames",
            value: frames,
            max: MAX_FRAMES,
        });
    }
    Ok(frames as usize)
}

fn required_name(object: &Map<String, Value>) -> Result<String, AnimError> {
    let Some(name) = object.get("name").and_then(Value::as_str) else {
        return Err(AnimError::Invalid { field: "name" });
    };
    if name.len() > MAX_NAME_BYTES {
        return Err(AnimError::InvalidStringLength {
            field: "name",
            value: name.len(),
            max: MAX_NAME_BYTES,
        });
    }
    Ok(name.to_owned())
}

fn required_i32(object: &Map<String, Value>, field: &'static str) -> Result<i32, AnimError> {
    let Some(number) = object.get(field).and_then(Value::as_i64) else {
        return Err(AnimError::Invalid { field });
    };
    i32::try_from(number).map_err(|_| AnimError::Invalid { field })
}

fn required_u32(object: &Map<String, Value>, field: &'static str) -> Result<u32, AnimError> {
    let Some(number) = object.get(field).and_then(Value::as_i64) else {
        return Err(AnimError::Invalid { field });
    };
    u32::try_from(number).map_err(|_| AnimError::Invalid { field })
}

fn read_point(value: &Value) -> Result<Point, AnimError> {
    let Some(object) = value.as_object() else {
        return Err(AnimError::Invalid { field: "points" });
    };
    Ok(Point {
        x: read_finite(object, "x")?,
        y: read_finite(object, "y")?,
    })
}

fn read_finite(object: &Map<String, Value>, field: &'static str) -> Result<f32, AnimError> {
    let Some(number) = object.get(field).and_then(Value::as_f64) else {
        return Err(AnimError::Invalid { field });
    };
    let value = number as f32;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(AnimError::NonFinite { field })
    }
}
