use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::WrathTable;

pub type WeatherKey = crate::PrimaryKey<i32, Weather>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Weather {
    pub rows: Vec<WeatherRow>,
}

impl Weather {
    pub const FILENAME: &'static str = "Weather.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

    pub fn verify(&self, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&WeatherRow>> {
        for row in &self.rows {
            if row.ambience_id.id != 0 && sound_entries.get(&row.ambience_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Weather>(),
                    row,
                    id,
                    row.ambience_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for Weather {
    fn into(self) -> WrathTable {
        WrathTable::Weather(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Weather {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WeatherRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WeatherRow] { &mut self.rows }

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

            // id: primary_key (Weather) int32
            let id = WeatherKey::new(crate::util::read_i32_le(chunk)?);

            // ambience_id: foreign_key (SoundEntries) int32
            let ambience_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // effect_type: int32
            let effect_type = crate::util::read_i32_le(chunk)?;

            // transition_sky_box: float
            let transition_sky_box = crate::util::read_f32_le(chunk)?;

            // effect_color: float[3]
            let effect_color = crate::util::read_array_f32::<3>(chunk)?;

            // effect_texture: string_ref
            let effect_texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(WeatherRow {
                id,
                ambience_id,
                effect_type,
                transition_sky_box,
                effect_color,
                effect_texture,
            });
        }

        Ok(Weather { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Weather) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ambience_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.ambience_id.id as i32).to_le_bytes())?;

            // effect_type: int32
            b.write_all(&row.effect_type.to_le_bytes())?;

            // transition_sky_box: float
            b.write_all(&row.transition_sky_box.to_le_bytes())?;

            // effect_color: float[3]
            for i in row.effect_color {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_texture: string_ref
            b.write_all(&string_cache.add_string(&row.effect_texture).to_le_bytes())?;

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
impl Indexable<i32> for Weather {
    type Table = Self;

    fn get(&self, key: &WeatherKey) -> Option<&WeatherRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WeatherKey) -> Option<&mut WeatherRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeatherRow {
    pub id: WeatherKey,
    pub ambience_id: SoundEntriesKey,
    pub effect_type: i32,
    pub transition_sky_box: f32,
    pub effect_color: [f32; 3],
    pub effect_texture: String,
}

impl DbcRow for WeatherRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn weather() {
        let mut file = File::open("../wrath-dbc/Weather.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Weather::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Weather::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
