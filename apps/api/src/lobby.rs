//! Room listings the browser can sort, filter, and search. Ping is measured by the client;
//! this only carries the columns.

use protocol::RoomInfo;

#[derive(Clone, Debug, Default)]
pub struct LobbyQuery {
    pub search: String,
    pub hide_full: bool,
    pub hide_empty: bool,
    pub mode: Option<String>,
}

pub fn filter_rooms(rooms: &[RoomInfo], query: &LobbyQuery) -> Vec<RoomInfo> {
    let needle = query.search.trim().to_ascii_lowercase();
    rooms
        .iter()
        .filter(|room| {
            if query.hide_full && room.players >= room.capacity {
                return false;
            }
            if query.hide_empty && room.players == 0 {
                return false;
            }
            if let Some(mode) = &query.mode {
                if !room.mode.eq_ignore_ascii_case(mode) {
                    return false;
                }
            }
            if needle.is_empty() {
                return true;
            }
            room.name.to_ascii_lowercase().contains(&needle)
                || room.map.to_ascii_lowercase().contains(&needle)
                || room.mode.to_ascii_lowercase().contains(&needle)
        })
        .cloned()
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SortKey {
    Name,
    Players,
    Map,
    Mode,
}

pub fn sort_rooms(mut rooms: Vec<RoomInfo>, key: SortKey) -> Vec<RoomInfo> {
    rooms.sort_by(|a, b| match key {
        SortKey::Name => a.name.cmp(&b.name),
        SortKey::Players => b.players.cmp(&a.players).then(a.name.cmp(&b.name)),
        SortKey::Map => a.map.cmp(&b.map).then(a.name.cmp(&b.name)),
        SortKey::Mode => a.mode.cmp(&b.mode).then(a.name.cmp(&b.name)),
    });
    rooms
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::WireModifiers;

    fn room(name: &str, players: usize, capacity: usize, mode: &str) -> RoomInfo {
        RoomInfo {
            id: 1,
            name: name.into(),
            mode: mode.into(),
            players,
            capacity,
            map: "Aero".into(),
            weapon_mod: String::new(),
            weapon_hash: 0,
            modifiers: WireModifiers::default(),
            ruleset: None,
            password: false,
            version: protocol::VERSION,
            required_mod: None,
            region: None,
        }
    }

    #[test]
    fn search_and_full_filter_hide_what_they_should() {
        let rooms = vec![
            room("Night", 16, 16, "deathmatch"),
            room("Friends", 0, 16, "ctf"),
            room("Arena", 4, 16, "deathmatch"),
        ];
        let found = filter_rooms(
            &rooms,
            &LobbyQuery {
                search: "ar".into(),
                hide_full: true,
                hide_empty: true,
                mode: None,
            },
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "Arena");
    }

    #[test]
    fn sorting_by_players_puts_the_busiest_room_first() {
        let rooms = vec![
            room("A", 1, 16, "deathmatch"),
            room("B", 8, 16, "deathmatch"),
        ];
        let sorted = sort_rooms(rooms, SortKey::Players);
        assert_eq!(sorted[0].name, "B");
    }
}
