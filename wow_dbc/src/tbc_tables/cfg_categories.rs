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

pub type Cfg_CategoriesKey = crate::PrimaryKey<i32, Cfg_Categories>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cfg_Categories {
    pub rows: Vec<Cfg_CategoriesRow>,
}

impl Cfg_Categories {
    pub const FILENAME: &'static str = "Cfg_Categories.dbc";
    pub const FIELD_COUNT: usize = 21;
    pub const ROW_SIZE: usize = 84;

}

impl Into<TbcTable> for Cfg_Categories {
    fn into(self) -> TbcTable {
        TbcTable::Cfg_Categories(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Cfg_Categories {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[Cfg_CategoriesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Cfg_CategoriesRow] { &mut self.rows }

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

            // id: primary_key (Cfg_Categories) int32
            let id = Cfg_CategoriesKey::new(crate::util::read_i32_le(chunk)?);

            // locale_mask: int32
            let locale_mask = crate::util::read_i32_le(chunk)?;

            // create_charset_mask: int32
            let create_charset_mask = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(Cfg_CategoriesRow {
                id,
                locale_mask,
                create_charset_mask,
                flags,
                name_lang,
            });
        }

        Ok(Cfg_Categories { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Cfg_Categories) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // locale_mask: int32
            b.write_all(&row.locale_mask.to_le_bytes())?;

            // create_charset_mask: int32
            b.write_all(&row.create_charset_mask.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<i32> for Cfg_Categories {
    type Table = Self;

    fn get(&self, key: &Cfg_CategoriesKey) -> Option<&Cfg_CategoriesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &Cfg_CategoriesKey) -> Option<&mut Cfg_CategoriesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cfg_CategoriesRow {
    pub id: Cfg_CategoriesKey,
    pub locale_mask: i32,
    pub create_charset_mask: i32,
    pub flags: i32,
    pub name_lang: ExtendedLocalizedString,
}

impl DbcRow for Cfg_CategoriesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn cfg_categories() {
        let mut file = File::open("../tbc-dbc/Cfg_Categories.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Cfg_Categories::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Cfg_Categories::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
