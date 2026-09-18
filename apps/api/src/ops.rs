//! Liveness versus readiness, and the extra counters /metrics already started.

pub fn liveness() -> serde_json::Value {
    serde_json::json!({"status": "ok"})
}

pub fn readiness(drain: bool, rooms: usize, players: usize) -> (u16, serde_json::Value) {
    if drain {
        (
            503,
            serde_json::json!({"status":"drain","drain":true,"rooms":rooms,"players":players}),
        )
    } else {
        (
            200,
            serde_json::json!({"status":"ready","drain":false,"rooms":rooms,"players":players}),
        )
    }
}

pub fn metrics_text(
    guests: usize,
    rooms: usize,
    players: usize,
    audit: usize,
    bans: usize,
    tick_ms: u64,
    errors: u64,
) -> String {
    format!(
        "game_guests {guests}\ngame_rooms {rooms}\ngame_players {players}\ngame_audit {audit}\ngame_bans {bans}\ngame_tick_ms {tick_ms}\ngame_errors {errors}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Acceptance evidence: rust:ops:ready
    fn liveness_stays_up_while_readiness_drops_during_drain() {
        assert_eq!(liveness()["status"], "ok");
        let (code, body) = readiness(true, 1, 4);
        assert_eq!(code, 503);
        assert_eq!(body["status"], "drain");
        assert_eq!(readiness(false, 1, 0).0, 200);
    }

    #[test]
    fn metrics_name_tick_and_error_counters() {
        let text = metrics_text(2, 1, 4, 3, 1, 2, 0);
        assert!(text.contains("game_tick_ms 2"));
        assert!(text.contains("game_players 4"));
    }
}
