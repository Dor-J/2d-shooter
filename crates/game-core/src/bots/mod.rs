//! Bots.
//!
//! A bot is a player the server drives. It submits exactly the same `Input` a human's client
//! does — no privileged access, no separate movement path, no damage it could not have dealt — so
//! anything a bot can do, a player could have done, and the authoritative simulation does not need
//! to know which is which.

pub mod combat;
pub mod navigation;
pub mod objectives;
pub mod perception;
pub mod profile;

use serde::{Deserialize, Serialize};

pub use combat::Trigger;
pub use navigation::{Movement, Navigation, Waypoint};
pub use objectives::{BotGoal, Objective};
pub use perception::{Awareness, Contact};
pub use profile::{BotProfile, Difficulty, ProfileError};

use crate::collision::CollisionWorld;
use crate::modes::modifiers::advance::Unlocked;
use crate::modes::objectives_state::Objectives;
use crate::modes::rules::ModeRules;
use crate::weapons::WeaponTable;
use crate::{Input, SimRng, Vec2};

/// Everything the server keeps about one bot between ticks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    pub profile: BotProfile,
    pub awareness: Awareness,
    pub navigation: Navigation,
    pub goal: BotGoal,
    /// Ticks until it is willing to say something again.
    pub chat_cooldown: u16,
}

/// How long a bot keeps quiet between remarks.
pub const CHAT_COOLDOWN_TICKS: u16 = 60 * 20;

impl Bot {
    pub fn new(profile: BotProfile) -> Self {
        Self {
            profile,
            awareness: Awareness::default(),
            navigation: Navigation::default(),
            goal: BotGoal::Fight,
            chat_cooldown: CHAT_COOLDOWN_TICKS,
        }
    }
}

/// Everything a bot needs to know about the match around it this tick.
///
/// Gathered into one borrow rather than passed as six arguments, because a signature long enough
/// to get the order wrong is a signature somebody will get wrong.
#[derive(Clone, Copy, Debug)]
pub struct BotWorld<'a> {
    /// Everybody the bot might see, itself included.
    pub others: &'a [Contact],
    pub rules: &'a ModeRules,
    pub objectives: &'a Objectives,
    pub table: &'a WeaponTable,
    pub collision: &'a CollisionWorld,
    /// Somewhere to head for when there is nothing else to do.
    pub spawns: &'a [Vec2],
    /// Kits and dropped guns the bot may walk over.
    pub loot: &'a [Vec2],
}

/// Everything a bot needs to know about itself this tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BotView {
    pub me: Contact,
    pub fuel: f32,
    pub ammo: u16,
    pub magazine: u16,
    pub reloading: bool,
    pub grenades: u8,
    pub holding: crate::weapons::WeaponKind,
    pub unlocked: Unlocked,
    /// How many are on this bot's side, which decides whether it defends.
    pub team_size: usize,
}

/// Works out the `Input` one bot would send this tick.
///
/// Deterministic for a given RNG state: two runs of the same match produce the same bots, which is
/// what lets a recorded match be replayed at all.
pub fn think(
    bot: &mut Bot,
    view: BotView,
    world: BotWorld<'_>,
    rng: &mut SimRng,
    seq: u32,
) -> Input {
    let BotWorld {
        others,
        rules,
        objectives,
        table,
        collision,
        spawns,
        loot,
    } = world;
    bot.chat_cooldown = bot.chat_cooldown.saturating_sub(1);

    if !view.me.alive {
        bot.awareness.clear();
        bot.navigation = Navigation::default();
        // A dead bot still picks what it will come back holding, exactly as a player does.
        return Input {
            seq,
            weapon: combat::preferred_weapon(&bot.profile, table, view.unlocked, None).unwrap_or(0),
            aim: view.me.pos,
            ..Input::default()
        };
    }

    perception::observe(
        &mut bot.awareness,
        collision,
        bot.profile.difficulty,
        view.me,
        others,
    );

    let target = bot
        .awareness
        .target
        .and_then(|id| others.iter().find(|other| other.id == id).copied());
    let distance = target.map(|contact| {
        let dx = contact.pos.x - view.me.pos.x;
        let dy = contact.pos.y - view.me.pos.y;
        (dx * dx + dy * dy).sqrt()
    });

    // What is this bot for, right now?
    let defender = objectives::is_defender(view.me.id, view.team_size);
    let objective = objectives::choose(rules, objectives, view.me.id, view.me.team, defender);
    bot.goal = objective.goal;

    // Where to walk. A fight takes priority over an errand only when there is nowhere to be.
    let destination = match (objective.destination, target, distance) {
        (Some(place), _, _) if objective.goal.is_travel() => Some(place),
        (_, Some(contact), Some(distance))
            if combat::wants_to_close(&bot.profile, view.holding, distance) =>
        {
            Some(contact.pos)
        }
        (_, Some(_), _) => None,
        _ => nearest_loot(view.me.pos, loot)
            .or_else(|| objectives::wander_target(spawns, rng.next_u64())),
    };

    if let Some(place) = destination {
        // Re-point only on arrival or when it had nowhere to be, so a moving target does not make
        // the bot restart its progress check every tick and never notice it is stuck.
        let needs_new_destination = bot.navigation.destination.is_none()
            || (bot.navigation.destination != Some(place)
                && bot.navigation.has_arrived(view.me.pos));
        if needs_new_destination {
            bot.navigation.go_to(place, view.me.pos);
        }
    }
    bot.navigation
        .check_progress(view.me.pos, rng.next_u64() % 2 == 0);
    let movement = navigation::steer(&bot.navigation, view.me.pos, view.fuel);

    // Where to shoot.
    let speed = table.get(view.holding).speed.max(0.001) * 60.0;
    let aim = match target {
        Some(contact) => perception::aim_point(
            bot.profile.difficulty,
            contact,
            speed,
            distance.unwrap_or(0.0),
            rng.next_unit(),
        ),
        None => combat::aim_at(&bot.awareness, None, forward_of(view.me)),
    };

    let fire = combat::should_fire(
        &bot.awareness,
        bot.profile.difficulty,
        view.ammo,
        view.reloading,
    );
    let reload = combat::should_reload(&bot.awareness, view.ammo, view.magazine, view.reloading);
    let throw_grenade = combat::should_throw_grenade(
        &bot.profile,
        &bot.awareness,
        view.grenades,
        distance.unwrap_or(f32::MAX),
        (rng.next_u64() % u64::from(u32::MAX)) as u32,
    );
    let weapon = combat::preferred_weapon(&bot.profile, table, view.unlocked, distance);
    let pickup = nearest_loot(view.me.pos, loot).is_some_and(|pos| {
        let dx = pos.x - view.me.pos.x;
        let dy = pos.y - view.me.pos.y;
        dx * dx + dy * dy <= 28.0 * 28.0
    });

    Input {
        seq,
        left: movement.left,
        right: movement.right,
        jump: movement.jump,
        jet: movement.jet,
        crouch: movement.crouch,
        prone: movement.prone,
        roll: movement.roll,
        fire,
        reload,
        throw_grenade,
        pickup,
        aim,
        weapon: weapon.unwrap_or(0),
        ..Input::default()
    }
}

fn nearest_loot(from: Vec2, loot: &[Vec2]) -> Option<Vec2> {
    loot.iter().copied().min_by(|left, right| {
        let d = |pos: Vec2| {
            let dx = pos.x - from.x;
            let dy = pos.y - from.y;
            dx * dx + dy * dy
        };
        d(*left).total_cmp(&d(*right))
    })
}

/// Somewhere in front of a bot to point at when it has no target, so it is not aiming at its feet.
fn forward_of(me: Contact) -> Vec2 {
    let facing = if me.velocity.x < 0.0 { -1.0 } else { 1.0 };
    Vec2 {
        x: me.pos.x + facing * 200.0,
        y: me.pos.y,
    }
}

/// How many bots a server should add to reach the wanted total, and how to split them.
///
/// A team mode fills both sides evenly; a free-for-all simply counts heads.
pub fn wanted_per_team(total: usize, team_mode: bool) -> (usize, usize) {
    if !team_mode {
        return (total, 0);
    }
    let alpha = total.div_ceil(2);
    (alpha, total - alpha)
}

/// What a bot says, if it says anything.
///
/// Deliberately sparse: a bot that comments on every kill is noise, and the cooldown is what keeps
/// a room full of them readable.
pub fn chat_line(bot: &mut Bot, event: BotChatEvent) -> Option<String> {
    if !bot.profile.chatty || bot.chat_cooldown > 0 {
        return None;
    }
    bot.chat_cooldown = CHAT_COOLDOWN_TICKS;
    Some(
        match event {
            BotChatEvent::Killed => "Got one.",
            BotChatEvent::Died => "I'm down.",
            BotChatEvent::TookObjective => "I have it, cover me.",
            BotChatEvent::Captured => "Scored!",
        }
        .to_string(),
    )
}

/// The things a bot has an opinion about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BotChatEvent {
    Killed,
    Died,
    TookObjective,
    Captured,
}
