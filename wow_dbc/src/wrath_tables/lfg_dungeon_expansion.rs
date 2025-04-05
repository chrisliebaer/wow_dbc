use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type LFGDungeonExpansionKey = crate::PrimaryKey<i32, LFGDungeonExpansion>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonExpansion {
    pub rows: Vec<LFGDungeonExpansionRow>,
}

impl LFGDungeonExpansion {
    pub const FILENAME: &'static str = "LFGDungeonExpansion.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

}

impl Into<WrathTable> for LFGDungeonExpansion {
    fn into(self) -> WrathTable {
        WrathTable::LFGDungeonExpansion(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for LFGDungeonExpansion {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[LFGDungeonExpansionRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [LFGDungeonExpansionRow] { &mut self.rows }

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

            // id: primary_key (LFGDungeonExpansion) int32
            let id = LFGDungeonExpansionKey::new(crate::util::read_i32_le(chunk)?);

            // lfg_id: int32
            let lfg_id = crate::util::read_i32_le(chunk)?;

            // expansion_level: int32
            let expansion_level = crate::util::read_i32_le(chunk)?;

            // random_id: int32
            let random_id = crate::util::read_i32_le(chunk)?;

            // hard_level_min: int32
            let hard_level_min = crate::util::read_i32_le(chunk)?;

            // hard_level_max: int32
            let hard_level_max = crate::util::read_i32_le(chunk)?;

            // target_level_min: int32
            let target_level_min = crate::util::read_i32_le(chunk)?;

            // target_level_max: int32
            let target_level_max = crate::util::read_i32_le(chunk)?;


            rows.push(LFGDungeonExpansionRow {
                id,
                lfg_id,
                expansion_level,
                random_id,
                hard_level_min,
                hard_level_max,
                target_level_min,
                target_level_max,
            });
        }

        Ok(LFGDungeonExpansion { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LFGDungeonExpansion) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // lfg_id: int32
            b.write_all(&row.lfg_id.to_le_bytes())?;

            // expansion_level: int32
            b.write_all(&row.expansion_level.to_le_bytes())?;

            // random_id: int32
            b.write_all(&row.random_id.to_le_bytes())?;

            // hard_level_min: int32
            b.write_all(&row.hard_level_min.to_le_bytes())?;

            // hard_level_max: int32
            b.write_all(&row.hard_level_max.to_le_bytes())?;

            // target_level_min: int32
            b.write_all(&row.target_level_min.to_le_bytes())?;

            // target_level_max: int32
            b.write_all(&row.target_level_max.to_le_bytes())?;

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
impl Indexable<i32> for LFGDungeonExpansion {
    type Table = Self;

    fn get(&self, key: &LFGDungeonExpansionKey) -> Option<&LFGDungeonExpansionRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &LFGDungeonExpansionKey) -> Option<&mut LFGDungeonExpansionRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonExpansionRow {
    pub id: LFGDungeonExpansionKey,
    pub lfg_id: i32,
    pub expansion_level: i32,
    pub random_id: i32,
    pub hard_level_min: i32,
    pub hard_level_max: i32,
    pub target_level_min: i32,
    pub target_level_max: i32,
}

impl DbcRow for LFGDungeonExpansionRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn lfg_dungeon_expansion() {
        let mut file = File::open("../wrath-dbc/LFGDungeonExpansion.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LFGDungeonExpansion::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LFGDungeonExpansion::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
