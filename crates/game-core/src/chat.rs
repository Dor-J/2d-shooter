//! Chat that is not just a string: who may see it, how often it may be sent, and what is a
//! command rather than a remark.
//!
//! Visibility is decided here so Realistic and Survival cannot be argued with on the client.

use serde::{Deserialize, Serialize};

/// Who a line is for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatScope {
    #[default]
    All,
    Team,
    Server,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatLine {
    pub player: u32,
    pub name: String,
    pub text: String,
    pub scope: ChatScope,
}

/// `^` at the start of a line is team chat, the same shorthand the original used.
pub fn parse_chat(text: &str, requested: ChatScope) -> (ChatScope, String) {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix('^') {
        return (ChatScope::Team, rest.trim().to_string());
    }
    (requested, trimmed.to_string())
}

/// A small operator word list. Off until `censor` is asked for.
/// Acceptance evidence: rust:chat:censor
pub fn censor(text: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    let mut out = text.to_string();
    for word in ["fuck", "shit", "cunt"] {
        if let Some(start) = out.to_ascii_lowercase().find(word) {
            out.replace_range(start..start + word.len(), "***");
        }
    }
    out
}

/// Whether this recipient may see that line.
///
/// Team chat stays on the sender's side. Server lines are always shown. Open chat is hidden from
/// the other team in Realistic, and from the living in Survival when the sender is dead — the
/// dead may talk to each other, not to the people still playing.
#[allow(clippy::too_many_arguments)]
pub fn may_see(
    line: &ChatLine,
    recipient: u32,
    recipient_team: u8,
    sender_team: u8,
    recipient_alive: bool,
    sender_alive: bool,
    realistic: bool,
    survival: bool,
) -> bool {
    if line.scope == ChatScope::Server || line.player == recipient {
        return true;
    }
    if line.scope == ChatScope::Team {
        return recipient_team != 0 && recipient_team == sender_team;
    }
    if survival && !sender_alive && recipient_alive {
        return false;
    }
    if realistic && sender_team != 0 && recipient_team != 0 && sender_team != recipient_team {
        return false;
    }
    true
}

/// How many messages one player may send in a window. The first second is free; after that the
/// bucket is what stops a flood.
pub const CHAT_WINDOW_TICKS: u32 = 60;
pub const CHAT_BURST: u8 = 4;

#[derive(Clone, Debug, Default)]
pub struct Flood {
    pub sent: u8,
    pub ticks: u32,
}

impl Flood {
    pub fn allow(&mut self) -> bool {
        if self.sent >= CHAT_BURST {
            return false;
        }
        self.sent = self.sent.saturating_add(1);
        true
    }

    pub fn tick(&mut self) {
        if self.ticks >= CHAT_WINDOW_TICKS {
            self.sent = 0;
            self.ticks = 0;
        } else {
            self.ticks = self.ticks.saturating_add(1);
        }
    }
}
