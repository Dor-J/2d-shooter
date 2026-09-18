//! The three modifiers, each orthogonal to the mode and to the other two.
//!
//! A modifier wraps a mode's rules rather than branching through them, which is what lets
//! Realistic Survival CTF be an ordinary combination instead of a special case.

pub mod advance;
pub mod realistic;
pub mod survival;

pub use advance::{AdvanceConfig, Unlocked};
pub use realistic::{has_line_of_sight, visibility_between, Viewer, Visibility};
pub use survival::{standing as survival_standing, RoundStanding, SurvivalConfig, Survivor};
