use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::currency_category::{
    CurrencyCategory, CurrencyCategoryKey,
};
use crate::wrath_tables::item::{
    Item, ItemKey,
};
use std::io::Write;
use super::WrathTable;

pub type CurrencyTypesKey = crate::PrimaryKey<i32, CurrencyTypes>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CurrencyTypes {
    pub rows: Vec<CurrencyTypesRow>,
}

impl CurrencyTypes {
    pub const FILENAME: &'static str = "CurrencyTypes.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, currency_category: &CurrencyCategory, item: &Item) -> Result<(), crate::InvalidForeignKeyError<&CurrencyTypesRow>> {
        for row in &self.rows {
            if row.item_id.id != 0 && item.get(&row.item_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CurrencyTypes>(),
                    row,
                    id,
                    row.item_id.id.into()
                ));
            }

            if row.category_id.id != 0 && currency_category.get(&row.category_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CurrencyTypes>(),
                    row,
                    id,
                    row.category_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for CurrencyTypes {
    fn into(self) -> WrathTable {
        WrathTable::CurrencyTypes(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CurrencyTypes {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CurrencyTypesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CurrencyTypesRow] { &mut self.rows }

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

            // id: primary_key (CurrencyTypes) int32
            let id = CurrencyTypesKey::new(crate::util::read_i32_le(chunk)?);

            // item_id: foreign_key (Item) int32
            let item_id = ItemKey::new(crate::util::read_i32_le(chunk)?.into());

            // category_id: foreign_key (CurrencyCategory) int32
            let category_id = CurrencyCategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // bit_index: int32
            let bit_index = crate::util::read_i32_le(chunk)?;


            rows.push(CurrencyTypesRow {
                id,
                item_id,
                category_id,
                bit_index,
            });
        }

        Ok(CurrencyTypes { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CurrencyTypes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_id: foreign_key (Item) int32
            b.write_all(&(row.item_id.id as i32).to_le_bytes())?;

            // category_id: foreign_key (CurrencyCategory) int32
            b.write_all(&(row.category_id.id as i32).to_le_bytes())?;

            // bit_index: int32
            b.write_all(&row.bit_index.to_le_bytes())?;

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
impl Indexable<i32> for CurrencyTypes {
    type Table = Self;

    fn get(&self, key: &CurrencyTypesKey) -> Option<&CurrencyTypesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CurrencyTypesKey) -> Option<&mut CurrencyTypesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CurrencyTypesRow {
    pub id: CurrencyTypesKey,
    pub item_id: ItemKey,
    pub category_id: CurrencyCategoryKey,
    pub bit_index: i32,
}

impl DbcRow for CurrencyTypesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn currency_types() {
        let mut file = File::open("../wrath-dbc/CurrencyTypes.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CurrencyTypes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CurrencyTypes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
