use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::faction_template::{
    FactionTemplate, FactionTemplateKey,
};
use std::io::Write;
use super::WrathTable;

pub type SummonPropertiesKey = crate::PrimaryKey<i32, SummonProperties>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SummonProperties {
    pub rows: Vec<SummonPropertiesRow>,
}

impl SummonProperties {
    pub const FILENAME: &'static str = "SummonProperties.dbc";
    pub const FIELD_COUNT: usize = 6;
    pub const ROW_SIZE: usize = 24;

    pub fn verify(&self, faction_template: &FactionTemplate) -> Result<(), crate::InvalidForeignKeyError<&SummonPropertiesRow>> {
        for row in &self.rows {
            if row.faction.id != 0 && faction_template.get(&row.faction).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SummonProperties>(),
                    row,
                    id,
                    row.faction.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for SummonProperties {
    fn into(self) -> WrathTable {
        WrathTable::SummonProperties(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SummonProperties {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SummonPropertiesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SummonPropertiesRow] { &mut self.rows }

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

            // id: primary_key (SummonProperties) int32
            let id = SummonPropertiesKey::new(crate::util::read_i32_le(chunk)?);

            // control: int32
            let control = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (FactionTemplate) int32
            let faction = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?.into());

            // title: int32
            let title = crate::util::read_i32_le(chunk)?;

            // slot: int32
            let slot = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(SummonPropertiesRow {
                id,
                control,
                faction,
                title,
                slot,
                flags,
            });
        }

        Ok(SummonProperties { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SummonProperties) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // control: int32
            b.write_all(&row.control.to_le_bytes())?;

            // faction: foreign_key (FactionTemplate) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // title: int32
            b.write_all(&row.title.to_le_bytes())?;

            // slot: int32
            b.write_all(&row.slot.to_le_bytes())?;

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
impl Indexable<i32> for SummonProperties {
    type Table = Self;

    fn get(&self, key: &SummonPropertiesKey) -> Option<&SummonPropertiesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SummonPropertiesKey) -> Option<&mut SummonPropertiesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SummonPropertiesRow {
    pub id: SummonPropertiesKey,
    pub control: i32,
    pub faction: FactionTemplateKey,
    pub title: i32,
    pub slot: i32,
    pub flags: i32,
}

impl DbcRow for SummonPropertiesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn summon_properties() {
        let mut file = File::open("../wrath-dbc/SummonProperties.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SummonProperties::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SummonProperties::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
