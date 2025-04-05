use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type CameraShakesKey = crate::PrimaryKey<i32, CameraShakes>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CameraShakes {
    pub rows: Vec<CameraShakesRow>,
}

impl CameraShakes {
    pub const FILENAME: &'static str = "CameraShakes.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

}

impl Into<WrathTable> for CameraShakes {
    fn into(self) -> WrathTable {
        WrathTable::CameraShakes(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CameraShakes {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CameraShakesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CameraShakesRow] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CameraShakes) int32
            let id = CameraShakesKey::new(crate::util::read_i32_le(chunk)?);

            // shake_type: int32
            let shake_type = crate::util::read_i32_le(chunk)?;

            // direction: int32
            let direction = crate::util::read_i32_le(chunk)?;

            // amplitude: float
            let amplitude = crate::util::read_f32_le(chunk)?;

            // frequency: float
            let frequency = crate::util::read_f32_le(chunk)?;

            // duration: float
            let duration = crate::util::read_f32_le(chunk)?;

            // phase: float
            let phase = crate::util::read_f32_le(chunk)?;

            // coefficient: float
            let coefficient = crate::util::read_f32_le(chunk)?;


            rows.push(CameraShakesRow {
                id,
                shake_type,
                direction,
                amplitude,
                frequency,
                duration,
                phase,
                coefficient,
            });
        }

        Ok(CameraShakes { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CameraShakes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // shake_type: int32
            b.write_all(&row.shake_type.to_le_bytes())?;

            // direction: int32
            b.write_all(&row.direction.to_le_bytes())?;

            // amplitude: float
            b.write_all(&row.amplitude.to_le_bytes())?;

            // frequency: float
            b.write_all(&row.frequency.to_le_bytes())?;

            // duration: float
            b.write_all(&row.duration.to_le_bytes())?;

            // phase: float
            b.write_all(&row.phase.to_le_bytes())?;

            // coefficient: float
            b.write_all(&row.coefficient.to_le_bytes())?;

        }

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

#[allow(refining_impl_trait)]
impl Indexable<i32> for CameraShakes {
    type Table = Self;

    fn get(&self, key: &CameraShakesKey) -> Option<&CameraShakesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CameraShakesKey) -> Option<&mut CameraShakesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CameraShakesRow {
    pub id: CameraShakesKey,
    pub shake_type: i32,
    pub direction: i32,
    pub amplitude: f32,
    pub frequency: f32,
    pub duration: f32,
    pub phase: f32,
    pub coefficient: f32,
}

impl DbcRow for CameraShakesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn camera_shakes() {
        let mut file = File::open("../wrath-dbc/CameraShakes.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CameraShakes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CameraShakes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
