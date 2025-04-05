use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::liquid_type::{
    LiquidType, LiquidTypeKey,
};
use crate::vanilla_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::FluidSpeed;

pub type SoundWaterTypeKey = crate::PrimaryKey<u32, SoundWaterType>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundWaterType {
    pub rows: Vec<SoundWaterTypeRow>,
}

impl SoundWaterType {
    pub const FILENAME: &'static str = "SoundWaterType.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, liquid_type: &LiquidType, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&SoundWaterTypeRow>> {
        for row in &self.rows {
            if row.liquid_type.id != 0 && liquid_type.get(&row.liquid_type).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SoundWaterType>(),
                    row,
                    id,
                    row.liquid_type.id.into()
                ));
            }

            if row.sound.id != 0 && sound_entries.get(&row.sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SoundWaterType>(),
                    row,
                    id,
                    row.sound.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for SoundWaterType {
    fn into(self) -> VanillaTable {
        VanillaTable::SoundWaterType(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SoundWaterType {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SoundWaterTypeRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SoundWaterTypeRow] { &mut self.rows }

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

            // id: primary_key (SoundWaterType) uint32
            let id = SoundWaterTypeKey::new(crate::util::read_u32_le(chunk)?);

            // liquid_type: foreign_key (LiquidType) uint32
            let liquid_type = LiquidTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // fluid_speed: FluidSpeed
            let fluid_speed = crate::util::read_i32_le(chunk)?.try_into()?;

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SoundWaterTypeRow {
                id,
                liquid_type,
                fluid_speed,
                sound,
            });
        }

        Ok(SoundWaterType { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SoundWaterType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // liquid_type: foreign_key (LiquidType) uint32
            b.write_all(&(row.liquid_type.id as u32).to_le_bytes())?;

            // fluid_speed: FluidSpeed
            b.write_all(&(row.fluid_speed.as_int() as i32).to_le_bytes())?;

            // sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for SoundWaterType {
    type Table = Self;

    fn get(&self, key: &SoundWaterTypeKey) -> Option<&SoundWaterTypeRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SoundWaterTypeKey) -> Option<&mut SoundWaterTypeRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundWaterTypeRow {
    pub id: SoundWaterTypeKey,
    pub liquid_type: LiquidTypeKey,
    pub fluid_speed: FluidSpeed,
    pub sound: SoundEntriesKey,
}

impl DbcRow for SoundWaterTypeRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_water_type() {
        let mut file = File::open("../vanilla-dbc/SoundWaterType.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundWaterType::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundWaterType::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
