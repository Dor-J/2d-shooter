use content::pms::{encode, parse, pms_hash, ContentError, ContentLimits};

fn push_fixed_string(bytes: &mut Vec<u8>, value: &str, field_size: usize) {
    bytes.push(value.len() as u8);
    bytes.extend_from_slice(value.as_bytes());
    bytes.resize(bytes.len() + field_size - value.len(), 0);
}

fn minimal_pms() -> Vec<u8> {
    pms_with_geometry(0, &[])
}

fn pms_with_geometry(polygon_count: i32, sector_indices: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&1_i32.to_le_bytes());
    push_fixed_string(&mut bytes, "Fixture", 38);
    push_fixed_string(&mut bytes, "fixture.png", 24);
    bytes.extend_from_slice(&[10, 20, 30, 255]);
    bytes.extend_from_slice(&[40, 50, 60, 255]);
    bytes.extend_from_slice(&100_i32.to_le_bytes());
    bytes.extend_from_slice(&[2, 3, 1, 4]);
    bytes.extend_from_slice(&7_i32.to_le_bytes());
    bytes.extend_from_slice(&polygon_count.to_le_bytes());
    for _ in 0..polygon_count {
        bytes.extend_from_slice(&[0; 121]);
    }
    bytes.extend_from_slice(&50_i32.to_le_bytes());
    bytes.extend_from_slice(&0_i32.to_le_bytes());
    bytes.extend_from_slice(&(sector_indices.len() as u16).to_le_bytes());
    for index in sector_indices {
        bytes.extend_from_slice(&index.to_le_bytes());
    }
    for _ in 0..5 {
        bytes.extend_from_slice(&0_i32.to_le_bytes());
    }
    bytes
}

#[test]
fn parses_a_bounded_minimal_pms_map() {
    let map = parse(&minimal_pms(), &ContentLimits::default()).unwrap();

    assert_eq!(map.version, 1);
    assert_eq!(map.name, "Fixture");
    assert_eq!(map.texture, "fixture.png");
    assert_eq!(map.start_jet, 100);
    assert_eq!(map.grenade_packs, 2);
    assert_eq!(map.medikits, 3);
    assert_eq!(map.weather, 1);
    assert_eq!(map.steps, 4);
    assert_eq!(map.random_id, 7);
    assert!(map.polygons.is_empty());
    assert_eq!(map.sectors.len(), 1);
}

#[test]
fn rejects_truncated_input_at_the_exact_missing_field() {
    let bytes = minimal_pms();
    let error = parse(&bytes[..bytes.len() - 1], &ContentLimits::default()).unwrap_err();

    assert!(matches!(error, ContentError::Truncated { .. }));
}

#[test]
fn rejects_polygon_counts_above_the_configured_limit_before_allocating() {
    let mut bytes = minimal_pms();
    let polygon_count_offset = 4 + 39 + 25 + 8 + 4 + 4 + 4;
    bytes[polygon_count_offset..polygon_count_offset + 4].copy_from_slice(&2_i32.to_le_bytes());
    let limits = ContentLimits {
        max_polygons: 1,
        ..ContentLimits::default()
    };

    let error = parse(&bytes, &limits).unwrap_err();

    assert_eq!(
        error,
        ContentError::CountOutOfRange {
            field: "polygons",
            value: 2,
            max: 1,
        }
    );
}

#[test]
fn rejects_sector_references_to_nonexistent_polygons() {
    let bytes = pms_with_geometry(1, &[2]);

    let error = parse(&bytes, &ContentLimits::default()).unwrap_err();

    assert_eq!(
        error,
        ContentError::InvalidReference {
            field: "sector polygon",
            value: 2,
            count: 1,
        }
    );
}

#[test]
fn rejects_texture_paths_that_escape_the_content_root() {
    let mut bytes = minimal_pms();
    let texture_offset = 4 + 39;
    bytes[texture_offset] = 8;
    bytes[texture_offset + 1..texture_offset + 9].copy_from_slice(b"../x.png");

    let error = parse(&bytes, &ContentLimits::default()).unwrap_err();

    assert_eq!(error, ContentError::UnsafePath { field: "texture" });
}

#[test]
fn rejects_unsupported_pms_versions() {
    let mut bytes = minimal_pms();
    bytes[..4].copy_from_slice(&2_i32.to_le_bytes());

    let error = parse(&bytes, &ContentLimits::default()).unwrap_err();

    assert_eq!(error, ContentError::UnsupportedVersion { value: 2 });
}

#[test]
fn computes_the_opensoldat_map_hash() {
    assert_eq!(pms_hash(b"123456789"), 0x6e90_93c9);
    assert_eq!(
        parse(&minimal_pms(), &ContentLimits::default())
            .unwrap()
            .hash,
        pms_hash(&minimal_pms())
    );
}

#[test]
fn every_truncated_prefix_returns_an_error_without_panicking() {
    let bytes = minimal_pms();

    for length in 0..bytes.len() {
        assert!(
            parse(&bytes[..length], &ContentLimits::default()).is_err(),
            "prefix of {length} bytes was accepted"
        );
    }
}

#[test]
fn bounded_mutations_never_panic() {
    let original = minimal_pms();
    for seed in 0_u32..512 {
        let mut bytes = original.clone();
        let index = seed as usize % bytes.len();
        bytes[index] ^= (seed.wrapping_mul(31) >> 3) as u8;
        let _ = parse(&bytes, &ContentLimits::default());
    }
}

#[test]
fn pms_writer_round_trips_a_valid_map() {
    let original = parse(&minimal_pms(), &ContentLimits::default()).unwrap();
    let encoded = encode(&original).unwrap();
    let decoded = parse(&encoded, &ContentLimits::default()).unwrap();

    assert_eq!(decoded.name, original.name);
    assert_eq!(decoded.texture, original.texture);
    assert_eq!(decoded.start_jet, original.start_jet);
    assert_eq!(decoded.sectors.len(), original.sectors.len());
}
