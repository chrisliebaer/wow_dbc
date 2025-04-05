use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::item_extended_cost::{
    ItemExtendedCost, ItemExtendedCostKey,
};
use std::io::Write;
use super::WrathTable;

pub type ItemCondExtCostsKey = crate::PrimaryKey<i32, ItemCondExtCosts>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemCondExtCosts {
    pub rows: Vec<ItemCondExtCostsRow>,
}

impl ItemCondExtCosts {
    pub const FILENAME: &'static str = "ItemCondExtCosts.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, item_extended_cost: &ItemExtendedCost) -> Result<(), crate::InvalidForeignKeyError<&ItemCondExtCostsRow>> {
        for row in &self.rows {
            if row.item_extended_cost_entry.id != 0 && item_extended_cost.get(&row.item_extended_cost_entry).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ItemCondExtCosts>(),
                    row,
                    id,
                    row.item_extended_cost_entry.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for ItemCondExtCosts {
    fn into(self) -> WrathTable {
        WrathTable::ItemCondExtCosts(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ItemCondExtCosts {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ItemCondExtCostsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ItemCondExtCostsRow] { &mut self.rows }

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

            // id: primary_key (ItemCondExtCosts) int32
            let id = ItemCondExtCostsKey::new(crate::util::read_i32_le(chunk)?);

            // cond_extended_cost: int32
            let cond_extended_cost = crate::util::read_i32_le(chunk)?;

            // item_extended_cost_entry: foreign_key (ItemExtendedCost) int32
            let item_extended_cost_entry = ItemExtendedCostKey::new(crate::util::read_i32_le(chunk)?.into());

            // arena_season: int32
            let arena_season = crate::util::read_i32_le(chunk)?;


            rows.push(ItemCondExtCostsRow {
                id,
                cond_extended_cost,
                item_extended_cost_entry,
                arena_season,
            });
        }

        Ok(ItemCondExtCosts { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ItemCondExtCosts) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // cond_extended_cost: int32
            b.write_all(&row.cond_extended_cost.to_le_bytes())?;

            // item_extended_cost_entry: foreign_key (ItemExtendedCost) int32
            b.write_all(&(row.item_extended_cost_entry.id as i32).to_le_bytes())?;

            // arena_season: int32
            b.write_all(&row.arena_season.to_le_bytes())?;

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
impl Indexable<i32> for ItemCondExtCosts {
    type Table = Self;

    fn get(&self, key: &ItemCondExtCostsKey) -> Option<&ItemCondExtCostsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ItemCondExtCostsKey) -> Option<&mut ItemCondExtCostsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemCondExtCostsRow {
    pub id: ItemCondExtCostsKey,
    pub cond_extended_cost: i32,
    pub item_extended_cost_entry: ItemExtendedCostKey,
    pub arena_season: i32,
}

impl DbcRow for ItemCondExtCostsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_cond_ext_costs() {
        let mut file = File::open("../wrath-dbc/ItemCondExtCosts.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemCondExtCosts::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemCondExtCosts::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
