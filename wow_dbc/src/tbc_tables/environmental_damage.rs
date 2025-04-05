use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::spell_visual_kit::{
    SpellVisualKit, SpellVisualKitKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type EnvironmentalDamageKey = crate::PrimaryKey<i32, EnvironmentalDamage>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentalDamage {
    pub rows: Vec<EnvironmentalDamageRow>,
}

impl EnvironmentalDamage {
    pub const FILENAME: &'static str = "EnvironmentalDamage.dbc";
    pub const FIELD_COUNT: usize = 3;
    pub const ROW_SIZE: usize = 12;

    pub fn verify(&self, spell_visual_kit: &SpellVisualKit) -> Result<(), crate::InvalidForeignKeyError<&EnvironmentalDamageRow>> {
        for row in &self.rows {
            if row.visualkit_id.id != 0 && spell_visual_kit.get(&row.visualkit_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<EnvironmentalDamage>(),
                    row,
                    id,
                    row.visualkit_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for EnvironmentalDamage {
    fn into(self) -> TbcTable {
        TbcTable::EnvironmentalDamage(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for EnvironmentalDamage {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[EnvironmentalDamageRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [EnvironmentalDamageRow] { &mut self.rows }

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

            // id: primary_key (EnvironmentalDamage) int32
            let id = EnvironmentalDamageKey::new(crate::util::read_i32_le(chunk)?);

            // enum_id: int32
            let enum_id = crate::util::read_i32_le(chunk)?;

            // visualkit_id: foreign_key (SpellVisualKit) int32
            let visualkit_id = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(EnvironmentalDamageRow {
                id,
                enum_id,
                visualkit_id,
            });
        }

        Ok(EnvironmentalDamage { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (EnvironmentalDamage) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // enum_id: int32
            b.write_all(&row.enum_id.to_le_bytes())?;

            // visualkit_id: foreign_key (SpellVisualKit) int32
            b.write_all(&(row.visualkit_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for EnvironmentalDamage {
    type Table = Self;

    fn get(&self, key: &EnvironmentalDamageKey) -> Option<&EnvironmentalDamageRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &EnvironmentalDamageKey) -> Option<&mut EnvironmentalDamageRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentalDamageRow {
    pub id: EnvironmentalDamageKey,
    pub enum_id: i32,
    pub visualkit_id: SpellVisualKitKey,
}

impl DbcRow for EnvironmentalDamageRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn environmental_damage() {
        let mut file = File::open("../tbc-dbc/EnvironmentalDamage.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = EnvironmentalDamage::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = EnvironmentalDamage::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
