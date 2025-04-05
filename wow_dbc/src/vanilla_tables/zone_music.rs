use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::VanillaTable;

pub type ZoneMusicKey = crate::PrimaryKey<u32, ZoneMusic>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneMusic {
    pub rows: Vec<ZoneMusicRow>,
}

impl ZoneMusic {
    pub const FILENAME: &'static str = "ZoneMusic.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

    pub fn verify(&self, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&ZoneMusicRow>> {
        for row in &self.rows {
            if row.day_sound.id != 0 && sound_entries.get(&row.day_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ZoneMusic>(),
                    row,
                    id,
                    row.day_sound.id.into()
                ));
            }

            if row.night_sound.id != 0 && sound_entries.get(&row.night_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ZoneMusic>(),
                    row,
                    id,
                    row.night_sound.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for ZoneMusic {
    fn into(self) -> VanillaTable {
        VanillaTable::ZoneMusic(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ZoneMusic {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ZoneMusicRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ZoneMusicRow] { &mut self.rows }

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

            // id: primary_key (ZoneMusic) uint32
            let id = ZoneMusicKey::new(crate::util::read_u32_le(chunk)?);

            // set_name: string_ref
            let set_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // silence_interval_min_day: int32
            let silence_interval_min_day = crate::util::read_i32_le(chunk)?;

            // silence_interval_min_night: int32
            let silence_interval_min_night = crate::util::read_i32_le(chunk)?;

            // silence_interval_max_day: int32
            let silence_interval_max_day = crate::util::read_i32_le(chunk)?;

            // silence_interval_max_night: int32
            let silence_interval_max_night = crate::util::read_i32_le(chunk)?;

            // day_sound: foreign_key (SoundEntries) uint32
            let day_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // night_sound: foreign_key (SoundEntries) uint32
            let night_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(ZoneMusicRow {
                id,
                set_name,
                silence_interval_min_day,
                silence_interval_min_night,
                silence_interval_max_day,
                silence_interval_max_night,
                day_sound,
                night_sound,
            });
        }

        Ok(ZoneMusic { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ZoneMusic) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // set_name: string_ref
            b.write_all(&string_cache.add_string(&row.set_name).to_le_bytes())?;

            // silence_interval_min_day: int32
            b.write_all(&row.silence_interval_min_day.to_le_bytes())?;

            // silence_interval_min_night: int32
            b.write_all(&row.silence_interval_min_night.to_le_bytes())?;

            // silence_interval_max_day: int32
            b.write_all(&row.silence_interval_max_day.to_le_bytes())?;

            // silence_interval_max_night: int32
            b.write_all(&row.silence_interval_max_night.to_le_bytes())?;

            // day_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.day_sound.id as u32).to_le_bytes())?;

            // night_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.night_sound.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for ZoneMusic {
    type Table = Self;

    fn get(&self, key: &ZoneMusicKey) -> Option<&ZoneMusicRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ZoneMusicKey) -> Option<&mut ZoneMusicRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneMusicRow {
    pub id: ZoneMusicKey,
    pub set_name: String,
    pub silence_interval_min_day: i32,
    pub silence_interval_min_night: i32,
    pub silence_interval_max_day: i32,
    pub silence_interval_max_night: i32,
    pub day_sound: SoundEntriesKey,
    pub night_sound: SoundEntriesKey,
}

impl DbcRow for ZoneMusicRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn zone_music() {
        let mut file = File::open("../vanilla-dbc/ZoneMusic.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ZoneMusic::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ZoneMusic::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
