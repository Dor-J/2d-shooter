use crate::{Input, World};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimRng {
    state: u64,
}

impl SimRng {
    pub fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    /// A deterministic unit interval used for spread. Not cryptographic.
    pub fn next_unit(&mut self) -> f32 {
        (self.next_u64() % 10_000) as f32 / 10_000.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorldDigest(String);

impl WorldDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_encoded(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameInput {
    pub inputs: BTreeMap<u32, Input>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub source_revision: String,
    pub source_path: String,
    pub seed: u64,
    pub initial: World,
    pub frames: Vec<FrameInput>,
    pub expected: Vec<WorldDigest>,
}

impl Fixture {
    pub fn replay(&self) -> Vec<WorldDigest> {
        let mut world = self.initial.clone();
        world.rng = SimRng::seeded(self.seed);
        self.frames
            .iter()
            .map(|frame| {
                world.step(&frame.inputs);
                world.digest()
            })
            .collect()
    }
}

pub fn replay_fixture_json(fixture_json: &str) -> Result<String, String> {
    let fixture: Fixture = serde_json::from_str(fixture_json).map_err(|error| error.to_string())?;
    serde_json::to_string(&fixture.replay()).map_err(|error| error.to_string())
}

pub(crate) fn digest(bytes: &[u8]) -> WorldDigest {
    let hash = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    WorldDigest(format!("{hash:016x}"))
}
