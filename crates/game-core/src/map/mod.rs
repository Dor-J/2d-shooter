use content::pms::MapAsset;

#[derive(Clone, Debug)]
pub struct ValidatedMap {
    asset: MapAsset,
}

impl ValidatedMap {
    pub fn asset(&self) -> &MapAsset {
        &self.asset
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapValidationError {
    DegeneratePolygon { index: usize },
    InvalidPolygonKind { index: usize, kind: u8 },
}

impl TryFrom<MapAsset> for ValidatedMap {
    type Error = MapValidationError;

    fn try_from(asset: MapAsset) -> Result<Self, Self::Error> {
        for (index, polygon) in asset.polygons.iter().enumerate() {
            if polygon.kind > 25 {
                return Err(MapValidationError::InvalidPolygonKind {
                    index,
                    kind: polygon.kind,
                });
            }
            let [a, b, c] = polygon.vertices;
            let twice_area = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            if !twice_area.is_finite() || twice_area.abs() <= f32::EPSILON {
                return Err(MapValidationError::DegeneratePolygon { index });
            }
        }
        Ok(Self { asset })
    }
}
