use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::faction_group::{
    FactionGroup, FactionGroupKey,
};
use crate::vanilla_tables::light::{
    Light, LightKey,
};
use crate::vanilla_tables::liquid_type::{
    LiquidType, LiquidTypeKey,
};
use crate::vanilla_tables::map::{
    Map, MapKey,
};
use crate::vanilla_tables::sound_ambience::{
    SoundAmbience, SoundAmbienceKey,
};
use crate::vanilla_tables::sound_provider_preferences::{
    SoundProviderPreferences, SoundProviderPreferencesKey,
};
use crate::vanilla_tables::zone_intro_music_table::{
    ZoneIntroMusicTable, ZoneIntroMusicTableKey,
};
use crate::vanilla_tables::zone_music::{
    ZoneMusic, ZoneMusicKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::AreaFlags;

pub type AreaTableKey = crate::PrimaryKey<u32, AreaTable>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTable {
    pub rows: Vec<AreaTableRow>,
}

impl AreaTable {
    pub const FILENAME: &'static str = "AreaTable.dbc";
    pub const FIELD_COUNT: usize = 25;
    pub const ROW_SIZE: usize = 100;

    pub fn verify(&self, faction_group: &FactionGroup, light: &Light, liquid_type: &LiquidType, map: &Map, sound_ambience: &SoundAmbience, sound_provider_preferences: &SoundProviderPreferences, zone_intro_music_table: &ZoneIntroMusicTable, zone_music: &ZoneMusic) -> Result<(), crate::InvalidForeignKeyError<&AreaTableRow>> {
        for row in &self.rows {
            if row.map.id != 0 && map.get(&row.map).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.map.id.into()
                ));
            }

            if row.parent_area_table.id != 0 && self.get(&row.parent_area_table).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.parent_area_table.id.into()
                ));
            }

            if row.sound_preferences.id != 0 && sound_provider_preferences.get(&row.sound_preferences).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.sound_preferences.id.into()
                ));
            }

            if row.sound_preferences_underwater.id != 0 && sound_provider_preferences.get(&row.sound_preferences_underwater).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.sound_preferences_underwater.id.into()
                ));
            }

            if row.sound_ambience.id != 0 && sound_ambience.get(&row.sound_ambience).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.sound_ambience.id.into()
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

            if row.zone_music_intro.id != 0 && zone_intro_music_table.get(&row.zone_music_intro).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.zone_music_intro.id.into()
                ));
            }

            if row.faction_group.id != 0 && faction_group.get(&row.faction_group).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.faction_group.id.into()
                ));
            }

            if row.liquid_type.id != 0 && liquid_type.get(&row.liquid_type).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.liquid_type.id.into()
                ));
            }

            if row.light.id != 0 && light.get(&row.light).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaTable>(),
                    row,
                    id,
                    row.light.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for AreaTable {
    fn into(self) -> VanillaTable {
        VanillaTable::AreaTable(self)
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

            // id: primary_key (AreaTable) uint32
            let id = AreaTableKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // parent_area_table: foreign_key (AreaTable) uint32
            let parent_area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_bit: int32
            let area_bit = crate::util::read_i32_le(chunk)?;

            // flags: AreaFlags
            let flags = AreaFlags::new(crate::util::read_i32_le(chunk)? as _);

            // sound_preferences: foreign_key (SoundProviderPreferences) uint32
            let sound_preferences = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_preferences_underwater: foreign_key (SoundProviderPreferences) uint32
            let sound_preferences_underwater = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_ambience: foreign_key (SoundAmbience) uint32
            let sound_ambience = SoundAmbienceKey::new(crate::util::read_u32_le(chunk)?.into());

            // zone_music: foreign_key (ZoneMusic) uint32
            let zone_music = ZoneMusicKey::new(crate::util::read_u32_le(chunk)?.into());

            // zone_music_intro: foreign_key (ZoneIntroMusicTable) uint32
            let zone_music_intro = ZoneIntroMusicTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // exploration_level: int32
            let exploration_level = crate::util::read_i32_le(chunk)?;

            // area_name: string_ref_loc
            let area_name = crate::util::read_localized_string(chunk, &string_block)?;

            // faction_group: foreign_key (FactionGroup) uint32
            let faction_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // liquid_type: foreign_key (LiquidType) uint32
            let liquid_type = LiquidTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // min_elevation: int32
            let min_elevation = crate::util::read_i32_le(chunk)?;

            // ambient_multiplier: float
            let ambient_multiplier = crate::util::read_f32_le(chunk)?;

            // light: foreign_key (Light) uint32
            let light = LightKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(AreaTableRow {
                id,
                map,
                parent_area_table,
                area_bit,
                flags,
                sound_preferences,
                sound_preferences_underwater,
                sound_ambience,
                zone_music,
                zone_music_intro,
                exploration_level,
                area_name,
                faction_group,
                liquid_type,
                min_elevation,
                ambient_multiplier,
                light,
            });
        }

        Ok(AreaTable { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AreaTable) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // parent_area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.parent_area_table.id as u32).to_le_bytes())?;

            // area_bit: int32
            b.write_all(&row.area_bit.to_le_bytes())?;

            // flags: AreaFlags
            b.write_all(&(row.flags.as_int() as i32).to_le_bytes())?;

            // sound_preferences: foreign_key (SoundProviderPreferences) uint32
            b.write_all(&(row.sound_preferences.id as u32).to_le_bytes())?;

            // sound_preferences_underwater: foreign_key (SoundProviderPreferences) uint32
            b.write_all(&(row.sound_preferences_underwater.id as u32).to_le_bytes())?;

            // sound_ambience: foreign_key (SoundAmbience) uint32
            b.write_all(&(row.sound_ambience.id as u32).to_le_bytes())?;

            // zone_music: foreign_key (ZoneMusic) uint32
            b.write_all(&(row.zone_music.id as u32).to_le_bytes())?;

            // zone_music_intro: foreign_key (ZoneIntroMusicTable) uint32
            b.write_all(&(row.zone_music_intro.id as u32).to_le_bytes())?;

            // exploration_level: int32
            b.write_all(&row.exploration_level.to_le_bytes())?;

            // area_name: string_ref_loc
            b.write_all(&row.area_name.string_indices_as_array(&mut string_cache))?;

            // faction_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.faction_group.id as u32).to_le_bytes())?;

            // liquid_type: foreign_key (LiquidType) uint32
            b.write_all(&(row.liquid_type.id as u32).to_le_bytes())?;

            // min_elevation: int32
            b.write_all(&row.min_elevation.to_le_bytes())?;

            // ambient_multiplier: float
            b.write_all(&row.ambient_multiplier.to_le_bytes())?;

            // light: foreign_key (Light) uint32
            b.write_all(&(row.light.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for AreaTable {
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
    pub map: MapKey,
    pub parent_area_table: AreaTableKey,
    pub area_bit: i32,
    pub flags: AreaFlags,
    pub sound_preferences: SoundProviderPreferencesKey,
    pub sound_preferences_underwater: SoundProviderPreferencesKey,
    pub sound_ambience: SoundAmbienceKey,
    pub zone_music: ZoneMusicKey,
    pub zone_music_intro: ZoneIntroMusicTableKey,
    pub exploration_level: i32,
    pub area_name: LocalizedString,
    pub faction_group: FactionGroupKey,
    pub liquid_type: LiquidTypeKey,
    pub min_elevation: i32,
    pub ambient_multiplier: f32,
    pub light: LightKey,
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
        let mut file = File::open("../vanilla-dbc/AreaTable.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaTable::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaTable::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
