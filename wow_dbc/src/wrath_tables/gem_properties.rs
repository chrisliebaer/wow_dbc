use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::spell_item_enchantment::{
    SpellItemEnchantment, SpellItemEnchantmentKey,
};
use std::io::Write;
use super::WrathTable;

pub type GemPropertiesKey = crate::PrimaryKey<i32, GemProperties>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GemProperties {
    pub rows: Vec<GemPropertiesRow>,
}

impl GemProperties {
    pub const FILENAME: &'static str = "GemProperties.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

    pub fn verify(&self, spell_item_enchantment: &SpellItemEnchantment) -> Result<(), crate::InvalidForeignKeyError<&GemPropertiesRow>> {
        for row in &self.rows {
            if row.enchant_id.id != 0 && spell_item_enchantment.get(&row.enchant_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<GemProperties>(),
                    row,
                    id,
                    row.enchant_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for GemProperties {
    fn into(self) -> WrathTable {
        WrathTable::GemProperties(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GemProperties {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GemPropertiesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GemPropertiesRow] { &mut self.rows }

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

            // id: primary_key (GemProperties) int32
            let id = GemPropertiesKey::new(crate::util::read_i32_le(chunk)?);

            // enchant_id: foreign_key (SpellItemEnchantment) int32
            let enchant_id = SpellItemEnchantmentKey::new(crate::util::read_i32_le(chunk)?.into());

            // maxcount_inv: int32
            let maxcount_inv = crate::util::read_i32_le(chunk)?;

            // maxcount_item: int32
            let maxcount_item = crate::util::read_i32_le(chunk)?;

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;


            rows.push(GemPropertiesRow {
                id,
                enchant_id,
                maxcount_inv,
                maxcount_item,
                ty,
            });
        }

        Ok(GemProperties { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GemProperties) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // enchant_id: foreign_key (SpellItemEnchantment) int32
            b.write_all(&(row.enchant_id.id as i32).to_le_bytes())?;

            // maxcount_inv: int32
            b.write_all(&row.maxcount_inv.to_le_bytes())?;

            // maxcount_item: int32
            b.write_all(&row.maxcount_item.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

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
impl Indexable<i32> for GemProperties {
    type Table = Self;

    fn get(&self, key: &GemPropertiesKey) -> Option<&GemPropertiesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GemPropertiesKey) -> Option<&mut GemPropertiesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GemPropertiesRow {
    pub id: GemPropertiesKey,
    pub enchant_id: SpellItemEnchantmentKey,
    pub maxcount_inv: i32,
    pub maxcount_item: i32,
    pub ty: i32,
}

impl DbcRow for GemPropertiesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn gem_properties() {
        let mut file = File::open("../wrath-dbc/GemProperties.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GemProperties::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GemProperties::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
