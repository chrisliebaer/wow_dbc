use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use crate::tbc_tables::emotes_text::{
    EmotesText, EmotesTextKey,
};
use crate::tbc_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type EmotesTextSoundKey = crate::PrimaryKey<i32, EmotesTextSound>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesTextSound {
    pub rows: Vec<EmotesTextSoundRow>,
}

impl EmotesTextSound {
    pub const FILENAME: &'static str = "EmotesTextSound.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

    pub fn verify(&self, chr_races: &ChrRaces, emotes_text: &EmotesText, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&EmotesTextSoundRow>> {
        for row in &self.rows {
            if row.emotes_text_id.id != 0 && emotes_text.get(&row.emotes_text_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<EmotesTextSound>(),
                    row,
                    id,
                    row.emotes_text_id.id.into()
                ));
            }

            if row.race_id.id != 0 && chr_races.get(&row.race_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<EmotesTextSound>(),
                    row,
                    id,
                    row.race_id.id.into()
                ));
            }

            if row.sound_id.id != 0 && sound_entries.get(&row.sound_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<EmotesTextSound>(),
                    row,
                    id,
                    row.sound_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for EmotesTextSound {
    fn into(self) -> TbcTable {
        TbcTable::EmotesTextSound(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for EmotesTextSound {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[EmotesTextSoundRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [EmotesTextSoundRow] { &mut self.rows }

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

            // id: primary_key (EmotesTextSound) int32
            let id = EmotesTextSoundKey::new(crate::util::read_i32_le(chunk)?);

            // emotes_text_id: foreign_key (EmotesText) int32
            let emotes_text_id = EmotesTextKey::new(crate::util::read_i32_le(chunk)?.into());

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex_id: int32
            let sex_id = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(EmotesTextSoundRow {
                id,
                emotes_text_id,
                race_id,
                sex_id,
                sound_id,
            });
        }

        Ok(EmotesTextSound { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (EmotesTextSound) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // emotes_text_id: foreign_key (EmotesText) int32
            b.write_all(&(row.emotes_text_id.id as i32).to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for EmotesTextSound {
    type Table = Self;

    fn get(&self, key: &EmotesTextSoundKey) -> Option<&EmotesTextSoundRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &EmotesTextSoundKey) -> Option<&mut EmotesTextSoundRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesTextSoundRow {
    pub id: EmotesTextSoundKey,
    pub emotes_text_id: EmotesTextKey,
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub sound_id: SoundEntriesKey,
}

impl DbcRow for EmotesTextSoundRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn emotes_text_sound() {
        let mut file = File::open("../tbc-dbc/EmotesTextSound.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = EmotesTextSound::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = EmotesTextSound::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
