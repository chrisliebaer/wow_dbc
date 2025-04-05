use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::map::{
    Map, MapKey,
};
use std::io::Write;
use super::WrathTable;

pub type PvpDifficultyKey = crate::PrimaryKey<i32, PvpDifficulty>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PvpDifficulty {
    pub rows: Vec<PvpDifficultyRow>,
}

impl PvpDifficulty {
    pub const FILENAME: &'static str = "PvpDifficulty.dbc";
    pub const FIELD_COUNT: usize = 6;
    pub const ROW_SIZE: usize = 24;

    pub fn verify(&self, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&PvpDifficultyRow>> {
        for row in &self.rows {
            if row.map_id.id != 0 && map.get(&row.map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<PvpDifficulty>(),
                    row,
                    id,
                    row.map_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for PvpDifficulty {
    fn into(self) -> WrathTable {
        WrathTable::PvpDifficulty(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for PvpDifficulty {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[PvpDifficultyRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [PvpDifficultyRow] { &mut self.rows }

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

            // id: primary_key (PvpDifficulty) int32
            let id = PvpDifficultyKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // range_index: int32
            let range_index = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;


            rows.push(PvpDifficultyRow {
                id,
                map_id,
                range_index,
                min_level,
                max_level,
                difficulty,
            });
        }

        Ok(PvpDifficulty { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (PvpDifficulty) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // range_index: int32
            b.write_all(&row.range_index.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

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
impl Indexable<i32> for PvpDifficulty {
    type Table = Self;

    fn get(&self, key: &PvpDifficultyKey) -> Option<&PvpDifficultyRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &PvpDifficultyKey) -> Option<&mut PvpDifficultyRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PvpDifficultyRow {
    pub id: PvpDifficultyKey,
    pub map_id: MapKey,
    pub range_index: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub difficulty: i32,
}

impl DbcRow for PvpDifficultyRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn pvp_difficulty() {
        let mut file = File::open("../wrath-dbc/PvpDifficulty.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = PvpDifficulty::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = PvpDifficulty::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
