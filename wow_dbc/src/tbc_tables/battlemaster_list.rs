use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type BattlemasterListKey = crate::PrimaryKey<i32, BattlemasterList>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BattlemasterList {
    pub rows: Vec<BattlemasterListRow>,
}

impl BattlemasterList {
    pub const FILENAME: &'static str = "BattlemasterList.dbc";
    pub const FIELD_COUNT: usize = 33;
    pub const ROW_SIZE: usize = 132;

}

impl Into<TbcTable> for BattlemasterList {
    fn into(self) -> TbcTable {
        TbcTable::BattlemasterList(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for BattlemasterList {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[BattlemasterListRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [BattlemasterListRow] { &mut self.rows }

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

            // id: primary_key (BattlemasterList) int32
            let id = BattlemasterListKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: int32[8]
            let map_id = crate::util::read_array_i32::<8>(chunk)?;

            // instance_type: int32
            let instance_type = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // field_2_0_0_5610_005: int32
            let field_2_0_0_5610_005 = crate::util::read_i32_le(chunk)?;

            // field_2_0_0_5610_006: int32
            let field_2_0_0_5610_006 = crate::util::read_i32_le(chunk)?;

            // groups_allowed: int32
            let groups_allowed = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // field_2_4_0_8089_009: int32
            let field_2_4_0_8089_009 = crate::util::read_i32_le(chunk)?;


            rows.push(BattlemasterListRow {
                id,
                map_id,
                instance_type,
                min_level,
                max_level,
                field_2_0_0_5610_005,
                field_2_0_0_5610_006,
                groups_allowed,
                name_lang,
                field_2_4_0_8089_009,
            });
        }

        Ok(BattlemasterList { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (BattlemasterList) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: int32[8]
            for i in row.map_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // instance_type: int32
            b.write_all(&row.instance_type.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // field_2_0_0_5610_005: int32
            b.write_all(&row.field_2_0_0_5610_005.to_le_bytes())?;

            // field_2_0_0_5610_006: int32
            b.write_all(&row.field_2_0_0_5610_006.to_le_bytes())?;

            // groups_allowed: int32
            b.write_all(&row.groups_allowed.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // field_2_4_0_8089_009: int32
            b.write_all(&row.field_2_4_0_8089_009.to_le_bytes())?;

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
impl Indexable<i32> for BattlemasterList {
    type Table = Self;

    fn get(&self, key: &BattlemasterListKey) -> Option<&BattlemasterListRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &BattlemasterListKey) -> Option<&mut BattlemasterListRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BattlemasterListRow {
    pub id: BattlemasterListKey,
    pub map_id: [i32; 8],
    pub instance_type: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub field_2_0_0_5610_005: i32,
    pub field_2_0_0_5610_006: i32,
    pub groups_allowed: i32,
    pub name_lang: ExtendedLocalizedString,
    pub field_2_4_0_8089_009: i32,
}

impl DbcRow for BattlemasterListRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn battlemaster_list() {
        let mut file = File::open("../tbc-dbc/BattlemasterList.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = BattlemasterList::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = BattlemasterList::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
