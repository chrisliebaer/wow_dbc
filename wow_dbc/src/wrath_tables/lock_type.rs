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

pub type LockTypeKey = crate::PrimaryKey<i32, LockType>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockType {
    pub rows: Vec<LockTypeRow>,
}

impl LockType {
    pub const FILENAME: &'static str = "LockType.dbc";
    pub const FIELD_COUNT: usize = 53;
    pub const ROW_SIZE: usize = 212;

}

impl Into<WrathTable> for LockType {
    fn into(self) -> WrathTable {
        WrathTable::LockType(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for LockType {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[LockTypeRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [LockTypeRow] { &mut self.rows }

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

            // id: primary_key (LockType) int32
            let id = LockTypeKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // resource_name_lang: string_ref_loc (Extended)
            let resource_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // verb_lang: string_ref_loc (Extended)
            let verb_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // cursor_name: string_ref
            let cursor_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(LockTypeRow {
                id,
                name_lang,
                resource_name_lang,
                verb_lang,
                cursor_name,
            });
        }

        Ok(LockType { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LockType) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // resource_name_lang: string_ref_loc (Extended)
            b.write_all(&row.resource_name_lang.string_indices_as_array(&mut string_cache))?;

            // verb_lang: string_ref_loc (Extended)
            b.write_all(&row.verb_lang.string_indices_as_array(&mut string_cache))?;

            // cursor_name: string_ref
            b.write_all(&string_cache.add_string(&row.cursor_name).to_le_bytes())?;

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
impl Indexable<i32> for LockType {
    type Table = Self;

    fn get(&self, key: &LockTypeKey) -> Option<&LockTypeRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &LockTypeKey) -> Option<&mut LockTypeRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockTypeRow {
    pub id: LockTypeKey,
    pub name_lang: ExtendedLocalizedString,
    pub resource_name_lang: ExtendedLocalizedString,
    pub verb_lang: ExtendedLocalizedString,
    pub cursor_name: String,
}

impl DbcRow for LockTypeRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn lock_type() {
        let mut file = File::open("../wrath-dbc/LockType.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LockType::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LockType::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
