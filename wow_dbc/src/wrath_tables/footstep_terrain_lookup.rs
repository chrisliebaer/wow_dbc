use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::WrathTable;

pub type FootstepTerrainLookupKey = crate::PrimaryKey<i32, FootstepTerrainLookup>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootstepTerrainLookup {
    pub rows: Vec<FootstepTerrainLookupRow>,
}

impl FootstepTerrainLookup {
    pub const FILENAME: &'static str = "FootstepTerrainLookup.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

    pub fn verify(&self, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&FootstepTerrainLookupRow>> {
        for row in &self.rows {
            if row.sound_id.id != 0 && sound_entries.get(&row.sound_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<FootstepTerrainLookup>(),
                    row,
                    id,
                    row.sound_id.id.into()
                ));
            }

            if row.sound_id_splash.id != 0 && sound_entries.get(&row.sound_id_splash).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<FootstepTerrainLookup>(),
                    row,
                    id,
                    row.sound_id_splash.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for FootstepTerrainLookup {
    fn into(self) -> WrathTable {
        WrathTable::FootstepTerrainLookup(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for FootstepTerrainLookup {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[FootstepTerrainLookupRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [FootstepTerrainLookupRow] { &mut self.rows }

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

            // id: primary_key (FootstepTerrainLookup) int32
            let id = FootstepTerrainLookupKey::new(crate::util::read_i32_le(chunk)?);

            // creature_footstep_id: int32
            let creature_footstep_id = crate::util::read_i32_le(chunk)?;

            // terrain_sound_id: int32
            let terrain_sound_id = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_id_splash: foreign_key (SoundEntries) int32
            let sound_id_splash = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(FootstepTerrainLookupRow {
                id,
                creature_footstep_id,
                terrain_sound_id,
                sound_id,
                sound_id_splash,
            });
        }

        Ok(FootstepTerrainLookup { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (FootstepTerrainLookup) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // creature_footstep_id: int32
            b.write_all(&row.creature_footstep_id.to_le_bytes())?;

            // terrain_sound_id: int32
            b.write_all(&row.terrain_sound_id.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // sound_id_splash: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id_splash.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for FootstepTerrainLookup {
    type Table = Self;

    fn get(&self, key: &FootstepTerrainLookupKey) -> Option<&FootstepTerrainLookupRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &FootstepTerrainLookupKey) -> Option<&mut FootstepTerrainLookupRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootstepTerrainLookupRow {
    pub id: FootstepTerrainLookupKey,
    pub creature_footstep_id: i32,
    pub terrain_sound_id: i32,
    pub sound_id: SoundEntriesKey,
    pub sound_id_splash: SoundEntriesKey,
}

impl DbcRow for FootstepTerrainLookupRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn footstep_terrain_lookup() {
        let mut file = File::open("../wrath-dbc/FootstepTerrainLookup.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = FootstepTerrainLookup::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = FootstepTerrainLookup::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
