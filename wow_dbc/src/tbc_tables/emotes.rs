use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::animation_data::{
    AnimationData, AnimationDataKey,
};
use crate::tbc_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type EmotesKey = crate::PrimaryKey<i32, Emotes>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Emotes {
    pub rows: Vec<EmotesRow>,
}

impl Emotes {
    pub const FILENAME: &'static str = "Emotes.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, animation_data: &AnimationData, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&EmotesRow>> {
        for row in &self.rows {
            if row.anim_id.id != 0 && animation_data.get(&row.anim_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Emotes>(),
                    row,
                    id,
                    row.anim_id.id.into()
                ));
            }

            if row.event_sound_id.id != 0 && sound_entries.get(&row.event_sound_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Emotes>(),
                    row,
                    id,
                    row.event_sound_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for Emotes {
    fn into(self) -> TbcTable {
        TbcTable::Emotes(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Emotes {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[EmotesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [EmotesRow] { &mut self.rows }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Emotes) int32
            let id = EmotesKey::new(crate::util::read_i32_le(chunk)?);

            // emote_slash_command: string_ref
            let emote_slash_command = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // anim_id: foreign_key (AnimationData) int32
            let anim_id = AnimationDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // emote_flags: int32
            let emote_flags = crate::util::read_i32_le(chunk)?;

            // emote_spec_proc: int32
            let emote_spec_proc = crate::util::read_i32_le(chunk)?;

            // emote_spec_proc_param: int32
            let emote_spec_proc_param = crate::util::read_i32_le(chunk)?;

            // event_sound_id: foreign_key (SoundEntries) int32
            let event_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(EmotesRow {
                id,
                emote_slash_command,
                anim_id,
                emote_flags,
                emote_spec_proc,
                emote_spec_proc_param,
                event_sound_id,
            });
        }

        Ok(Emotes { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Emotes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // emote_slash_command: string_ref
            b.write_all(&string_cache.add_string(&row.emote_slash_command).to_le_bytes())?;

            // anim_id: foreign_key (AnimationData) int32
            b.write_all(&(row.anim_id.id as i32).to_le_bytes())?;

            // emote_flags: int32
            b.write_all(&row.emote_flags.to_le_bytes())?;

            // emote_spec_proc: int32
            b.write_all(&row.emote_spec_proc.to_le_bytes())?;

            // emote_spec_proc_param: int32
            b.write_all(&row.emote_spec_proc_param.to_le_bytes())?;

            // event_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.event_sound_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for Emotes {
    type Table = Self;

    fn get(&self, key: &EmotesKey) -> Option<&EmotesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &EmotesKey) -> Option<&mut EmotesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesRow {
    pub id: EmotesKey,
    pub emote_slash_command: String,
    pub anim_id: AnimationDataKey,
    pub emote_flags: i32,
    pub emote_spec_proc: i32,
    pub emote_spec_proc_param: i32,
    pub event_sound_id: SoundEntriesKey,
}

impl DbcRow for EmotesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn emotes() {
        let mut file = File::open("../tbc-dbc/Emotes.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Emotes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Emotes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
