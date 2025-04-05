use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type SpellRuneCostKey = crate::PrimaryKey<i32, SpellRuneCost>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellRuneCost {
    pub rows: Vec<SpellRuneCostRow>,
}

impl SpellRuneCost {
    pub const FILENAME: &'static str = "SpellRuneCost.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

}

impl Into<WrathTable> for SpellRuneCost {
    fn into(self) -> WrathTable {
        WrathTable::SpellRuneCost(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellRuneCost {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellRuneCostRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellRuneCostRow] { &mut self.rows }

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

            // id: primary_key (SpellRuneCost) int32
            let id = SpellRuneCostKey::new(crate::util::read_i32_le(chunk)?);

            // blood: int32
            let blood = crate::util::read_i32_le(chunk)?;

            // unholy: int32
            let unholy = crate::util::read_i32_le(chunk)?;

            // frost: int32
            let frost = crate::util::read_i32_le(chunk)?;

            // runic_power: int32
            let runic_power = crate::util::read_i32_le(chunk)?;


            rows.push(SpellRuneCostRow {
                id,
                blood,
                unholy,
                frost,
                runic_power,
            });
        }

        Ok(SpellRuneCost { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellRuneCost) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // blood: int32
            b.write_all(&row.blood.to_le_bytes())?;

            // unholy: int32
            b.write_all(&row.unholy.to_le_bytes())?;

            // frost: int32
            b.write_all(&row.frost.to_le_bytes())?;

            // runic_power: int32
            b.write_all(&row.runic_power.to_le_bytes())?;

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
impl Indexable<i32> for SpellRuneCost {
    type Table = Self;

    fn get(&self, key: &SpellRuneCostKey) -> Option<&SpellRuneCostRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellRuneCostKey) -> Option<&mut SpellRuneCostRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellRuneCostRow {
    pub id: SpellRuneCostKey,
    pub blood: i32,
    pub unholy: i32,
    pub frost: i32,
    pub runic_power: i32,
}

impl DbcRow for SpellRuneCostRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_rune_cost() {
        let mut file = File::open("../wrath-dbc/SpellRuneCost.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellRuneCost::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellRuneCost::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
