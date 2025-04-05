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

pub type Achievement_CategoryKey = crate::PrimaryKey<i32, Achievement_Category>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement_Category {
    pub rows: Vec<Achievement_CategoryRow>,
}

impl Achievement_Category {
    pub const FILENAME: &'static str = "Achievement_Category.dbc";
    pub const FIELD_COUNT: usize = 20;
    pub const ROW_SIZE: usize = 80;

    pub fn verify(&self, ) -> Result<(), crate::InvalidForeignKeyError<&Achievement_CategoryRow>> {
        for row in &self.rows {
            if row.parent.id != 0 && self.get(&row.parent).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement_Category>(),
                    row,
                    id,
                    row.parent.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for Achievement_Category {
    fn into(self) -> WrathTable {
        WrathTable::Achievement_Category(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Achievement_Category {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[Achievement_CategoryRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Achievement_CategoryRow] { &mut self.rows }

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

            // id: primary_key (Achievement_Category) int32
            let id = Achievement_CategoryKey::new(crate::util::read_i32_le(chunk)?);

            // parent: foreign_key (Achievement_Category) int32
            let parent = Achievement_CategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // ui_order: int32
            let ui_order = crate::util::read_i32_le(chunk)?;


            rows.push(Achievement_CategoryRow {
                id,
                parent,
                name_lang,
                ui_order,
            });
        }

        Ok(Achievement_Category { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Achievement_Category) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // parent: foreign_key (Achievement_Category) int32
            b.write_all(&(row.parent.id as i32).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // ui_order: int32
            b.write_all(&row.ui_order.to_le_bytes())?;

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
impl Indexable<i32> for Achievement_Category {
    type Table = Self;

    fn get(&self, key: &Achievement_CategoryKey) -> Option<&Achievement_CategoryRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &Achievement_CategoryKey) -> Option<&mut Achievement_CategoryRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement_CategoryRow {
    pub id: Achievement_CategoryKey,
    pub parent: Achievement_CategoryKey,
    pub name_lang: ExtendedLocalizedString,
    pub ui_order: i32,
}

impl DbcRow for Achievement_CategoryRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn achievement_category() {
        let mut file = File::open("../wrath-dbc/Achievement_Category.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Achievement_Category::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Achievement_Category::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
