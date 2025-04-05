use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type OverrideSpellDataKey = crate::PrimaryKey<i32, OverrideSpellData>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OverrideSpellData {
    pub rows: Vec<OverrideSpellDataRow>,
}

impl OverrideSpellData {
    pub const FILENAME: &'static str = "OverrideSpellData.dbc";
    pub const FIELD_COUNT: usize = 12;
    pub const ROW_SIZE: usize = 48;

}

impl Into<WrathTable> for OverrideSpellData {
    fn into(self) -> WrathTable {
        WrathTable::OverrideSpellData(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for OverrideSpellData {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[OverrideSpellDataRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [OverrideSpellDataRow] { &mut self.rows }

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

            // id: primary_key (OverrideSpellData) int32
            let id = OverrideSpellDataKey::new(crate::util::read_i32_le(chunk)?);

            // spells: int32[10]
            let spells = crate::util::read_array_i32::<10>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(OverrideSpellDataRow {
                id,
                spells,
                flags,
            });
        }

        Ok(OverrideSpellData { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (OverrideSpellData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // spells: int32[10]
            for i in row.spells {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

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
impl Indexable<i32> for OverrideSpellData {
    type Table = Self;

    fn get(&self, key: &OverrideSpellDataKey) -> Option<&OverrideSpellDataRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &OverrideSpellDataKey) -> Option<&mut OverrideSpellDataRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OverrideSpellDataRow {
    pub id: OverrideSpellDataKey,
    pub spells: [i32; 10],
    pub flags: i32,
}

impl DbcRow for OverrideSpellDataRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn override_spell_data() {
        let mut file = File::open("../wrath-dbc/OverrideSpellData.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = OverrideSpellData::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = OverrideSpellData::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
