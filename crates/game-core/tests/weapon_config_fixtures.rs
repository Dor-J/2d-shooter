//! Field-by-field parity fixtures for the normal and Realistic weapon tables.
//!
//! The expected values live in `tests/fixtures/reference/weapons.json`, which records the pinned
//! upstream revision, the two configuration paths the numbers come from, and the source symbols
//! that give them meaning. Nothing here copies upstream implementation code.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:weapon_config_fixtures — sixteen fields, both tables, clusters, hash, loader bounds

use game_core::{
    BulletStyle, NoCollision, WeaponConfigError, WeaponKind, WeaponLimits, WeaponTable,
};
use serde_json::Value;

const TABLES_COMMIT: &str = "5b6e5bef23f5c0d58fb1d4d887b9b94ebcf799b4";
const SEMANTICS_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

fn fixture() -> Value {
    let raw = include_str!("../../../tests/fixtures/reference/weapons.json");
    serde_json::from_str(raw).expect("weapon reference fixture parses")
}

fn expected(table: &str, weapon: &str, field: &str) -> f64 {
    fixture()["tables"][table]["weapons"][weapon][field]
        .as_f64()
        .unwrap_or_else(|| panic!("fixture is missing {table}/{weapon}/{field}"))
}

fn expected_f32(table: &str, weapon: &str, field: &str) -> f32 {
    expected(table, weapon, field) as f32
}

#[test]
fn fixture_metadata_is_pinned_to_the_reference_lock() {
    let fixture = fixture();
    assert_eq!(fixture["source"]["tablesCommit"], TABLES_COMMIT);
    assert_eq!(fixture["source"]["semanticsCommit"], SEMANTICS_COMMIT);
    assert_eq!(
        fixture["fields"].as_array().map(Vec::len),
        Some(16),
        "gap 4 lists sixteen configurable fields per weapon"
    );
    assert_eq!(
        fixture["hashOrder"].as_array().map(Vec::len),
        Some(20),
        "the canonical hash covers the twenty original weapons"
    );
}

#[test]
fn every_configured_weapon_matches_every_field_of_the_normal_table() {
    let table = WeaponTable::normal();
    assert_eq!(table.name(), "Default mod");
    assert_eq!(table.version(), "1.7.1");
    assert!(!table.is_realistic());

    for kind in WeaponKind::configured() {
        let def = table.get(kind);
        let name = kind.ini_name();
        assert_eq!(
            def.damage,
            expected_f32("normal", name, "Damage"),
            "{name} damage"
        );
        assert_eq!(
            f64::from(def.fire_interval),
            expected("normal", name, "FireInterval"),
            "{name} fire interval"
        );
        assert_eq!(
            f64::from(def.ammo),
            expected("normal", name, "Ammo"),
            "{name} ammo"
        );
        assert_eq!(
            f64::from(def.reload_time),
            expected("normal", name, "ReloadTime"),
            "{name} reload time"
        );
        assert_eq!(
            def.speed,
            expected_f32("normal", name, "Speed"),
            "{name} speed"
        );
        assert_eq!(
            f64::from(def.bullet_style.code()),
            expected("normal", name, "BulletStyle"),
            "{name} bullet style"
        );
        assert_eq!(
            f64::from(def.start_up_time),
            expected("normal", name, "StartUpTime"),
            "{name} start-up time"
        );
        assert_eq!(
            f64::from(def.bink),
            expected("normal", name, "Bink"),
            "{name} bink"
        );
        assert_eq!(
            def.movement_acc,
            expected_f32("normal", name, "MovementAcc"),
            "{name} movement accuracy"
        );
        assert_eq!(
            def.bullet_spread,
            expected_f32("normal", name, "BulletSpread"),
            "{name} bullet spread"
        );
        assert_eq!(
            f64::from(def.recoil),
            expected("normal", name, "Recoil"),
            "{name} recoil"
        );
        assert_eq!(
            def.push,
            expected_f32("normal", name, "Push"),
            "{name} push"
        );
        assert_eq!(
            def.inherited_velocity,
            expected_f32("normal", name, "InheritedVelocity"),
            "{name} inherited velocity"
        );
        assert_eq!(
            def.modifier_head,
            expected_f32("normal", name, "ModifierHead"),
            "{name} head modifier"
        );
        assert_eq!(
            def.modifier_chest,
            expected_f32("normal", name, "ModifierChest"),
            "{name} chest modifier"
        );
        assert_eq!(
            def.modifier_legs,
            expected_f32("normal", name, "ModifierLegs"),
            "{name} leg modifier"
        );
    }
}

#[test]
fn every_configured_weapon_matches_every_field_of_the_realistic_table() {
    let table = WeaponTable::realistic();
    assert_eq!(table.name(), "Realistic mod");
    assert!(table.is_realistic());

    for kind in WeaponKind::configured() {
        let def = table.get(kind);
        let name = kind.ini_name();
        assert_eq!(
            def.damage,
            expected_f32("realistic", name, "Damage"),
            "{name} damage"
        );
        assert_eq!(
            f64::from(def.fire_interval),
            expected("realistic", name, "FireInterval"),
            "{name} fire interval"
        );
        assert_eq!(
            f64::from(def.ammo),
            expected("realistic", name, "Ammo"),
            "{name} ammo"
        );
        assert_eq!(
            f64::from(def.reload_time),
            expected("realistic", name, "ReloadTime"),
            "{name} reload time"
        );
        assert_eq!(
            def.speed,
            expected_f32("realistic", name, "Speed"),
            "{name} speed"
        );
        assert_eq!(
            f64::from(def.bullet_style.code()),
            expected("realistic", name, "BulletStyle"),
            "{name} bullet style"
        );
        assert_eq!(
            f64::from(def.start_up_time),
            expected("realistic", name, "StartUpTime"),
            "{name} start-up time"
        );
        assert_eq!(
            f64::from(def.bink),
            expected("realistic", name, "Bink"),
            "{name} bink"
        );
        assert_eq!(
            def.movement_acc,
            expected_f32("realistic", name, "MovementAcc"),
            "{name} movement accuracy"
        );
        assert_eq!(
            def.bullet_spread,
            expected_f32("realistic", name, "BulletSpread"),
            "{name} bullet spread"
        );
        assert_eq!(
            f64::from(def.recoil),
            expected("realistic", name, "Recoil"),
            "{name} recoil"
        );
        assert_eq!(
            def.push,
            expected_f32("realistic", name, "Push"),
            "{name} push"
        );
        assert_eq!(
            def.inherited_velocity,
            expected_f32("realistic", name, "InheritedVelocity"),
            "{name} inherited velocity"
        );
        assert_eq!(
            def.modifier_head,
            expected_f32("realistic", name, "ModifierHead"),
            "{name} head modifier"
        );
        assert_eq!(
            def.modifier_chest,
            expected_f32("realistic", name, "ModifierChest"),
            "{name} chest modifier"
        );
        assert_eq!(
            def.modifier_legs,
            expected_f32("realistic", name, "ModifierLegs"),
            "{name} leg modifier"
        );
    }
}

#[test]
fn the_realistic_table_carries_the_negative_bink_that_the_normal_table_does_not() {
    let normal = WeaponTable::normal();
    let realistic = WeaponTable::realistic();

    assert_eq!(normal.get(WeaponKind::Mp5).bink, 0);
    assert_eq!(realistic.get(WeaponKind::Mp5).bink, -10);
    assert!(realistic.get(WeaponKind::Mp5).self_binks());
    assert!(!realistic.get(WeaponKind::Mp5).binks_the_victim());

    // Barrett binks whoever it hits in both tables and never self-binks.
    assert_eq!(normal.get(WeaponKind::Barrett).bink, 65);
    assert!(normal.get(WeaponKind::Barrett).binks_the_victim());
    assert!(!normal.get(WeaponKind::Barrett).self_binks());
}

#[test]
fn cluster_and_thrown_knife_nest_under_their_parent_weapons() {
    for table in [WeaponTable::normal(), WeaponTable::realistic()] {
        let grenade = table.get(WeaponKind::FragGrenade);
        let cluster_grenade = table.get(WeaponKind::ClusterGrenade);
        let cluster = table.get(WeaponKind::Cluster);
        let knife = table.get(WeaponKind::CombatKnife);
        let thrown = table.get(WeaponKind::ThrownKnife);

        // Everything but the bullet style is inherited from the parent section.
        assert_eq!(cluster_grenade.damage, grenade.damage);
        assert_eq!(cluster_grenade.fire_interval, grenade.fire_interval);
        assert_eq!(cluster_grenade.ammo, grenade.ammo);
        assert_eq!(cluster_grenade.reload_time, grenade.reload_time);
        assert_eq!(cluster_grenade.speed, grenade.speed);
        assert_eq!(cluster_grenade.bink, grenade.bink);
        assert_eq!(cluster_grenade.push, grenade.push);
        assert_eq!(
            cluster_grenade.inherited_velocity,
            grenade.inherited_velocity
        );
        assert_eq!(cluster_grenade.bullet_style, BulletStyle::ClusterGrenade);

        assert_eq!(cluster.damage, cluster_grenade.damage);
        assert_eq!(cluster.bullet_style, BulletStyle::Cluster);

        assert_eq!(thrown.damage, knife.damage);
        assert_eq!(thrown.speed, knife.speed);
        assert_eq!(thrown.bullet_style, BulletStyle::ThrownKnife);

        // Derived weapons have no ini section of their own, so they cannot be configured.
        assert!(WeaponKind::ClusterGrenade.ini_name().is_empty());
        assert!(!WeaponKind::configured().any(|kind| kind == WeaponKind::Cluster));
    }
}

#[test]
fn derived_style_capabilities_follow_the_bullet_style() {
    let table = WeaponTable::normal();

    // Lifetimes from shared/Constants.pas: BULLET_TIMEOUT, GRENADE_TIMEOUT, FLAMER_TIMEOUT,
    // MELEE_TIMEOUT, M2BULLET_TIMEOUT, applied per bullet style in BuildWeapons.
    assert_eq!(table.get(WeaponKind::Ak74).timeout(), 420);
    assert_eq!(table.get(WeaponKind::FragGrenade).timeout(), 180);
    assert_eq!(table.get(WeaponKind::ClusterGrenade).timeout(), 180);
    assert_eq!(table.get(WeaponKind::Flamer).timeout(), 32);
    assert_eq!(table.get(WeaponKind::CombatKnife).timeout(), 1);
    assert_eq!(table.get(WeaponKind::Punch).timeout(), 1);
    assert_eq!(table.get(WeaponKind::StationaryGun).timeout(), 60);

    assert_eq!(table.get(WeaponKind::Spas12).pellets(), 6);
    assert_eq!(table.get(WeaponKind::DesertEagles).pellets(), 2);
    assert_eq!(table.get(WeaponKind::Ak74).pellets(), 1);

    assert!(table.get(WeaponKind::M79).explosion_radius() == 64.0);
    assert!(table.get(WeaponKind::FragGrenade).explosion_radius() == 85.0);
    assert!(table.get(WeaponKind::Cluster).explosion_radius() == 35.0);
    assert!(table.get(WeaponKind::Ak74).explosion_radius() == 0.0);

    assert!(table.get(WeaponKind::CombatKnife).bullet_style.is_melee());
    assert!(table.get(WeaponKind::Chainsaw).bullet_style.is_melee());
    assert!(table.get(WeaponKind::Law).bullet_style.is_explosive());
    assert!(!table.get(WeaponKind::Ak74).bullet_style.is_explosive());

    // Barrett, M79, knife, and LAW keep full power at range; everything else degrades.
    assert!(!table.get(WeaponKind::Barrett).degrades_over_distance());
    assert!(!table.get(WeaponKind::M79).degrades_over_distance());
    assert!(!table.get(WeaponKind::CombatKnife).degrades_over_distance());
    assert!(!table.get(WeaponKind::Law).degrades_over_distance());
    assert!(table.get(WeaponKind::Ak74).degrades_over_distance());

    // Clip reload and fire mode come from CreateWeaponsBase, not from the ini.
    assert!(table.get(WeaponKind::Ak74).clip_reload());
    assert!(!table.get(WeaponKind::Spas12).clip_reload());
    assert!(table.get(WeaponKind::DesertEagles).semi_automatic());
    assert!(!table.get(WeaponKind::Ak74).semi_automatic());
}

#[test]
fn canonical_hashing_matches_the_pinned_checksum_and_separates_the_two_tables() {
    let fixture = fixture();
    let normal = fixture["tables"]["normal"]["checksum"].as_u64().unwrap();
    let realistic = fixture["tables"]["realistic"]["checksum"].as_u64().unwrap();

    assert_eq!(u64::from(WeaponTable::normal().canonical_hash()), normal);
    assert_eq!(
        u64::from(WeaponTable::realistic().canonical_hash()),
        realistic
    );
    assert_ne!(normal, realistic);
}

#[test]
fn canonical_hashing_changes_when_any_hashed_field_changes() {
    let baseline = WeaponTable::normal().canonical_hash();
    let source = format!(
        "[Info]\nName=Tweaked\nVersion=1\n{}",
        WeaponTable::normal().to_ini_body()
    );

    let unchanged = WeaponTable::parse_ini(&source, &WeaponLimits::default()).unwrap();
    assert_eq!(unchanged.canonical_hash(), baseline);

    let nudged = source.replace("Damage=1.81", "Damage=1.82");
    assert_ne!(nudged, source);
    let nudged = WeaponTable::parse_ini(&nudged, &WeaponLimits::default()).unwrap();
    assert_ne!(nudged.canonical_hash(), baseline);

    // The hitbox modifiers are deliberately outside the checksum, exactly as upstream.
    let modifier = source.replace("ModifierHead=1.1", "ModifierHead=1.2");
    assert_ne!(modifier, source);
    let modifier = WeaponTable::parse_ini(&modifier, &WeaponLimits::default()).unwrap();
    assert_eq!(modifier.canonical_hash(), baseline);
    assert_ne!(
        modifier.get(WeaponKind::DesertEagles).modifier_head,
        WeaponTable::normal()
            .get(WeaponKind::DesertEagles)
            .modifier_head
    );
}

#[test]
fn the_loader_round_trips_both_default_tables() {
    for expected in [WeaponTable::normal(), WeaponTable::realistic()] {
        let ini = expected.to_ini();
        let parsed = WeaponTable::parse_ini(&ini, &WeaponLimits::default()).unwrap();
        assert_eq!(parsed, expected);
    }
}

#[test]
fn the_loader_rejects_unknown_and_duplicate_keys_and_sections() {
    let base = WeaponTable::normal().to_ini();

    let unknown_key = base.replace("[HK MP5]", "[HK MP5]\nMagic=1");
    assert!(matches!(
        WeaponTable::parse_ini(&unknown_key, &WeaponLimits::default()),
        Err(WeaponConfigError::UnknownKey { section, key, line })
            if section == "HK MP5" && key == "Magic" && line > 0
    ));

    let duplicate_key = base.replace("[HK MP5]", "[HK MP5]\nAmmo=30");
    assert!(matches!(
        WeaponTable::parse_ini(&duplicate_key, &WeaponLimits::default()),
        Err(WeaponConfigError::DuplicateKey { section, key, .. }) if section == "HK MP5" && key == "Ammo"
    ));

    let unknown_section = format!("{base}\n[Laser Cannon]\nDamage=1\n");
    assert!(matches!(
        WeaponTable::parse_ini(&unknown_section, &WeaponLimits::default()),
        Err(WeaponConfigError::UnknownSection { section, .. }) if section == "Laser Cannon"
    ));

    let duplicate_section = format!("{base}\n[HK MP5]\n");
    assert!(matches!(
        WeaponTable::parse_ini(&duplicate_section, &WeaponLimits::default()),
        Err(WeaponConfigError::DuplicateSection { section, .. }) if section == "HK MP5"
    ));
}

#[test]
fn the_loader_rejects_missing_sections_and_missing_keys() {
    let base = WeaponTable::normal().to_ini();

    let missing_key = base.replacen("Damage=1.81\n", "", 1);
    assert!(matches!(
        WeaponTable::parse_ini(&missing_key, &WeaponLimits::default()),
        Err(WeaponConfigError::MissingKey { section, key }) if section == "Desert Eagles" && key == "Damage"
    ));

    let missing_section = base
        .split("[Chainsaw]")
        .next()
        .expect("split always yields a head")
        .to_string();
    assert!(matches!(
        WeaponTable::parse_ini(&missing_section, &WeaponLimits::default()),
        Err(WeaponConfigError::MissingSection { section }) if section == "Chainsaw"
    ));
}

#[test]
fn the_loader_rejects_values_that_are_out_of_range_or_not_finite() {
    let base = WeaponTable::normal().to_ini();
    let limits = WeaponLimits::default();

    for (from, to, field) in [
        ("Ammo=7", "Ammo=999", "Ammo"),
        ("FireInterval=24", "FireInterval=0", "FireInterval"),
        ("Speed=19", "Speed=-19", "Speed"),
        ("Damage=1.81", "Damage=-1", "Damage"),
        ("ModifierHead=1.1", "ModifierHead=99", "ModifierHead"),
        ("BulletStyle=1", "BulletStyle=42", "BulletStyle"),
    ] {
        let tweaked = base.replacen(from, to, 1);
        assert_ne!(tweaked, base, "{field} substitution did not apply");
        match WeaponTable::parse_ini(&tweaked, &limits) {
            Err(WeaponConfigError::OutOfRange { key, .. }) => assert_eq!(key, field),
            other => panic!("{field} out of range was accepted: {other:?}"),
        }
    }

    for bad in ["Damage=nan", "Damage=inf", "Damage=", "Damage=x"] {
        let tweaked = base.replacen("Damage=1.81", bad, 1);
        assert!(
            matches!(
                WeaponTable::parse_ini(&tweaked, &limits),
                Err(WeaponConfigError::InvalidValue { .. })
            ),
            "{bad} was accepted"
        );
    }
}

#[test]
fn the_loader_bounds_the_input_before_reading_it() {
    let limits = WeaponLimits {
        max_bytes: 64,
        ..WeaponLimits::default()
    };
    assert!(matches!(
        WeaponTable::parse_ini(&WeaponTable::normal().to_ini(), &limits),
        Err(WeaponConfigError::TooLarge { .. })
    ));

    let long_line = format!("[Info]\nName={}\n", "n".repeat(10_000));
    assert!(matches!(
        WeaponTable::parse_ini(&long_line, &WeaponLimits::default()),
        Err(WeaponConfigError::LineTooLong { .. })
    ));
}

#[test]
fn units_are_documented_and_no_collision_defaults_to_colliding_with_everything() {
    let table = WeaponTable::normal();
    let eagles = table.get(WeaponKind::DesertEagles);

    // Ticks, not seconds: 60 ticks is one second.
    assert_eq!(eagles.fire_interval_seconds(), 24.0 / 60.0);
    assert_eq!(eagles.reload_seconds(), 87.0 / 60.0);
    assert_eq!(
        table.get(WeaponKind::Barrett).start_up_seconds(),
        19.0 / 60.0
    );

    WeaponTable::normal()
        .validate(&WeaponLimits::default())
        .unwrap();
    assert_eq!(eagles.no_collision, NoCollision::NONE);
    assert!(eagles.no_collision.collides_with_enemy());
    assert!(eagles.no_collision.collides_with_self());

    let ini = table
        .to_ini()
        .replace("[HK MP5]", "[HK MP5]\nNoCollision=5");
    let parsed = WeaponTable::parse_ini(&ini, &WeaponLimits::default()).unwrap();
    let mp5 = parsed.get(WeaponKind::Mp5);
    assert!(!mp5.no_collision.collides_with_enemy());
    assert!(mp5.no_collision.collides_with_team());
    assert!(!mp5.no_collision.collides_with_self());
}

#[test]
fn a_default_table_travels_as_name_hash_and_hud_names() {
    let json = serde_json::to_value(WeaponTable::normal()).unwrap();
    assert_eq!(json["name"], "Default mod");
    assert_eq!(json["hash"], 911_431_262);
    assert!(json.get("defs").is_none());
    let names = json["names"]
        .as_array()
        .expect("HUD names travel with the table");
    assert_eq!(names.len(), 14);
    assert_eq!(names[13], "LAW");
    WeaponTable::normal()
        .validate(&WeaponLimits::default())
        .unwrap();
}
