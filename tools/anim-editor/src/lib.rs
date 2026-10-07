#![forbid(unsafe_code)]

use content::anim::{encode, parse, AnimError, Clip, CLIP_VERSION, MAX_FRAMES};

pub fn blank_clip(
    id: u32,
    name: &str,
    frames: u32,
    speed: u32,
    looping: bool,
) -> Result<Vec<u8>, AnimError> {
    if frames == 0 || frames as usize > MAX_FRAMES {
        return Err(AnimError::CountOutOfRange {
            field: "frames",
            value: i32::try_from(frames).unwrap_or(i32::MAX),
            max: MAX_FRAMES,
        });
    }
    let mut points = Vec::with_capacity(frames as usize);
    for _ in 0..frames {
        points.push(Vec::new());
    }
    encode(&Clip {
        version: CLIP_VERSION,
        id,
        name: name.to_owned(),
        frames,
        speed,
        looping,
        points,
    })
}

pub fn check_clip(bytes: &[u8]) -> Result<(), AnimError> {
    parse(bytes).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::{blank_clip, check_clip};

    #[test]
    fn blank_clip_parses_through_the_content_parser() {
        let bytes = blank_clip(0, "blank", 2, 1, false).unwrap();
        check_clip(&bytes).unwrap();
        let clip = content::anim::parse(&bytes).unwrap();
        assert_eq!(clip.id, 0);
        assert_eq!(clip.name, "blank");
        assert_eq!(clip.frames, 2);
        assert_eq!(clip.speed, 1);
        assert!(!clip.looping);
        assert_eq!(clip.points, vec![vec![], vec![]]);
    }
}
