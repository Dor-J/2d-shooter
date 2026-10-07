use game_core::{
    Event, Input, MatchEvent, ModeKind, Replay, ReplayHeader, Rotation, ValidatedMap, World,
};
use map_editor::{original_default_map, DEFAULT_MAPS};
use protocol::{WireModifiers, VERSION};
use std::collections::BTreeMap;

pub(crate) struct Room {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) mode: String,
    pub(crate) code: String,
    pub(crate) public: bool,
    pub(crate) world: World,
    pub(crate) inputs: BTreeMap<u32, Input>,
    pub(crate) map: String,
    /// The simulation clears its events every tick but snapshots go out less often, so they are
    /// gathered here and drained into the snapshot. Otherwise kills and hits would be dropped.
    pub(crate) pending_events: Vec<Event>,
    /// Lifecycle decisions gathered the same way, so a round ending between snapshots is never
    /// lost to the client.
    pub(crate) pending_match_events: Vec<MatchEvent>,
    /// The maps this room plays, in order.
    pub(crate) rotation: Rotation,
    /// The community ruleset this room is playing, when it is playing one.
    pub(crate) ruleset: Option<String>,
    pub(crate) password: String,
    pub(crate) paused: bool,
    pub(crate) capacity: usize,
    pub(crate) required_mod: String,
    pub(crate) replay: Replay,
    pub(crate) votes: std::collections::HashMap<u32, String>,
}

/// What a client asked for when it opened a room.
///
/// A struct rather than a long argument list: the fields are all optional-looking strings and
/// flags, and getting two of them the wrong way round would be silent.
#[derive(Clone, Debug, Default)]
pub(crate) struct RoomRequest {
    pub(crate) name: String,
    pub(crate) mode: String,
    pub(crate) public: bool,
    pub(crate) requested_map: Option<String>,
    pub(crate) weapon_mod: Option<String>,
    pub(crate) modifiers: Option<WireModifiers>,
    pub(crate) ruleset: Option<String>,
    /// Whether bonus kits are handed out. `None` takes the server default, which is on.
    pub(crate) bonuses: Option<bool>,
    /// How many bots to fill the room with, and how hard they should be.
    pub(crate) bots: Option<u8>,
    pub(crate) bot_difficulty: Option<String>,
    pub(crate) password: String,
    pub(crate) required_mod: String,
}

pub(crate) const ROOM_CAPACITY: usize = 16;

/// Builds the snapshot one recipient is allowed to see.
///
/// Outside Realistic every player sees the same world, so the shared snapshot is handed straight
/// back. In Realistic an enemy the recipient cannot actually see is left out of their copy
/// entirely, because a client that never receives a position cannot reveal it.
pub(crate) fn snapshot_for(world: &World, recipient: u32) -> World {
    if !world.rules.modifiers.realistic {
        return world.clone();
    }
    let Some(observer) = world.players.get(&recipient) else {
        return world.clone();
    };
    let observer =
        game_core::Viewer::new(observer.id, observer.team, observer.pos, observer.hp > 0);

    let mut filtered = world.clone();
    filtered.players.retain(|_, player| {
        game_core::visibility_between(
            world.collision(),
            observer,
            game_core::Viewer::new(player.id, player.team, player.pos, player.hp > 0),
        )
        .is_visible()
    });
    filtered
}

/// The most bots one room may hold, so a command cannot fill a server with them.
pub(crate) const MAX_BOTS: usize = 12;

/// Ids a server hands to bots, high enough never to collide with a guest's.
const BOT_ID_BASE: u32 = 900_000;

/// Fills a world with bots up to `wanted`, or removes them down to it.
///
/// Bots join through the same path a human does, so they are assigned to teams, spawned, and given
/// a loadout by exactly the same rules.
pub(crate) fn fill_with_bots(
    world: &mut World,
    wanted: usize,
    difficulty: game_core::Difficulty,
) -> usize {
    let profiles = game_core::bots::profile::default_profiles();
    let existing = world.bots.len();
    if wanted <= existing {
        return world.trim_bots(wanted);
    }
    for index in existing..wanted {
        let stock = &profiles[index % profiles.len()];
        // Past the stock list the names repeat, so they are numbered to stay distinct.
        let name = if index < profiles.len() {
            stock.name.clone()
        } else {
            format!("{} {}", stock.name, index / profiles.len() + 1)
        };
        let profile = game_core::BotProfile {
            name: game_core::bots::profile::clean_name(&name),
            difficulty,
            ..stock.clone()
        };
        world.add_bot(
            BOT_ID_BASE + index as u32,
            profile,
            game_core::TeamChoice::Auto,
        );
    }
    wanted - existing
}

/// Whether a map built for `map_mode` can host a room playing `mode`.
///
/// Teammatch happily runs on a deathmatch map; a flag mode cannot, because it needs the bases.
pub(crate) fn map_suits_mode(map_mode: content::manifest::MapMode, mode: ModeKind) -> bool {
    use content::manifest::MapMode;
    match mode {
        ModeKind::Deathmatch | ModeKind::Teammatch | ModeKind::Rambomatch => {
            matches!(map_mode, MapMode::Deathmatch | MapMode::TeamMatch)
        }
        ModeKind::Pointmatch => matches!(map_mode, MapMode::Deathmatch | MapMode::PointMatch),
        ModeKind::CaptureTheFlag => matches!(map_mode, MapMode::CaptureTheFlag),
        ModeKind::HoldTheFlag => matches!(map_mode, MapMode::HoldTheFlag),
        ModeKind::Infiltration => matches!(map_mode, MapMode::Infiltration),
    }
}

/// Every map the server ships that suits `mode`, looping, with `first` played first.
///
/// A rotation is never empty: a server with nothing to load would strand every room on it.
pub(crate) fn default_rotation(first: &str, mode: ModeKind) -> Rotation {
    let names: Vec<String> = DEFAULT_MAPS
        .iter()
        .filter(|entry| map_suits_mode(entry.mode, mode))
        .map(|entry| entry.name.to_string())
        .collect();
    let mut rotation = Rotation::new(names, true).unwrap_or_else(|| {
        Rotation::new(vec![first.to_string()], true).expect("one map is always a valid rotation")
    });
    rotation.select(first);
    rotation
}

/// Loads the next map in a room's rotation and restarts the match on it.
///
/// Players stay where they are in the room; the world, the score, and the clock are rebuilt, which
/// is what a map change is. A map that will not load leaves the room on the one it is already
/// playing rather than dropping everybody.
pub(crate) fn advance_map(room: &mut Room) {
    let next = room.rotation.next_map().to_string();
    let Some(entry) = DEFAULT_MAPS.iter().find(|entry| entry.name == next) else {
        room.world.restart_match();
        return;
    };
    let Ok(map) = ValidatedMap::try_from(original_default_map(*entry)) else {
        room.world.restart_match();
        return;
    };

    let carried: Vec<(u32, String)> = room
        .world
        .players
        .iter()
        .map(|(id, player)| (*id, player.name.clone()))
        .collect();
    let rules = room.world.rules;
    let weapons = room.world.weapons.clone();

    let mut world = World::with_map(&room.mode, &map);
    world.rules = rules;
    world.weapons = weapons;
    world.place_objectives();
    for (id, name) in carried {
        world.add_player(id, name);
    }
    world.restart_match();
    room.map = next;
    room.replay = Replay::new(ReplayHeader::new(
        VERSION,
        &room.map,
        &world,
        u64::from(room.id),
    ));
    room.world = world;
    room.inputs.clear();
}
