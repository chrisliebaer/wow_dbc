use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type BannedAddOnsKey = crate::PrimaryKey<i32, BannedAddOns>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BannedAddOns {
    pub rows: Vec<BannedAddOnsRow>,
}

impl BannedAddOns {
    pub const FILENAME: &'static str = "BannedAddOns.dbc";
    pub const FIELD_COUNT: usize = 11;
    pub const ROW_SIZE: usize = 44;

}

impl Into<WrathTable> for BannedAddOns {
    fn into(self) -> WrathTable {
        WrathTable::BannedAddOns(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for BannedAddOns {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[BannedAddOnsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [BannedAddOnsRow] { &mut self.rows }

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

            // id: primary_key (BannedAddOns) int32
            let id = BannedAddOnsKey::new(crate::util::read_i32_le(chunk)?);

            // name_m_d5: int32[4]
            let name_m_d5 = crate::util::read_array_i32::<4>(chunk)?;

            // version_m_d5: int32[4]
            let version_m_d5 = crate::util::read_array_i32::<4>(chunk)?;

            // last_modified: int32
            let last_modified = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(BannedAddOnsRow {
                id,
                name_m_d5,
                version_m_d5,
                last_modified,
                flags,
            });
        }

        Ok(BannedAddOns { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (BannedAddOns) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_m_d5: int32[4]
            for i in row.name_m_d5 {
                b.write_all(&i.to_le_bytes())?;
            }


            // version_m_d5: int32[4]
            for i in row.version_m_d5 {
                b.write_all(&i.to_le_bytes())?;
            }


            // last_modified: int32
            b.write_all(&row.last_modified.to_le_bytes())?;

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
impl Indexable<i32> for BannedAddOns {
    type Table = Self;

    fn get(&self, key: &BannedAddOnsKey) -> Option<&BannedAddOnsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &BannedAddOnsKey) -> Option<&mut BannedAddOnsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BannedAddOnsRow {
    pub id: BannedAddOnsKey,
    pub name_m_d5: [i32; 4],
    pub version_m_d5: [i32; 4],
    pub last_modified: i32,
    pub flags: i32,
}

impl DbcRow for BannedAddOnsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn banned_add_ons() {
        let mut file = File::open("../wrath-dbc/BannedAddOns.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = BannedAddOns::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = BannedAddOns::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
