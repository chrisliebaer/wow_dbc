use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::map::{
    Map, MapKey,
};
use std::io::Write;
use super::WrathTable;

pub type MapDifficultyKey = crate::PrimaryKey<i32, MapDifficulty>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapDifficulty {
    pub rows: Vec<MapDifficultyRow>,
}

impl MapDifficulty {
    pub const FILENAME: &'static str = "MapDifficulty.dbc";
    pub const FIELD_COUNT: usize = 23;
    pub const ROW_SIZE: usize = 92;

    pub fn verify(&self, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&MapDifficultyRow>> {
        for row in &self.rows {
            if row.map_id.id != 0 && map.get(&row.map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<MapDifficulty>(),
                    row,
                    id,
                    row.map_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for MapDifficulty {
    fn into(self) -> WrathTable {
        WrathTable::MapDifficulty(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for MapDifficulty {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[MapDifficultyRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [MapDifficultyRow] { &mut self.rows }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (MapDifficulty) int32
            let id = MapDifficultyKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;

            // message_lang: string_ref_loc (Extended)
            let message_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // raid_duration: int32
            let raid_duration = crate::util::read_i32_le(chunk)?;

            // max_players: int32
            let max_players = crate::util::read_i32_le(chunk)?;

            // difficultystring: string_ref
            let difficultystring = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(MapDifficultyRow {
                id,
                map_id,
                difficulty,
                message_lang,
                raid_duration,
                max_players,
                difficultystring,
            });
        }

        Ok(MapDifficulty { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (MapDifficulty) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

            // message_lang: string_ref_loc (Extended)
            b.write_all(&row.message_lang.string_indices_as_array(&mut string_cache))?;

            // raid_duration: int32
            b.write_all(&row.raid_duration.to_le_bytes())?;

            // max_players: int32
            b.write_all(&row.max_players.to_le_bytes())?;

            // difficultystring: string_ref
            b.write_all(&string_cache.add_string(&row.difficultystring).to_le_bytes())?;

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
impl Indexable<i32> for MapDifficulty {
    type Table = Self;

    fn get(&self, key: &MapDifficultyKey) -> Option<&MapDifficultyRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &MapDifficultyKey) -> Option<&mut MapDifficultyRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapDifficultyRow {
    pub id: MapDifficultyKey,
    pub map_id: MapKey,
    pub difficulty: i32,
    pub message_lang: ExtendedLocalizedString,
    pub raid_duration: i32,
    pub max_players: i32,
    pub difficultystring: String,
}

impl DbcRow for MapDifficultyRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn map_difficulty() {
        let mut file = File::open("../wrath-dbc/MapDifficulty.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = MapDifficulty::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = MapDifficulty::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
