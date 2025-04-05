use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type LFGDungeonGroupKey = crate::PrimaryKey<i32, LFGDungeonGroup>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonGroup {
    pub rows: Vec<LFGDungeonGroupRow>,
}

impl LFGDungeonGroup {
    pub const FILENAME: &'static str = "LFGDungeonGroup.dbc";
    pub const FIELD_COUNT: usize = 21;
    pub const ROW_SIZE: usize = 84;

}

impl Into<WrathTable> for LFGDungeonGroup {
    fn into(self) -> WrathTable {
        WrathTable::LFGDungeonGroup(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for LFGDungeonGroup {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[LFGDungeonGroupRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [LFGDungeonGroupRow] { &mut self.rows }

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

            // id: primary_key (LFGDungeonGroup) int32
            let id = LFGDungeonGroupKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // order_index: int32
            let order_index = crate::util::read_i32_le(chunk)?;

            // parent_group_id: int32
            let parent_group_id = crate::util::read_i32_le(chunk)?;

            // type_id: int32
            let type_id = crate::util::read_i32_le(chunk)?;


            rows.push(LFGDungeonGroupRow {
                id,
                name_lang,
                order_index,
                parent_group_id,
                type_id,
            });
        }

        Ok(LFGDungeonGroup { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LFGDungeonGroup) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // order_index: int32
            b.write_all(&row.order_index.to_le_bytes())?;

            // parent_group_id: int32
            b.write_all(&row.parent_group_id.to_le_bytes())?;

            // type_id: int32
            b.write_all(&row.type_id.to_le_bytes())?;

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
impl Indexable<i32> for LFGDungeonGroup {
    type Table = Self;

    fn get(&self, key: &LFGDungeonGroupKey) -> Option<&LFGDungeonGroupRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &LFGDungeonGroupKey) -> Option<&mut LFGDungeonGroupRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonGroupRow {
    pub id: LFGDungeonGroupKey,
    pub name_lang: ExtendedLocalizedString,
    pub order_index: i32,
    pub parent_group_id: i32,
    pub type_id: i32,
}

impl DbcRow for LFGDungeonGroupRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn lfg_dungeon_group() {
        let mut file = File::open("../wrath-dbc/LFGDungeonGroup.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LFGDungeonGroup::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LFGDungeonGroup::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
