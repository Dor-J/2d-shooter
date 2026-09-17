use content::manifest::MapMode;
use content::pms::{MapAsset, Polygon, Sector, Spawnpoint, Vertex, Waypoint};

#[derive(Clone, Copy, Debug)]
pub struct DefaultMap {
    pub name: &'static str,
    pub mode: MapMode,
}

macro_rules! maps {
    ($($mode:ident => [$($name:literal),+]),+ $(,)?) => {
        &[$($(DefaultMap { name: $name, mode: MapMode::$mode }),+),+]
    };
}

pub const DEFAULT_MAPS: &[DefaultMap] = maps!(
    Deathmatch => ["Aero", "Airpirates", "Arena", "Arena2", "Arena3", "Bigfalls", "Blox", "Bridge", "Bunker", "Cambodia", "CrackedBoot", "Daybreak", "DesertWind", "Factory", "Flashback", "HH", "Island2k5", "Jungle", "Krab", "Lagrange", "Leaf", "MrSnowman", "RatCave", "Rok", "RR", "Shau", "Tropiccave", "Unlim", "Veoto"],
    CaptureTheFlag => ["ctf_Ash", "ctf_B2b", "ctf_Blade", "ctf_Campeche", "ctf_Cobra", "ctf_Crucifix", "ctf_Death", "ctf_Division", "ctf_Dropdown", "ctf_Equinox", "ctf_Guardian", "ctf_Hormone", "ctf_IceBeam", "ctf_Kampf", "ctf_Lanubya", "ctf_Laos", "ctf_Maya", "ctf_Mayapan", "ctf_MFM", "ctf_Nuubia", "ctf_Raspberry", "ctf_Rotten", "ctf_Ruins", "ctf_Run", "ctf_Scorpion", "ctf_Snakebite", "ctf_Steel", "ctf_Triumph", "ctf_Viet", "ctf_Voland", "ctf_Wretch", "ctf_X"],
    HoldTheFlag => ["htf_Arch", "htf_Baire", "htf_Boxed", "htf_Desert", "htf_Dorothy", "htf_Dusk", "htf_Erbium", "htf_Feast", "htf_Mossy", "htf_Muygen", "htf_Niall", "htf_Nuclear", "htf_Prison", "htf_Rubik", "htf_Star", "htf_Tower", "htf_Void", "htf_Vortex", "htf_Zajacz"],
    Infiltration => ["inf_Abel", "inf_April", "inf_Argy", "inf_Belltower", "inf_Biologic", "inf_Changeling", "inf_Flute", "inf_Fortress", "inf_Industrial", "inf_Messner", "inf_Moonshine", "inf_Motheaten", "inf_Outpost", "inf_Rescue", "inf_Rise", "inf_Warehouse", "inf_Warlock"],
);

pub fn original_default_map(entry: DefaultMap) -> MapAsset {
    let seed = entry.name.bytes().fold(0_u32, |value, byte| {
        value.wrapping_mul(33).wrapping_add(u32::from(byte))
    });
    let platform_y = 390.0 + (seed % 100) as f32;
    let polygons = vec![
        polygon([(0.0, 650.0), (1200.0, 650.0), (1200.0, 700.0)], 0),
        polygon([(0.0, 650.0), (1200.0, 700.0), (0.0, 700.0)], 0),
        polygon(
            [
                (180.0, platform_y),
                (520.0, platform_y - 30.0),
                (520.0, platform_y + 20.0),
            ],
            0,
        ),
        polygon(
            [
                (680.0, platform_y + 20.0),
                (1020.0, platform_y - 10.0),
                (680.0, platform_y - 30.0),
            ],
            if seed % 7 == 0 { 4 } else { 0 },
        ),
    ];
    let mut spawnpoints = vec![spawn(260, 600, 0), spawn(940, 600, 0)];
    if matches!(entry.mode, MapMode::CaptureTheFlag | MapMode::Infiltration) {
        spawnpoints.extend([
            spawn(180, 600, 1),
            spawn(1020, 600, 2),
            spawn(100, 610, 5),
            spawn(1100, 610, 6),
        ]);
    }
    MapAsset {
        hash: 0,
        version: 1,
        name: entry.name.into(),
        texture: "generated.ppm".into(),
        background_top: [20 + (seed % 40) as u8, 35, 55, 255],
        background_bottom: [70, 80 + (seed % 50) as u8, 90, 255],
        start_jet: 100 + (seed % 120) as i32,
        grenade_packs: 2,
        medikits: 2,
        weather: (seed % 4) as u8,
        steps: (seed % 5) as u8,
        random_id: seed as i32,
        polygons,
        sectors_division: 50,
        sectors_num: 0,
        sectors: vec![Sector {
            polygons: vec![1, 2, 3, 4],
        }],
        props: Vec::new(),
        scenery: Vec::new(),
        colliders: Vec::new(),
        spawnpoints,
        waypoints: vec![waypoint(1, 260, 600, 2), waypoint(2, 940, 600, 1)],
    }
}

fn vertex(x: f32, y: f32) -> Vertex {
    Vertex {
        x,
        y,
        z: 0.0,
        rhw: 1.0,
        color: [180, 190, 190, 255],
        u: x / 1200.0,
        v: y / 700.0,
    }
}
fn polygon(points: [(f32, f32); 3], kind: u8) -> Polygon {
    Polygon {
        vertices: points.map(|(x, y)| vertex(x, y)),
        normals: [[0.0, -1.0, 1.0]; 3],
        kind,
    }
}
fn spawn(x: i32, y: i32, team: i32) -> Spawnpoint {
    Spawnpoint {
        active: true,
        x,
        y,
        team,
    }
}
fn waypoint(id: i32, x: i32, y: i32, connection: i32) -> Waypoint {
    let mut connections = [0; 20];
    connections[0] = connection;
    Waypoint {
        active: true,
        id,
        x,
        y,
        left: true,
        right: true,
        up: false,
        down: false,
        jetpack: true,
        path_num: 0,
        action: 0,
        connections_num: 1,
        connections,
    }
}
