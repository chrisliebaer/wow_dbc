use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type SpellMissileKey = crate::PrimaryKey<i32, SpellMissile>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellMissile {
    pub rows: Vec<SpellMissileRow>,
}

impl SpellMissile {
    pub const FILENAME: &'static str = "SpellMissile.dbc";
    pub const FIELD_COUNT: usize = 15;
    pub const ROW_SIZE: usize = 60;

}

impl Into<WrathTable> for SpellMissile {
    fn into(self) -> WrathTable {
        WrathTable::SpellMissile(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellMissile {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellMissileRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellMissileRow] { &mut self.rows }

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

            // id: primary_key (SpellMissile) int32
            let id = SpellMissileKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // default_pitch_min: float
            let default_pitch_min = crate::util::read_f32_le(chunk)?;

            // default_pitch_max: float
            let default_pitch_max = crate::util::read_f32_le(chunk)?;

            // default_speed_min: float
            let default_speed_min = crate::util::read_f32_le(chunk)?;

            // default_speed_max: float
            let default_speed_max = crate::util::read_f32_le(chunk)?;

            // randomize_facing_min: float
            let randomize_facing_min = crate::util::read_f32_le(chunk)?;

            // randomize_facing_max: float
            let randomize_facing_max = crate::util::read_f32_le(chunk)?;

            // randomize_pitch_min: float
            let randomize_pitch_min = crate::util::read_f32_le(chunk)?;

            // randomize_pitch_max: float
            let randomize_pitch_max = crate::util::read_f32_le(chunk)?;

            // randomize_speed_min: float
            let randomize_speed_min = crate::util::read_f32_le(chunk)?;

            // randomize_speed_max: float
            let randomize_speed_max = crate::util::read_f32_le(chunk)?;

            // gravity: float
            let gravity = crate::util::read_f32_le(chunk)?;

            // max_duration: float
            let max_duration = crate::util::read_f32_le(chunk)?;

            // collision_radius: float
            let collision_radius = crate::util::read_f32_le(chunk)?;


            rows.push(SpellMissileRow {
                id,
                flags,
                default_pitch_min,
                default_pitch_max,
                default_speed_min,
                default_speed_max,
                randomize_facing_min,
                randomize_facing_max,
                randomize_pitch_min,
                randomize_pitch_max,
                randomize_speed_min,
                randomize_speed_max,
                gravity,
                max_duration,
                collision_radius,
            });
        }

        Ok(SpellMissile { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellMissile) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // default_pitch_min: float
            b.write_all(&row.default_pitch_min.to_le_bytes())?;

            // default_pitch_max: float
            b.write_all(&row.default_pitch_max.to_le_bytes())?;

            // default_speed_min: float
            b.write_all(&row.default_speed_min.to_le_bytes())?;

            // default_speed_max: float
            b.write_all(&row.default_speed_max.to_le_bytes())?;

            // randomize_facing_min: float
            b.write_all(&row.randomize_facing_min.to_le_bytes())?;

            // randomize_facing_max: float
            b.write_all(&row.randomize_facing_max.to_le_bytes())?;

            // randomize_pitch_min: float
            b.write_all(&row.randomize_pitch_min.to_le_bytes())?;

            // randomize_pitch_max: float
            b.write_all(&row.randomize_pitch_max.to_le_bytes())?;

            // randomize_speed_min: float
            b.write_all(&row.randomize_speed_min.to_le_bytes())?;

            // randomize_speed_max: float
            b.write_all(&row.randomize_speed_max.to_le_bytes())?;

            // gravity: float
            b.write_all(&row.gravity.to_le_bytes())?;

            // max_duration: float
            b.write_all(&row.max_duration.to_le_bytes())?;

            // collision_radius: float
            b.write_all(&row.collision_radius.to_le_bytes())?;

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
impl Indexable<i32> for SpellMissile {
    type Table = Self;

    fn get(&self, key: &SpellMissileKey) -> Option<&SpellMissileRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellMissileKey) -> Option<&mut SpellMissileRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellMissileRow {
    pub id: SpellMissileKey,
    pub flags: i32,
    pub default_pitch_min: f32,
    pub default_pitch_max: f32,
    pub default_speed_min: f32,
    pub default_speed_max: f32,
    pub randomize_facing_min: f32,
    pub randomize_facing_max: f32,
    pub randomize_pitch_min: f32,
    pub randomize_pitch_max: f32,
    pub randomize_speed_min: f32,
    pub randomize_speed_max: f32,
    pub gravity: f32,
    pub max_duration: f32,
    pub collision_radius: f32,
}

impl DbcRow for SpellMissileRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_missile() {
        let mut file = File::open("../wrath-dbc/SpellMissile.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellMissile::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellMissile::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
