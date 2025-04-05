use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::cinematic_sequences::{
    CinematicSequences, CinematicSequencesKey,
};
use crate::vanilla_tables::creature_display_info::{
    CreatureDisplayInfo, CreatureDisplayInfoKey,
};
use crate::vanilla_tables::creature_type::{
    CreatureType, CreatureTypeKey,
};
use crate::vanilla_tables::faction_template::{
    FactionTemplate, FactionTemplateKey,
};
use crate::vanilla_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use crate::vanilla_tables::spell::{
    Spell, SpellKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::{
    CharacterRaceFlags, Language,
};

pub type ChrRacesKey = crate::PrimaryKey<u32, ChrRaces>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrRaces {
    pub rows: Vec<ChrRacesRow>,
}

impl ChrRaces {
    pub const FILENAME: &'static str = "ChrRaces.dbc";
    pub const FIELD_COUNT: usize = 29;
    pub const ROW_SIZE: usize = 116;

    pub fn verify(&self, cinematic_sequences: &CinematicSequences, creature_display_info: &CreatureDisplayInfo, creature_type: &CreatureType, faction_template: &FactionTemplate, sound_entries: &SoundEntries, spell: &Spell) -> Result<(), crate::InvalidForeignKeyError<&ChrRacesRow>> {
        for row in &self.rows {
            if row.faction.id != 0 && faction_template.get(&row.faction).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.faction.id.into()
                ));
            }

            if row.exploration_sound.id != 0 && sound_entries.get(&row.exploration_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.exploration_sound.id.into()
                ));
            }

            if row.male_display.id != 0 && creature_display_info.get(&row.male_display).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.male_display.id.into()
                ));
            }

            if row.female_display.id != 0 && creature_display_info.get(&row.female_display).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.female_display.id.into()
                ));
            }

            if row.creature_type.id != 0 && creature_type.get(&row.creature_type).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.creature_type.id.into()
                ));
            }

            if row.login_effect.id != 0 && spell.get(&row.login_effect).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.login_effect.id.into()
                ));
            }

            if row.res_sickness_spell.id != 0 && spell.get(&row.res_sickness_spell).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.res_sickness_spell.id.into()
                ));
            }

            if row.splash_sound_entry.id != 0 && sound_entries.get(&row.splash_sound_entry).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.splash_sound_entry.id.into()
                ));
            }

            if row.cinematic_sequence.id != 0 && cinematic_sequences.get(&row.cinematic_sequence).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrRaces>(),
                    row,
                    id,
                    row.cinematic_sequence.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for ChrRaces {
    fn into(self) -> VanillaTable {
        VanillaTable::ChrRaces(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ChrRaces {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ChrRacesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ChrRacesRow] { &mut self.rows }

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

            // id: primary_key (ChrRaces) uint32
            let id = ChrRacesKey::new(crate::util::read_u32_le(chunk)?);

            // flags: CharacterRaceFlags
            let flags = CharacterRaceFlags::new(crate::util::read_u32_le(chunk)? as _);

            // faction: foreign_key (FactionTemplate) uint32
            let faction = FactionTemplateKey::new(crate::util::read_u32_le(chunk)?.into());

            // exploration_sound: foreign_key (SoundEntries) uint32
            let exploration_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // male_display: foreign_key (CreatureDisplayInfo) uint32
            let male_display = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?.into());

            // female_display: foreign_key (CreatureDisplayInfo) uint32
            let female_display = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?.into());

            // client_prefix: string_ref
            let client_prefix = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // speed_modifier: float
            let speed_modifier = crate::util::read_f32_le(chunk)?;

            // base_lang: Language
            let base_lang = crate::util::read_u32_le(chunk)?.try_into()?;

            // creature_type: foreign_key (CreatureType) uint32
            let creature_type = CreatureTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // login_effect: foreign_key (Spell) uint32
            let login_effect = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // unknown1: int32
            let unknown1 = crate::util::read_i32_le(chunk)?;

            // res_sickness_spell: foreign_key (Spell) uint32
            let res_sickness_spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // splash_sound_entry: foreign_key (SoundEntries) uint32
            let splash_sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // unknown2: int32
            let unknown2 = crate::util::read_i32_le(chunk)?;

            // client_file_path: string_ref
            let client_file_path = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // cinematic_sequence: foreign_key (CinematicSequences) uint32
            let cinematic_sequence = CinematicSequencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // facial_hair_customisation: string_ref[2]
            let facial_hair_customisation = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // hair_customisation: string_ref
            let hair_customisation = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(ChrRacesRow {
                id,
                flags,
                faction,
                exploration_sound,
                male_display,
                female_display,
                client_prefix,
                speed_modifier,
                base_lang,
                creature_type,
                login_effect,
                unknown1,
                res_sickness_spell,
                splash_sound_entry,
                unknown2,
                client_file_path,
                cinematic_sequence,
                name,
                facial_hair_customisation,
                hair_customisation,
            });
        }

        Ok(ChrRaces { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ChrRaces) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: CharacterRaceFlags
            b.write_all(&(row.flags.as_int() as u32).to_le_bytes())?;

            // faction: foreign_key (FactionTemplate) uint32
            b.write_all(&(row.faction.id as u32).to_le_bytes())?;

            // exploration_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.exploration_sound.id as u32).to_le_bytes())?;

            // male_display: foreign_key (CreatureDisplayInfo) uint32
            b.write_all(&(row.male_display.id as u32).to_le_bytes())?;

            // female_display: foreign_key (CreatureDisplayInfo) uint32
            b.write_all(&(row.female_display.id as u32).to_le_bytes())?;

            // client_prefix: string_ref
            b.write_all(&string_cache.add_string(&row.client_prefix).to_le_bytes())?;

            // speed_modifier: float
            b.write_all(&row.speed_modifier.to_le_bytes())?;

            // base_lang: Language
            b.write_all(&(row.base_lang.as_int() as u32).to_le_bytes())?;

            // creature_type: foreign_key (CreatureType) uint32
            b.write_all(&(row.creature_type.id as u32).to_le_bytes())?;

            // login_effect: foreign_key (Spell) uint32
            b.write_all(&(row.login_effect.id as u32).to_le_bytes())?;

            // unknown1: int32
            b.write_all(&row.unknown1.to_le_bytes())?;

            // res_sickness_spell: foreign_key (Spell) uint32
            b.write_all(&(row.res_sickness_spell.id as u32).to_le_bytes())?;

            // splash_sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.splash_sound_entry.id as u32).to_le_bytes())?;

            // unknown2: int32
            b.write_all(&row.unknown2.to_le_bytes())?;

            // client_file_path: string_ref
            b.write_all(&string_cache.add_string(&row.client_file_path).to_le_bytes())?;

            // cinematic_sequence: foreign_key (CinematicSequences) uint32
            b.write_all(&(row.cinematic_sequence.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // facial_hair_customisation: string_ref[2]
            for i in &row.facial_hair_customisation {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // hair_customisation: string_ref
            b.write_all(&string_cache.add_string(&row.hair_customisation).to_le_bytes())?;

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
impl Indexable<u32> for ChrRaces {
    type Table = Self;

    fn get(&self, key: &ChrRacesKey) -> Option<&ChrRacesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ChrRacesKey) -> Option<&mut ChrRacesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrRacesRow {
    pub id: ChrRacesKey,
    pub flags: CharacterRaceFlags,
    pub faction: FactionTemplateKey,
    pub exploration_sound: SoundEntriesKey,
    pub male_display: CreatureDisplayInfoKey,
    pub female_display: CreatureDisplayInfoKey,
    pub client_prefix: String,
    pub speed_modifier: f32,
    pub base_lang: Language,
    pub creature_type: CreatureTypeKey,
    pub login_effect: SpellKey,
    pub unknown1: i32,
    pub res_sickness_spell: SpellKey,
    pub splash_sound_entry: SoundEntriesKey,
    pub unknown2: i32,
    pub client_file_path: String,
    pub cinematic_sequence: CinematicSequencesKey,
    pub name: LocalizedString,
    pub facial_hair_customisation: [String; 2],
    pub hair_customisation: String,
}

impl DbcRow for ChrRacesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn chr_races() {
        let mut file = File::open("../vanilla-dbc/ChrRaces.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ChrRaces::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ChrRaces::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
