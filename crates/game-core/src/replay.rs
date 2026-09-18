//! Deterministic match recordings.
//!
//! A replay is the header plus the inputs that produced each tick. Playing it is `World::step`.
//! Nothing here invents state the simulation did not already compute.

use crate::fixtures::{FrameInput, WorldDigest};
use crate::{Input, World};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const REPLAY_FORMAT: u32 = 1;
/// Ten minutes at 60 Hz. Longer matches start a new file rather than growing without bound.
pub const MAX_CHUNKS: usize = 36_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayHeader {
    pub format: u32,
    pub protocol: u32,
    pub map: String,
    pub mode: String,
    pub weapon_hash: u32,
    pub seed: u64,
    pub source_revision: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayChunk {
    pub tick: u64,
    pub inputs: BTreeMap<u32, Input>,
    pub checksum: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Replay {
    pub header: ReplayHeader,
    pub chunks: Vec<ReplayChunk>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayError {
    Empty,
    Format,
    Protocol,
    WeaponHash,
    Truncated,
    Checksum,
    TooLong,
}

impl ReplayHeader {
    pub fn new(protocol: u32, map: impl Into<String>, world: &World, seed: u64) -> Self {
        Self {
            format: REPLAY_FORMAT,
            protocol,
            map: map.into(),
            mode: world.mode.clone(),
            weapon_hash: world.weapons.canonical_hash(),
            seed,
            source_revision: "c7596cdca32416cb66339b339105eb1e07e7fbbf".into(),
        }
    }

    pub fn compatible(&self, protocol: u32, weapon_hash: u32) -> Result<(), ReplayError> {
        if self.format != REPLAY_FORMAT {
            return Err(ReplayError::Format);
        }
        if self.protocol != protocol {
            return Err(ReplayError::Protocol);
        }
        if self.weapon_hash != weapon_hash {
            return Err(ReplayError::WeaponHash);
        }
        Ok(())
    }
}

impl Replay {
    pub fn new(header: ReplayHeader) -> Self {
        Self {
            header,
            chunks: Vec::new(),
        }
    }

    pub fn record(
        &mut self,
        world: &World,
        inputs: &BTreeMap<u32, Input>,
    ) -> Result<(), ReplayError> {
        if self.chunks.len() >= MAX_CHUNKS {
            return Err(ReplayError::TooLong);
        }
        self.chunks.push(ReplayChunk {
            tick: world.tick,
            inputs: inputs.clone(),
            checksum: world.digest().as_str().to_string(),
        });
        Ok(())
    }

    /// Steps `initial` through every recorded frame and checks each digest.
    pub fn play(&self, mut world: World) -> Result<World, ReplayError> {
        self.header
            .compatible(self.header.protocol, world.weapons.canonical_hash())?;
        if self.chunks.is_empty() {
            return Err(ReplayError::Empty);
        }
        for chunk in &self.chunks {
            world.step_with_bots(&chunk.inputs);
            if world.digest().as_str() != chunk.checksum {
                return Err(ReplayError::Checksum);
            }
        }
        Ok(world)
    }

    pub fn seek(&self, world: World, tick: u64) -> Result<World, ReplayError> {
        let mut replay = self.clone();
        replay.chunks.retain(|chunk| chunk.tick <= tick);
        if replay.chunks.is_empty() {
            return Err(ReplayError::Empty);
        }
        replay.play(world)
    }

    /// Steps `count` recorded frames, skipping the rest. Used for fast-forward.
    pub fn fast_forward(&self, world: World, count: usize) -> Result<World, ReplayError> {
        let mut replay = self.clone();
        replay.chunks.truncate(count.max(1).min(self.chunks.len()));
        replay.play(world)
    }

    /// Drops a trailing chunk whose checksum was never written — a crash mid-tick.
    pub fn repair_truncated(mut self) -> Result<Self, ReplayError> {
        if let Some(last) = self.chunks.last() {
            if last.checksum.is_empty() {
                self.chunks.pop();
            }
        }
        if self.chunks.is_empty() {
            return Err(ReplayError::Truncated);
        }
        Ok(self)
    }

    pub fn export(&self) -> Result<String, ReplayError> {
        serde_json::to_string(self).map_err(|_| ReplayError::Empty)
    }

    pub fn import(text: &str) -> Result<Self, ReplayError> {
        serde_json::from_str(text).map_err(|_| ReplayError::Empty)
    }

    pub fn as_frames(&self) -> Vec<FrameInput> {
        self.chunks
            .iter()
            .map(|chunk| FrameInput {
                inputs: chunk.inputs.clone(),
            })
            .collect()
    }

    pub fn last_digest(&self) -> Option<WorldDigest> {
        self.chunks
            .last()
            .map(|chunk| WorldDigest::from_hex(&chunk.checksum))
    }
}

impl WorldDigest {
    fn from_hex(value: &str) -> Self {
        Self::from_encoded(value)
    }
}
