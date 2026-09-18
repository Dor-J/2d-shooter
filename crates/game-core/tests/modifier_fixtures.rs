//! Parity fixtures for the three modifiers and the community-mode extension point.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:modifiers:realistic — weapon table, line of sight, visibility, chat, tuning
//!   rust:modifiers:survival  — readiness, no respawn, last one standing, spectating, chat
//!   rust:modifiers:advance   — unlocks, revocations, per-player state, reset
//!   rust:modifiers:scripted  — the bounded community ruleset and all eleven presets
//!
//! Source: OpenSoldat `shared/mechanics/Sprites.pas` (the Advance unlock loop), `shared/Cvar.pas`
//! (`sv_advancemode_amount`, `sv_survivalmode_*`), and the Soldat wiki's mode list, against the
//! revision pinned in `docs/parity/reference-lock.md`.

use game_core::{
    has_line_of_sight, modes, survival_standing, visibility_between, AdvanceConfig, CollisionWorld,
    ModeKind, ModeRules, ModifierSet, Outcome, RoundStanding, ScriptedError, ScriptedRules, SimRng,
    SurvivalConfig, Survivor, Unlocked, Vec2, Viewer, Visibility, WeaponKind, WeaponTable, World,
    ALPHA, BRAVO,
};

fn collision() -> CollisionWorld {
    World::new("deathmatch").collision().clone()
}

fn survivor(id: u32, team: u8, alive: bool) -> Survivor {
    Survivor {
        id,
        team,
        alive,
        spectator: false,
    }
}

// ----- Realistic --------------------------------------------------------------------------

#[test]
fn realistic_uses_its_own_weapon_table() {
    let normal = modes::modifiers::realistic::weapon_table(false);
    let realistic = modes::modifiers::realistic::weapon_table(true);

    assert!(!normal.is_realistic());
    assert!(realistic.is_realistic());
    assert_eq!(realistic, WeaponTable::realistic());
    assert_ne!(
        normal.canonical_hash(),
        realistic.canonical_hash(),
        "the two tables are genuinely different rulesets"
    );

    // The Realistic table is the one that carries self-bink, which is its signature.
    assert!(realistic.get(WeaponKind::Mp5).self_binks());
    assert!(!normal.get(WeaponKind::Mp5).self_binks());
}

#[test]
fn you_can_see_through_open_air_and_not_through_the_ground() {
    let collision = collision();
    // Two points either side of the arena floor, which sits across the bottom of the map.
    let above = Vec2 { x: 600.0, y: 600.0 };
    let below = Vec2 { x: 600.0, y: 690.0 };
    assert!(
        !has_line_of_sight(&collision, above, below),
        "the floor is in the way"
    );

    let near = Vec2 { x: 620.0, y: 600.0 };
    assert!(
        has_line_of_sight(&collision, above, near),
        "nothing between them"
    );
}

#[test]
fn nothing_is_visible_past_the_sight_range_however_clear_the_air() {
    let collision = collision();
    let here = Vec2 { x: 100.0, y: 200.0 };
    let just_inside = Vec2 {
        x: 100.0 + modes::modifiers::realistic::SIGHT_RANGE - 10.0,
        y: 200.0,
    };
    let too_far = Vec2 {
        x: 100.0 + modes::modifiers::realistic::SIGHT_RANGE + 10.0,
        y: 200.0,
    };

    assert!(has_line_of_sight(&collision, here, just_inside));
    assert!(!has_line_of_sight(&collision, here, too_far));
}

#[test]
fn realistic_hides_an_enemy_you_cannot_see_but_never_your_own_team() {
    let collision = collision();
    let me = Vec2 { x: 600.0, y: 600.0 };
    let behind_the_floor = Vec2 { x: 600.0, y: 690.0 };

    // An enemy behind cover is left out of the snapshot entirely.
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, true),
            Viewer::new(2, BRAVO, behind_the_floor, true),
        ),
        Visibility::Hidden
    );
    // A teammate is always sent, or the team cannot play together.
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, true),
            Viewer::new(3, ALPHA, behind_the_floor, true),
        ),
        Visibility::Full
    );
    // You are always sent yourself.
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, true),
            Viewer::new(1, ALPHA, me, true),
        ),
        Visibility::Full
    );
    // An enemy in the open is sent.
    let in_the_open = Vec2 { x: 640.0, y: 600.0 };
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, true),
            Viewer::new(2, BRAVO, in_the_open, true),
        ),
        Visibility::Full
    );
}

#[test]
fn a_dead_player_in_realistic_is_told_nothing_about_anybody_else() {
    let collision = collision();
    let me = Vec2 { x: 600.0, y: 600.0 };
    let right_there = Vec2 { x: 610.0, y: 600.0 };

    // Even a teammate standing next to the corpse is withheld, so a dead player cannot spot.
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, false),
            Viewer::new(3, ALPHA, right_there, true),
        ),
        Visibility::Hidden
    );
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, false),
            Viewer::new(2, BRAVO, right_there, true),
        ),
        Visibility::Hidden
    );
    // They still get themselves, so their own corpse renders.
    assert_eq!(
        visibility_between(
            &collision,
            Viewer::new(1, ALPHA, me, false),
            Viewer::new(1, ALPHA, me, true),
        ),
        Visibility::Full
    );
}

#[test]
fn team_chat_never_reaches_the_other_side() {
    use modes::modifiers::realistic::team_chat_reaches;
    assert!(team_chat_reaches(true, true, ALPHA, ALPHA));
    assert!(!team_chat_reaches(true, true, ALPHA, BRAVO));
    // An all-chat message reaches everybody, in every mode.
    assert!(team_chat_reaches(true, false, ALPHA, BRAVO));
    assert!(team_chat_reaches(false, false, ALPHA, BRAVO));
}

#[test]
fn realistic_hurts_more_to_fall_and_takes_longer_to_settle_the_aim() {
    use modes::modifiers::realistic::{fall_damage, hud_is_minimal, recoil_recovery};
    assert_eq!(fall_damage(false, 20.0), 20.0);
    assert!(fall_damage(true, 20.0) > 20.0);

    assert_eq!(recoil_recovery(false, 1.0), 1.0);
    assert!(
        recoil_recovery(true, 1.0) < 1.0,
        "the aim comes back slower"
    );

    assert!(hud_is_minimal(true));
    assert!(!hud_is_minimal(false));
}

#[test]
fn realistic_is_a_flag_on_the_rules_rather_than_a_mode_of_its_own() {
    let mut rules = ModeRules::new(ModeKind::CaptureTheFlag);
    assert!(!rules.modifiers.realistic);
    rules.modifiers.realistic = true;
    assert_eq!(rules.kind, ModeKind::CaptureTheFlag, "it is still CTF");
    assert!(rules.modifiers.realistic);
}

// ----- Survival ---------------------------------------------------------------------------

#[test]
fn a_survival_round_waits_until_enough_players_are_ready() {
    use modes::modifiers::survival::is_ready;
    let config = SurvivalConfig::default();
    assert_eq!(config.minimum_ready, 2);

    assert!(!is_ready(&config, 0));
    assert!(!is_ready(&config, 1));
    assert!(is_ready(&config, 2));
    assert!(is_ready(&config, 9));

    // A misconfigured zero still needs one player, because a round with nobody in it is not a
    // round.
    let broken = SurvivalConfig {
        minimum_ready: 0,
        ..config
    };
    assert!(!is_ready(&broken, 0));
    assert!(is_ready(&broken, 1));
}

#[test]
fn in_survival_a_dead_player_waits_for_the_next_round_and_watches_this_one() {
    use modes::modifiers::survival::{dead_players_spectate, may_respawn};
    assert!(may_respawn(false), "normally you come back");
    assert!(!may_respawn(true), "in survival you do not");
    assert!(dead_players_spectate(true));
    assert!(!dead_players_spectate(false));

    // The lifecycle agrees: a survival respawn has no delay because there is no respawn.
    let mut rules = ModeRules::new(ModeKind::Deathmatch);
    rules.modifiers = ModifierSet {
        survival: true,
        ..ModifierSet::default()
    };
    assert_eq!(modes::spawn::respawn_delay(&rules, 120), None);
}

#[test]
fn a_free_for_all_survival_round_ends_with_one_player_standing() {
    let rules = ModeRules::new(ModeKind::Deathmatch);

    let ongoing = [survivor(1, 0, true), survivor(2, 0, true)];
    assert_eq!(survival_standing(&rules, &ongoing), RoundStanding::Ongoing);
    assert!(!survival_standing(&rules, &ongoing).is_over());

    let decided = [survivor(1, 0, true), survivor(2, 0, false)];
    assert_eq!(
        survival_standing(&rules, &decided),
        RoundStanding::LastPlayer(1)
    );
    assert_eq!(
        survival_standing(&rules, &decided).outcome(),
        Some(Outcome::Player(1))
    );

    // Trading a simultaneous kill wipes the board rather than crowning a corpse.
    let wiped = [survivor(1, 0, false), survivor(2, 0, false)];
    assert_eq!(survival_standing(&rules, &wiped), RoundStanding::Wiped);
    assert_eq!(
        survival_standing(&rules, &wiped).outcome(),
        Some(Outcome::Draw)
    );
}

#[test]
fn one_player_alone_in_a_survival_round_has_not_won_anything_yet() {
    let rules = ModeRules::new(ModeKind::Deathmatch);
    let alone = [survivor(1, 0, true)];
    assert_eq!(
        survival_standing(&rules, &alone),
        RoundStanding::Ongoing,
        "you cannot be the last one standing when you were the only one"
    );
}

#[test]
fn a_team_survival_round_ends_when_one_side_is_wiped_out() {
    let rules = ModeRules::new(ModeKind::Teammatch);

    let ongoing = [
        survivor(1, ALPHA, true),
        survivor(2, BRAVO, true),
        survivor(3, ALPHA, false),
    ];
    assert_eq!(survival_standing(&rules, &ongoing), RoundStanding::Ongoing);

    let decided = [
        survivor(1, ALPHA, true),
        survivor(2, BRAVO, false),
        survivor(3, ALPHA, false),
    ];
    assert_eq!(
        survival_standing(&rules, &decided),
        RoundStanding::LastTeam(ALPHA)
    );
    assert_eq!(
        survival_standing(&rules, &decided).outcome(),
        Some(Outcome::Team(ALPHA))
    );
}

#[test]
fn spectators_never_hold_up_or_decide_a_survival_round() {
    let rules = ModeRules::new(ModeKind::Deathmatch);
    let watching = Survivor {
        id: 9,
        team: 0,
        alive: false,
        spectator: true,
    };

    // A spectator sitting there dead does not make the last living player a winner prematurely,
    // nor does it stop the round ending.
    let two_playing = [survivor(1, 0, true), survivor(2, 0, true), watching];
    assert_eq!(
        survival_standing(&rules, &two_playing),
        RoundStanding::Ongoing
    );

    let decided = [survivor(1, 0, true), survivor(2, 0, false), watching];
    assert_eq!(
        survival_standing(&rules, &decided),
        RoundStanding::LastPlayer(1)
    );
}

#[test]
fn the_flags_freeze_once_a_survival_round_is_decided() {
    use modes::modifiers::survival::flags_are_live;
    assert!(flags_are_live(true, RoundStanding::Ongoing));
    assert!(!flags_are_live(true, RoundStanding::LastTeam(ALPHA)));
    assert!(!flags_are_live(true, RoundStanding::Wiped));
    // Outside survival the flags are always live.
    assert!(flags_are_live(false, RoundStanding::LastTeam(ALPHA)));
}

#[test]
fn anti_spy_chat_stops_a_corpse_talking_to_the_living() {
    use modes::modifiers::survival::{chat_reaches_the_living, keeps_weapons};
    let off = SurvivalConfig::default();
    let on = SurvivalConfig {
        anti_spy_chat: true,
        ..off
    };

    assert!(
        chat_reaches_the_living(&on, true, true),
        "the living may talk"
    );
    assert!(
        !chat_reaches_the_living(&on, true, false),
        "the dead may not, with anti-spy on"
    );
    assert!(
        chat_reaches_the_living(&off, true, false),
        "and may, with it off"
    );
    assert!(
        chat_reaches_the_living(&on, false, false),
        "outside survival"
    );

    assert!(keeps_weapons(&off));
    assert!(!keeps_weapons(&SurvivalConfig {
        clear_weapons: true,
        ..off
    }));
}

// ----- Advance ----------------------------------------------------------------------------

#[test]
fn an_advance_player_starts_with_no_primary_and_keeps_their_secondary() {
    let start = Unlocked::starting();
    assert_eq!(start.count(), 0, "nothing earned yet");

    for primary in [WeaponKind::Ak74, WeaponKind::Barrett, WeaponKind::Spas12] {
        assert!(!start.has(primary), "{primary:?} is locked");
    }
    // A secondary is never locked, so a player always has something to shoot with.
    for secondary in [
        WeaponKind::Ussocom,
        WeaponKind::CombatKnife,
        WeaponKind::Chainsaw,
        WeaponKind::Law,
    ] {
        assert!(start.has(secondary), "{secondary:?} is always available");
    }
    assert_eq!(start.available().count(), 4, "the four secondaries");
}

#[test]
fn kills_unlock_primaries_one_at_a_time_and_deaths_take_them_back() {
    let config = AdvanceConfig::default();
    assert_eq!(config.kills_per_unlock, 2);

    let mut rng = SimRng::seeded(11);
    let mut unlocked = Unlocked::starting();

    let first = modes::modifiers::advance::unlock_one(&mut unlocked, &mut rng)
        .expect("something was unlocked");
    assert!(unlocked.has(first));
    assert_eq!(unlocked.count(), 1);

    let second = modes::modifiers::advance::unlock_one(&mut unlocked, &mut rng).unwrap();
    assert_ne!(first, second, "it never unlocks the same weapon twice");
    assert_eq!(unlocked.count(), 2);

    let lost = modes::modifiers::advance::revoke_one(&mut unlocked, &mut rng).unwrap();
    assert!(!unlocked.has(lost));
    assert_eq!(unlocked.count(), 1);
}

#[test]
fn unlocking_stops_once_a_player_has_every_primary() {
    let mut rng = SimRng::seeded(3);
    let mut unlocked = Unlocked::starting();

    for _ in 0..modes::modifiers::advance::PRIMARY_COUNT {
        assert!(modes::modifiers::advance::unlock_one(&mut unlocked, &mut rng).is_some());
    }
    assert_eq!(
        unlocked.count() as usize,
        modes::modifiers::advance::PRIMARY_COUNT
    );
    assert_eq!(
        modes::modifiers::advance::unlock_one(&mut unlocked, &mut rng),
        None,
        "there is nothing left to earn"
    );

    // And revoking stops once there is nothing left to take.
    let mut stripped = Unlocked::starting();
    assert_eq!(
        modes::modifiers::advance::revoke_one(&mut stripped, &mut rng),
        None
    );
}

#[test]
fn unlocks_are_deterministic_so_a_replay_agrees_with_the_match() {
    let run = || {
        let mut rng = SimRng::seeded(42);
        let mut unlocked = Unlocked::starting();
        let mut order = Vec::new();
        for _ in 0..5 {
            order.push(modes::modifiers::advance::unlock_one(
                &mut unlocked,
                &mut rng,
            ));
        }
        order
    };
    assert_eq!(run(), run());
}

#[test]
fn the_unlock_arrives_on_the_right_kill_and_the_loss_on_the_right_death() {
    use modes::modifiers::advance::{death_costs_unlock, kill_earns_unlock};
    let config = AdvanceConfig::default();

    assert!(!kill_earns_unlock(&config, 0), "no kills, no reward");
    assert!(!kill_earns_unlock(&config, 1));
    assert!(kill_earns_unlock(&config, 2));
    assert!(!kill_earns_unlock(&config, 3));
    assert!(kill_earns_unlock(&config, 4));

    assert!(!death_costs_unlock(&config, 1));
    assert!(death_costs_unlock(&config, 2));

    // A longer ladder is configurable.
    let slow = AdvanceConfig {
        kills_per_unlock: 5,
    };
    assert!(!kill_earns_unlock(&slow, 4));
    assert!(kill_earns_unlock(&slow, 5));
}

#[test]
fn a_player_whose_weapon_was_taken_away_still_spawns_holding_something() {
    use modes::modifiers::advance::resolve_choice;
    let mut unlocked = Unlocked::starting();
    unlocked.grant(WeaponKind::Ak74);

    // They may pick what they have earned.
    assert_eq!(
        resolve_choice(true, unlocked, WeaponKind::Ak74),
        WeaponKind::Ak74
    );
    // Picking something locked falls back rather than refusing.
    let fallback = resolve_choice(true, unlocked, WeaponKind::Barrett);
    assert_ne!(fallback, WeaponKind::Barrett);
    assert!(
        unlocked.has(fallback),
        "and the fallback is something they have"
    );

    // Outside Advance every choice stands.
    assert_eq!(
        resolve_choice(false, Unlocked::starting(), WeaponKind::Barrett),
        WeaponKind::Barrett
    );
}

#[test]
fn progress_is_per_player_and_is_wiped_between_matches() {
    let mut rng = SimRng::seeded(5);
    let mut one = Unlocked::starting();
    let two = Unlocked::starting();

    modes::modifiers::advance::unlock_one(&mut one, &mut rng);
    assert_eq!(one.count(), 1);
    assert_eq!(two.count(), 0, "one player's progress is not another's");

    assert_eq!(modes::modifiers::advance::reset().count(), 0);
    assert_eq!(
        Unlocked::everything().count() as usize,
        modes::modifiers::advance::PRIMARY_COUNT
    );
}

// ----- The community extension point ------------------------------------------------------

#[test]
fn a_plain_community_ruleset_changes_nothing_about_its_base_mode() {
    let plain = ScriptedRules::plain("Plain", ModeKind::Deathmatch);
    assert!(plain.validate().is_ok());
    assert!(plain.allows_weapon(WeaponKind::Barrett), "no restriction");
    assert_eq!(plain.spawn_health(), 100);
    assert!(plain.damage_enabled);
    assert_eq!(plain.to_mode_rules().kind, ModeKind::Deathmatch);
}

#[test]
fn a_community_ruleset_is_checked_before_it_is_used() {
    let base = ScriptedRules::plain("Base", ModeKind::Deathmatch);

    assert_eq!(
        ScriptedRules {
            name: "   ".into(),
            ..base.clone()
        }
        .validate(),
        Err(ScriptedError::EmptyName)
    );

    let long = ScriptedRules {
        name: "x".repeat(200),
        ..base.clone()
    };
    assert!(matches!(
        long.validate(),
        Err(ScriptedError::NameTooLong { .. })
    ));

    let greedy = ScriptedRules {
        allowed_weapons: vec![WeaponKind::Ak74; 40],
        ..base.clone()
    };
    assert!(matches!(
        greedy.validate(),
        Err(ScriptedError::TooManyWeapons { .. })
    ));

    let unplayable = ScriptedRules {
        allowed_weapons: vec![WeaponKind::StationaryGun],
        ..base.clone()
    };
    assert_eq!(unplayable.validate(), Err(ScriptedError::NoUsableWeapon));

    for percent in [0u8, 101, 255] {
        let bad = ScriptedRules {
            spawn_health_percent: percent,
            ..base.clone()
        };
        assert!(matches!(
            bad.validate(),
            Err(ScriptedError::HealthOutOfRange { .. })
        ));
    }
}

#[test]
fn a_ruleset_name_is_truncated_rather_than_allowed_to_grow_without_bound() {
    let long = ScriptedRules::plain(&"y".repeat(500), ModeKind::Deathmatch);
    assert!(long.name.len() <= modes::scripted::MAX_NAME_BYTES);
    assert!(long.validate().is_ok());
}

#[test]
fn every_community_mode_the_wiki_lists_is_a_valid_ruleset() {
    let all = modes::scripted::presets::all();
    assert_eq!(all.len(), 11, "the eleven modes the wiki names");

    for ruleset in &all {
        assert!(
            ruleset.validate().is_ok(),
            "{} does not validate: {:?}",
            ruleset.name,
            ruleset.validate()
        );
        assert!(!ruleset.name.trim().is_empty());
        // Each one is built on an official mode rather than inventing its own lifecycle.
        assert_eq!(ruleset.to_mode_rules().kind, ruleset.base);
    }

    // They are all distinct: a preset list with a duplicate would be a copy-paste mistake.
    let mut names: Vec<&str> = all.iter().map(|ruleset| ruleset.name.as_str()).collect();
    names.sort_unstable();
    let count = names.len();
    names.dedup();
    assert_eq!(names.len(), count, "every preset has its own name");
}

#[test]
fn each_community_mode_says_what_makes_it_different() {
    use modes::scripted::presets;

    // Climb is a race nobody can interfere with.
    let climb = presets::climb();
    assert!(!climb.damage_enabled);
    assert!(climb.race_to_finish);

    // Knife Only and Dodgeball restrict the armoury.
    for restricted in [presets::knife_only(), presets::dodgeball()] {
        assert!(restricted.allows_weapon(WeaponKind::CombatKnife));
        assert!(
            !restricted.allows_weapon(WeaponKind::Barrett),
            "{}",
            restricted.name
        );
    }

    // OneShots makes everybody fragile.
    assert_eq!(presets::one_shots().spawn_health(), 1);

    // Zombie turns the dead into the horde.
    assert!(presets::zombie().converts_on_death);

    // Hide and Seek is a hunt.
    assert!(presets::hide_and_seek().hunt);

    // RS/CS is Realistic and Survival together, which is the whole point of the modifiers being
    // orthogonal.
    let rscs = presets::realistic_counter_strike();
    assert!(rscs.modifiers.realistic);
    assert!(rscs.modifiers.survival);
    assert_eq!(rscs.base, ModeKind::Teammatch);

    // Tactical Trench Wars is Trench Wars with Realistic on top.
    assert_eq!(
        presets::trench_wars().base,
        presets::tactical_trench_wars().base
    );
    assert!(!presets::trench_wars().modifiers.realistic);
    assert!(presets::tactical_trench_wars().modifiers.realistic);

    // Domination is scored on the hold timer rather than on captures.
    assert_eq!(presets::domination().base, ModeKind::HoldTheFlag);
}

#[test]
fn a_community_mode_can_be_found_by_name_and_an_unknown_one_is_simply_not_found() {
    use modes::scripted::presets::by_id;
    assert_eq!(by_id("Zombie").map(|r| r.base), Some(ModeKind::Teammatch));
    assert_eq!(by_id("zombie").map(|r| r.name), Some("Zombie".into()));
    assert!(by_id("Not A Mode").is_none());
    assert!(by_id("").is_none());
}

#[test]
fn the_three_modifiers_are_independent_of_one_another() {
    let all_on = ModifierSet {
        realistic: true,
        survival: true,
        advance: true,
    };
    assert!(all_on.respawn_is_deferred(), "survival defers the respawn");

    // Turning one off leaves the others alone.
    let no_survival = ModifierSet {
        survival: false,
        ..all_on
    };
    assert!(!no_survival.respawn_is_deferred());
    assert!(no_survival.realistic && no_survival.advance);

    // And they combine with any mode.
    for kind in [
        ModeKind::Deathmatch,
        ModeKind::Teammatch,
        ModeKind::CaptureTheFlag,
        ModeKind::HoldTheFlag,
        ModeKind::Infiltration,
        ModeKind::Pointmatch,
        ModeKind::Rambomatch,
    ] {
        let mut rules = ModeRules::new(kind);
        rules.modifiers = all_on;
        assert_eq!(rules.kind, kind, "the modifiers did not change the mode");
        assert!(rules.modifiers.realistic);
    }
}
