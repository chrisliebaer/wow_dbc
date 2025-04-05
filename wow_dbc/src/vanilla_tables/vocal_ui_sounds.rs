use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use crate::vanilla_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::VanillaTable;

pub type VocalUISoundsKey = crate::PrimaryKey<u32, VocalUISounds>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VocalUISounds {
    pub rows: Vec<VocalUISoundsRow>,
}

impl VocalUISounds {
    pub const FILENAME: &'static str = "VocalUISounds.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, chr_races: &ChrRaces, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&VocalUISoundsRow>> {
        for row in &self.rows {
            if row.race.id != 0 && chr_races.get(&row.race).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<VocalUISounds>(),
                    row,
                    id,
                    row.race.id.into()
                ));
            }

            if row.normal_male_sound.id != 0 && sound_entries.get(&row.normal_male_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<VocalUISounds>(),
                    row,
                    id,
                    row.normal_male_sound.id.into()
                ));
            }

            if row.normal_female_sound.id != 0 && sound_entries.get(&row.normal_female_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<VocalUISounds>(),
                    row,
                    id,
                    row.normal_female_sound.id.into()
                ));
            }

            if row.pissed_male_sound.id != 0 && sound_entries.get(&row.pissed_male_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<VocalUISounds>(),
                    row,
                    id,
                    row.pissed_male_sound.id.into()
                ));
            }

            if row.pissed_female_sound.id != 0 && sound_entries.get(&row.pissed_female_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<VocalUISounds>(),
                    row,
                    id,
                    row.pissed_female_sound.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for VocalUISounds {
    fn into(self) -> VanillaTable {
        VanillaTable::VocalUISounds(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for VocalUISounds {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[VocalUISoundsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [VocalUISoundsRow] { &mut self.rows }

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

            // id: primary_key (VocalUISounds) uint32
            let id = VocalUISoundsKey::new(crate::util::read_u32_le(chunk)?);

            // vocal_ui_enum: int32
            let vocal_ui_enum = crate::util::read_i32_le(chunk)?;

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // normal_male_sound: foreign_key (SoundEntries) uint32
            let normal_male_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // normal_female_sound: foreign_key (SoundEntries) uint32
            let normal_female_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            let pissed_male_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            let pissed_female_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(VocalUISoundsRow {
                id,
                vocal_ui_enum,
                race,
                normal_male_sound,
                normal_female_sound,
                pissed_male_sound,
                pissed_female_sound,
            });
        }

        Ok(VocalUISounds { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (VocalUISounds) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vocal_ui_enum: int32
            b.write_all(&row.vocal_ui_enum.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // normal_male_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.normal_male_sound.id as u32).to_le_bytes())?;

            // normal_female_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.normal_female_sound.id as u32).to_le_bytes())?;

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.pissed_male_sound.id as u32).to_le_bytes())?;

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.pissed_female_sound.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for VocalUISounds {
    type Table = Self;

    fn get(&self, key: &VocalUISoundsKey) -> Option<&VocalUISoundsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &VocalUISoundsKey) -> Option<&mut VocalUISoundsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VocalUISoundsRow {
    pub id: VocalUISoundsKey,
    pub vocal_ui_enum: i32,
    pub race: ChrRacesKey,
    pub normal_male_sound: SoundEntriesKey,
    pub normal_female_sound: SoundEntriesKey,
    pub pissed_male_sound: SoundEntriesKey,
    pub pissed_female_sound: SoundEntriesKey,
}

impl DbcRow for VocalUISoundsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn vocal_ui_sounds() {
        let mut file = File::open("../vanilla-dbc/VocalUISounds.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = VocalUISounds::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = VocalUISounds::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
