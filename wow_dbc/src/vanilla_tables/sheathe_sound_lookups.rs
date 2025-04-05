use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::item_class::{
    ItemClass, ItemClassKey,
};
use crate::vanilla_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::ItemEnvTypes;

pub type SheatheSoundLookupsKey = crate::PrimaryKey<u32, SheatheSoundLookups>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookups {
    pub rows: Vec<SheatheSoundLookupsRow>,
}

impl SheatheSoundLookups {
    pub const FILENAME: &'static str = "SheatheSoundLookups.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, item_class: &ItemClass, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&SheatheSoundLookupsRow>> {
        for row in &self.rows {
            if row.item_class.id != 0 && item_class.get(&row.item_class).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SheatheSoundLookups>(),
                    row,
                    id,
                    row.item_class.id.into()
                ));
            }

            if row.sheathe_sound.id != 0 && sound_entries.get(&row.sheathe_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SheatheSoundLookups>(),
                    row,
                    id,
                    row.sheathe_sound.id.into()
                ));
            }

            if row.draw_sound.id != 0 && sound_entries.get(&row.draw_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SheatheSoundLookups>(),
                    row,
                    id,
                    row.draw_sound.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for SheatheSoundLookups {
    fn into(self) -> VanillaTable {
        VanillaTable::SheatheSoundLookups(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SheatheSoundLookups {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SheatheSoundLookupsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SheatheSoundLookupsRow] { &mut self.rows }

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

            // id: primary_key (SheatheSoundLookups) uint32
            let id = SheatheSoundLookupsKey::new(crate::util::read_u32_le(chunk)?);

            // item_class: foreign_key (ItemClass) uint32
            let item_class = ItemClassKey::new(crate::util::read_u32_le(chunk)?.into());

            // item_subclass: uint32
            let item_subclass = crate::util::read_u32_le(chunk)?;

            // item_env_types: ItemEnvTypes
            let item_env_types = crate::util::read_i32_le(chunk)?.try_into()?;

            // not_shield: bool32
            let not_shield = crate::util::read_u32_le(chunk)? != 0;

            // sheathe_sound: foreign_key (SoundEntries) uint32
            let sheathe_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // draw_sound: foreign_key (SoundEntries) uint32
            let draw_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SheatheSoundLookupsRow {
                id,
                item_class,
                item_subclass,
                item_env_types,
                not_shield,
                sheathe_sound,
                draw_sound,
            });
        }

        Ok(SheatheSoundLookups { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SheatheSoundLookups) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_class: foreign_key (ItemClass) uint32
            b.write_all(&(row.item_class.id as u32).to_le_bytes())?;

            // item_subclass: uint32
            b.write_all(&row.item_subclass.to_le_bytes())?;

            // item_env_types: ItemEnvTypes
            b.write_all(&(row.item_env_types.as_int() as i32).to_le_bytes())?;

            // not_shield: bool32
            b.write_all(&u32::from(row.not_shield).to_le_bytes())?;

            // sheathe_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sheathe_sound.id as u32).to_le_bytes())?;

            // draw_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.draw_sound.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for SheatheSoundLookups {
    type Table = Self;

    fn get(&self, key: &SheatheSoundLookupsKey) -> Option<&SheatheSoundLookupsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SheatheSoundLookupsKey) -> Option<&mut SheatheSoundLookupsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookupsRow {
    pub id: SheatheSoundLookupsKey,
    pub item_class: ItemClassKey,
    pub item_subclass: u32,
    pub item_env_types: ItemEnvTypes,
    pub not_shield: bool,
    pub sheathe_sound: SoundEntriesKey,
    pub draw_sound: SoundEntriesKey,
}

impl DbcRow for SheatheSoundLookupsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sheathe_sound_lookups() {
        let mut file = File::open("../vanilla-dbc/SheatheSoundLookups.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SheatheSoundLookups::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SheatheSoundLookups::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
