//! Legs and body playback. Not the soldier record and not the world.
//!
//! A frame lasts `speed` calls. The next call steps forward, so one cycle is
//! `frames * speed` calls. Without loop, the last frame holds.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Channel {
    pub id: u32,
    pub frame: u32,
    pub speed: u32,
    pub r#loop: bool,
    frames: u32,
    elapsed: u32,
}

impl Channel {
    pub fn start(id: u32, frames: u32, speed: u32, r#loop: bool) -> Self {
        Self {
            id,
            frame: 0,
            speed,
            r#loop,
            frames,
            elapsed: 0,
        }
    }

    fn step(&mut self) {
        if self.frames == 0 || self.speed == 0 {
            return;
        }
        let last = self.frames - 1;
        if !self.r#loop && self.frame >= last && self.elapsed >= self.speed {
            return;
        }
        self.elapsed = self.elapsed.saturating_add(1);
        if self.elapsed <= self.speed {
            return;
        }
        self.elapsed = 1;
        if self.frame < last {
            self.frame += 1;
        } else if self.r#loop {
            self.frame = 0;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimPlayer {
    pub legs: Channel,
    pub body: Channel,
}

impl AnimPlayer {
    pub fn new(legs: Channel, body: Channel) -> Self {
        Self { legs, body }
    }

    pub fn advance(&mut self) {
        self.legs.step();
        self.body.step();
    }
}

#[cfg(test)]
mod tests {
    use super::{AnimPlayer, Channel};

    #[test]
    fn speed_three_two_frames_stays_zero_for_three_calls_then_advances() {
        let mut player =
            AnimPlayer::new(Channel::start(4, 2, 3, true), Channel::start(4, 2, 3, true));
        for _ in 0..3 {
            player.advance();
            assert_eq!(player.legs.frame, 0);
            assert_eq!(player.body.frame, 0);
        }
        player.advance();
        assert_eq!(player.legs.frame, 1);
        assert_eq!(player.body.frame, 1);

        // One cycle is frames * speed calls. The call after that starts again at 0.
        player.advance();
        player.advance();
        assert_eq!(player.legs.frame, 1);
        player.advance();
        assert_eq!(player.legs.frame, 0);
        assert_eq!(player.body.frame, 0);

        let mut once = AnimPlayer::new(
            Channel::start(4, 2, 3, false),
            Channel::start(4, 2, 3, false),
        );
        for _ in 0..4 {
            once.advance();
        }
        assert_eq!(once.legs.frame, 1);
        once.advance();
        assert_eq!(once.legs.frame, 1);
        assert_eq!(once.body.frame, 1);
    }
}
