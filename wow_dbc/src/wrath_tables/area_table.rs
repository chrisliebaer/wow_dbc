use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::light::{
    Light, LightKey,
};
use crate::wrath_tables::map::{
    Map, MapKey,
};
use crate::wrath_tables::sound_ambience::{
    SoundAmbience, SoundAmbienceKey,
};
use crate::wrath_tables::sound_provider_preferences::{
    SoundProviderPreferences, SoundProviderPreferencesKey,
};
use crate::wrath_tables::zone_intro_music_table::{
    ZoneIntroMusicTable, ZoneIntroMusicTableKey,
};
use crate::wrath_tables::zone_music::{
    ZoneMusic, ZoneMusicKey,
};
use std::io::Write;
use super::WrathTable;

pub type AreaTableKey = crate::PrimaryKey<i32, AreaTable>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTable {
    pub rows: Vec<AreaTableRow>,
}

impl AreaTable {
    pub const FILENAME: &'static str = "AreaTable.dbc";
    pub const FIELD_COUNT: usize = 36;
    pub const ROW_SIZE: usize = 144;

    pub fn verify(&self, light: &Light, map: &Map, sound_ambience: &SoundAmbience, sound_provider_preferences: &SoundProviderPreferences, zone_intro_music_table: &ZoneIntroMusicTable, zone_music: &ZoneMusic) -> Result<(), crate::InvalidForeignKeyError<&AreaTableRow>> {
        for row in &self.rows {
            if row.continent_id.id != 0 && map.get(&row.continent_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.continent_id.id.into()
                ));
            }

            if row.parent_area_id.id != 0 && self.get(&row.parent_area_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.parent_area_id.id.into()
                ));
            }

            if row.sound_provider_pref.id != 0 && sound_provider_preferences.get(&row.sound_provider_pref).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.sound_provider_pref.id.into()
                ));
            }

            if row.sound_provider_pref_underwater.id != 0 && sound_provider_preferences.get(&row.sound_provider_pref_underwater).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.sound_provider_pref_underwater.id.into()
                ));
            }

            if row.ambience_id.id != 0 && sound_ambience.get(&row.ambience_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.ambience_id.id.into()
                ));
            }

            if row.zone_music.id != 0 && zone_music.get(&row.zone_music).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.zone_music.id.into()
                ));
            }

            if row.intro_sound.id != 0 && zone_intro_music_table.get(&row.intro_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.intro_sound.id.into()
                ));
            }

            if row.light_id.id != 0 && light.get(&row.light_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.light_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for AreaTable {
    fn into(self) -> WrathTable {
        WrathTable::AreaTable(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AreaTable {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AreaTableRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AreaTableRow] { &mut self.rows }

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

            // id: primary_key (AreaTable) int32
            let id = AreaTableKey::new(crate::util::read_i32_le(chunk)?);

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // parent_area_id: foreign_key (AreaTable) int32
            let parent_area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_bit: int32
            let area_bit = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // sound_provider_pref: foreign_key (SoundProviderPreferences) int32
            let sound_provider_pref = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_provider_pref_underwater: foreign_key (SoundProviderPreferences) int32
            let sound_provider_pref_underwater = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // ambience_id: foreign_key (SoundAmbience) int32
            let ambience_id = SoundAmbienceKey::new(crate::util::read_i32_le(chunk)?.into());

            // zone_music: foreign_key (ZoneMusic) int32
            let zone_music = ZoneMusicKey::new(crate::util::read_i32_le(chunk)?.into());

            // intro_sound: foreign_key (ZoneIntroMusicTable) int32
            let intro_sound = ZoneIntroMusicTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // exploration_level: int32
            let exploration_level = crate::util::read_i32_le(chunk)?;

            // area_name_lang: string_ref_loc (Extended)
            let area_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // faction_group_mask: int32
            let faction_group_mask = crate::util::read_i32_le(chunk)?;

            // liquid_type_id: int32[4]
            let liquid_type_id = crate::util::read_array_i32::<4>(chunk)?;

            // min_elevation: float
            let min_elevation = crate::util::read_f32_le(chunk)?;

            // ambient_multiplier: float
            let ambient_multiplier = crate::util::read_f32_le(chunk)?;

            // light_id: foreign_key (Light) int32
            let light_id = LightKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(AreaTableRow {
                id,
                continent_id,
                parent_area_id,
                area_bit,
                flags,
                sound_provider_pref,
                sound_provider_pref_underwater,
                ambience_id,
                zone_music,
                intro_sound,
                exploration_level,
                area_name_lang,
                faction_group_mask,
                liquid_type_id,
                min_elevation,
                ambient_multiplier,
                light_id,
            });
        }

        Ok(AreaTable { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AreaTable) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // parent_area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.parent_area_id.id as i32).to_le_bytes())?;

            // area_bit: int32
            b.write_all(&row.area_bit.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // sound_provider_pref: foreign_key (SoundProviderPreferences) int32
            b.write_all(&(row.sound_provider_pref.id as i32).to_le_bytes())?;

            // sound_provider_pref_underwater: foreign_key (SoundProviderPreferences) int32
            b.write_all(&(row.sound_provider_pref_underwater.id as i32).to_le_bytes())?;

            // ambience_id: foreign_key (SoundAmbience) int32
            b.write_all(&(row.ambience_id.id as i32).to_le_bytes())?;

            // zone_music: foreign_key (ZoneMusic) int32
            b.write_all(&(row.zone_music.id as i32).to_le_bytes())?;

            // intro_sound: foreign_key (ZoneIntroMusicTable) int32
            b.write_all(&(row.intro_sound.id as i32).to_le_bytes())?;

            // exploration_level: int32
            b.write_all(&row.exploration_level.to_le_bytes())?;

            // area_name_lang: string_ref_loc (Extended)
            b.write_all(&row.area_name_lang.string_indices_as_array(&mut string_cache))?;

            // faction_group_mask: int32
            b.write_all(&row.faction_group_mask.to_le_bytes())?;

            // liquid_type_id: int32[4]
            for i in row.liquid_type_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // min_elevation: float
            b.write_all(&row.min_elevation.to_le_bytes())?;

            // ambient_multiplier: float
            b.write_all(&row.ambient_multiplier.to_le_bytes())?;

            // light_id: foreign_key (Light) int32
            b.write_all(&(row.light_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for AreaTable {
    type Table = Self;

    fn get(&self, key: &AreaTableKey) -> Option<&AreaTableRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AreaTableKey) -> Option<&mut AreaTableRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTableRow {
    pub id: AreaTableKey,
    pub continent_id: MapKey,
    pub parent_area_id: AreaTableKey,
    pub area_bit: i32,
    pub flags: i32,
    pub sound_provider_pref: SoundProviderPreferencesKey,
    pub sound_provider_pref_underwater: SoundProviderPreferencesKey,
    pub ambience_id: SoundAmbienceKey,
    pub zone_music: ZoneMusicKey,
    pub intro_sound: ZoneIntroMusicTableKey,
    pub exploration_level: i32,
    pub area_name_lang: ExtendedLocalizedString,
    pub faction_group_mask: i32,
    pub liquid_type_id: [i32; 4],
    pub min_elevation: f32,
    pub ambient_multiplier: f32,
    pub light_id: LightKey,
}

impl DbcRow for AreaTableRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn area_table() {
        let mut file = File::open("../wrath-dbc/AreaTable.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaTable::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaTable::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
