use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type SoundProviderPreferencesKey = crate::PrimaryKey<i32, SoundProviderPreferences>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundProviderPreferences {
    pub rows: Vec<SoundProviderPreferencesRow>,
}

impl SoundProviderPreferences {
    pub const FILENAME: &'static str = "SoundProviderPreferences.dbc";
    pub const FIELD_COUNT: usize = 24;
    pub const ROW_SIZE: usize = 96;

}

impl Into<WrathTable> for SoundProviderPreferences {
    fn into(self) -> WrathTable {
        WrathTable::SoundProviderPreferences(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SoundProviderPreferences {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SoundProviderPreferencesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SoundProviderPreferencesRow] { &mut self.rows }

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

            // id: primary_key (SoundProviderPreferences) int32
            let id = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?);

            // description: string_ref
            let description = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // e_a_x_environment_selection: int32
            let e_a_x_environment_selection = crate::util::read_i32_le(chunk)?;

            // e_a_x_decay_time: float
            let e_a_x_decay_time = crate::util::read_f32_le(chunk)?;

            // e_a_x2_environment_size: float
            let e_a_x2_environment_size = crate::util::read_f32_le(chunk)?;

            // e_a_x2_environment_diffusion: float
            let e_a_x2_environment_diffusion = crate::util::read_f32_le(chunk)?;

            // e_a_x2_room: int32
            let e_a_x2_room = crate::util::read_i32_le(chunk)?;

            // e_a_x2_room_h_f: int32
            let e_a_x2_room_h_f = crate::util::read_i32_le(chunk)?;

            // e_a_x2_decay_h_f_ratio: float
            let e_a_x2_decay_h_f_ratio = crate::util::read_f32_le(chunk)?;

            // e_a_x2_reflections: int32
            let e_a_x2_reflections = crate::util::read_i32_le(chunk)?;

            // e_a_x2_reflections_delay: float
            let e_a_x2_reflections_delay = crate::util::read_f32_le(chunk)?;

            // e_a_x2_reverb: int32
            let e_a_x2_reverb = crate::util::read_i32_le(chunk)?;

            // e_a_x2_reverb_delay: float
            let e_a_x2_reverb_delay = crate::util::read_f32_le(chunk)?;

            // e_a_x2_room_rolloff: float
            let e_a_x2_room_rolloff = crate::util::read_f32_le(chunk)?;

            // e_a_x2_air_absorption: float
            let e_a_x2_air_absorption = crate::util::read_f32_le(chunk)?;

            // e_a_x3_room_l_f: int32
            let e_a_x3_room_l_f = crate::util::read_i32_le(chunk)?;

            // e_a_x3_decay_l_f_ratio: float
            let e_a_x3_decay_l_f_ratio = crate::util::read_f32_le(chunk)?;

            // e_a_x3_echo_time: float
            let e_a_x3_echo_time = crate::util::read_f32_le(chunk)?;

            // e_a_x3_echo_depth: float
            let e_a_x3_echo_depth = crate::util::read_f32_le(chunk)?;

            // e_a_x3_modulation_time: float
            let e_a_x3_modulation_time = crate::util::read_f32_le(chunk)?;

            // e_a_x3_modulation_depth: float
            let e_a_x3_modulation_depth = crate::util::read_f32_le(chunk)?;

            // e_a_x3_h_f_reference: float
            let e_a_x3_h_f_reference = crate::util::read_f32_le(chunk)?;

            // e_a_x3_l_f_reference: float
            let e_a_x3_l_f_reference = crate::util::read_f32_le(chunk)?;


            rows.push(SoundProviderPreferencesRow {
                id,
                description,
                flags,
                e_a_x_environment_selection,
                e_a_x_decay_time,
                e_a_x2_environment_size,
                e_a_x2_environment_diffusion,
                e_a_x2_room,
                e_a_x2_room_h_f,
                e_a_x2_decay_h_f_ratio,
                e_a_x2_reflections,
                e_a_x2_reflections_delay,
                e_a_x2_reverb,
                e_a_x2_reverb_delay,
                e_a_x2_room_rolloff,
                e_a_x2_air_absorption,
                e_a_x3_room_l_f,
                e_a_x3_decay_l_f_ratio,
                e_a_x3_echo_time,
                e_a_x3_echo_depth,
                e_a_x3_modulation_time,
                e_a_x3_modulation_depth,
                e_a_x3_h_f_reference,
                e_a_x3_l_f_reference,
            });
        }

        Ok(SoundProviderPreferences { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SoundProviderPreferences) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // description: string_ref
            b.write_all(&string_cache.add_string(&row.description).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // e_a_x_environment_selection: int32
            b.write_all(&row.e_a_x_environment_selection.to_le_bytes())?;

            // e_a_x_decay_time: float
            b.write_all(&row.e_a_x_decay_time.to_le_bytes())?;

            // e_a_x2_environment_size: float
            b.write_all(&row.e_a_x2_environment_size.to_le_bytes())?;

            // e_a_x2_environment_diffusion: float
            b.write_all(&row.e_a_x2_environment_diffusion.to_le_bytes())?;

            // e_a_x2_room: int32
            b.write_all(&row.e_a_x2_room.to_le_bytes())?;

            // e_a_x2_room_h_f: int32
            b.write_all(&row.e_a_x2_room_h_f.to_le_bytes())?;

            // e_a_x2_decay_h_f_ratio: float
            b.write_all(&row.e_a_x2_decay_h_f_ratio.to_le_bytes())?;

            // e_a_x2_reflections: int32
            b.write_all(&row.e_a_x2_reflections.to_le_bytes())?;

            // e_a_x2_reflections_delay: float
            b.write_all(&row.e_a_x2_reflections_delay.to_le_bytes())?;

            // e_a_x2_reverb: int32
            b.write_all(&row.e_a_x2_reverb.to_le_bytes())?;

            // e_a_x2_reverb_delay: float
            b.write_all(&row.e_a_x2_reverb_delay.to_le_bytes())?;

            // e_a_x2_room_rolloff: float
            b.write_all(&row.e_a_x2_room_rolloff.to_le_bytes())?;

            // e_a_x2_air_absorption: float
            b.write_all(&row.e_a_x2_air_absorption.to_le_bytes())?;

            // e_a_x3_room_l_f: int32
            b.write_all(&row.e_a_x3_room_l_f.to_le_bytes())?;

            // e_a_x3_decay_l_f_ratio: float
            b.write_all(&row.e_a_x3_decay_l_f_ratio.to_le_bytes())?;

            // e_a_x3_echo_time: float
            b.write_all(&row.e_a_x3_echo_time.to_le_bytes())?;

            // e_a_x3_echo_depth: float
            b.write_all(&row.e_a_x3_echo_depth.to_le_bytes())?;

            // e_a_x3_modulation_time: float
            b.write_all(&row.e_a_x3_modulation_time.to_le_bytes())?;

            // e_a_x3_modulation_depth: float
            b.write_all(&row.e_a_x3_modulation_depth.to_le_bytes())?;

            // e_a_x3_h_f_reference: float
            b.write_all(&row.e_a_x3_h_f_reference.to_le_bytes())?;

            // e_a_x3_l_f_reference: float
            b.write_all(&row.e_a_x3_l_f_reference.to_le_bytes())?;

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
impl Indexable<i32> for SoundProviderPreferences {
    type Table = Self;

    fn get(&self, key: &SoundProviderPreferencesKey) -> Option<&SoundProviderPreferencesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SoundProviderPreferencesKey) -> Option<&mut SoundProviderPreferencesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundProviderPreferencesRow {
    pub id: SoundProviderPreferencesKey,
    pub description: String,
    pub flags: i32,
    pub e_a_x_environment_selection: i32,
    pub e_a_x_decay_time: f32,
    pub e_a_x2_environment_size: f32,
    pub e_a_x2_environment_diffusion: f32,
    pub e_a_x2_room: i32,
    pub e_a_x2_room_h_f: i32,
    pub e_a_x2_decay_h_f_ratio: f32,
    pub e_a_x2_reflections: i32,
    pub e_a_x2_reflections_delay: f32,
    pub e_a_x2_reverb: i32,
    pub e_a_x2_reverb_delay: f32,
    pub e_a_x2_room_rolloff: f32,
    pub e_a_x2_air_absorption: f32,
    pub e_a_x3_room_l_f: i32,
    pub e_a_x3_decay_l_f_ratio: f32,
    pub e_a_x3_echo_time: f32,
    pub e_a_x3_echo_depth: f32,
    pub e_a_x3_modulation_time: f32,
    pub e_a_x3_modulation_depth: f32,
    pub e_a_x3_h_f_reference: f32,
    pub e_a_x3_l_f_reference: f32,
}

impl DbcRow for SoundProviderPreferencesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_provider_preferences() {
        let mut file = File::open("../wrath-dbc/SoundProviderPreferences.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundProviderPreferences::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundProviderPreferences::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
