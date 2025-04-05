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

pub type CreatureSoundDataKey = crate::PrimaryKey<u32, CreatureSoundData>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureSoundData {
    pub rows: Vec<CreatureSoundDataRow>,
}

impl CreatureSoundData {
    pub const FILENAME: &'static str = "CreatureSoundData.dbc";
    pub const FIELD_COUNT: usize = 30;
    pub const ROW_SIZE: usize = 120;

    pub fn verify(&self, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&CreatureSoundDataRow>> {
        for row in &self.rows {
            if row.sound_exertion.id != 0 && sound_entries.get(&row.sound_exertion).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_exertion.id.into()
                ));
            }

            if row.sound_exertion_critical.id != 0 && sound_entries.get(&row.sound_exertion_critical).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_exertion_critical.id.into()
                ));
            }

            if row.sound_injury.id != 0 && sound_entries.get(&row.sound_injury).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_injury.id.into()
                ));
            }

            if row.sound_injury_critical.id != 0 && sound_entries.get(&row.sound_injury_critical).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_injury_critical.id.into()
                ));
            }

            if row.sound_injury_crushing_blow.id != 0 && sound_entries.get(&row.sound_injury_crushing_blow).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_injury_crushing_blow.id.into()
                ));
            }

            if row.sound_death.id != 0 && sound_entries.get(&row.sound_death).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_death.id.into()
                ));
            }

            if row.sound_stun.id != 0 && sound_entries.get(&row.sound_stun).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_stun.id.into()
                ));
            }

            if row.sound_stand.id != 0 && sound_entries.get(&row.sound_stand).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_stand.id.into()
                ));
            }

            if row.sound_footstep.id != 0 && sound_entries.get(&row.sound_footstep).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_footstep.id.into()
                ));
            }

            if row.sound_aggro.id != 0 && sound_entries.get(&row.sound_aggro).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_aggro.id.into()
                ));
            }

            if row.sound_wing_flap.id != 0 && sound_entries.get(&row.sound_wing_flap).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_wing_flap.id.into()
                ));
            }

            if row.sound_wing_glide.id != 0 && sound_entries.get(&row.sound_wing_glide).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_wing_glide.id.into()
                ));
            }

            if row.sound_alert.id != 0 && sound_entries.get(&row.sound_alert).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_alert.id.into()
                ));
            }

            if row.sound_fidget.id != 0 && sound_entries.get(&row.sound_fidget).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_fidget.id.into()
                ));
            }

            if row.npc_sound.id != 0 && sound_entries.get(&row.npc_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.npc_sound.id.into()
                ));
            }

            if row.loop_sound.id != 0 && sound_entries.get(&row.loop_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.loop_sound.id.into()
                ));
            }

            if row.sound_jump_start.id != 0 && sound_entries.get(&row.sound_jump_start).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_jump_start.id.into()
                ));
            }

            if row.sound_jump_end.id != 0 && sound_entries.get(&row.sound_jump_end).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_jump_end.id.into()
                ));
            }

            if row.sound_pet_attack.id != 0 && sound_entries.get(&row.sound_pet_attack).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_pet_attack.id.into()
                ));
            }

            if row.sound_pet_order.id != 0 && sound_entries.get(&row.sound_pet_order).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_pet_order.id.into()
                ));
            }

            if row.sound_pet_dismiss.id != 0 && sound_entries.get(&row.sound_pet_dismiss).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.sound_pet_dismiss.id.into()
                ));
            }

            if row.birth_sound.id != 0 && sound_entries.get(&row.birth_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.birth_sound.id.into()
                ));
            }

            if row.spell_cast_directed_sound.id != 0 && sound_entries.get(&row.spell_cast_directed_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.spell_cast_directed_sound.id.into()
                ));
            }

            if row.submerge_sound.id != 0 && sound_entries.get(&row.submerge_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.submerge_sound.id.into()
                ));
            }

            if row.submerged_sound.id != 0 && sound_entries.get(&row.submerged_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureSoundData>(),
                    row,
                    id,
                    row.submerged_sound.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CreatureSoundData {
    fn into(self) -> VanillaTable {
        VanillaTable::CreatureSoundData(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CreatureSoundData {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CreatureSoundDataRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CreatureSoundDataRow] { &mut self.rows }

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

            // id: primary_key (CreatureSoundData) uint32
            let id = CreatureSoundDataKey::new(crate::util::read_u32_le(chunk)?);

            // sound_exertion: foreign_key (SoundEntries) uint32
            let sound_exertion = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_exertion_critical: foreign_key (SoundEntries) uint32
            let sound_exertion_critical = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_injury: foreign_key (SoundEntries) uint32
            let sound_injury = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_injury_critical: foreign_key (SoundEntries) uint32
            let sound_injury_critical = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_injury_crushing_blow: foreign_key (SoundEntries) uint32
            let sound_injury_crushing_blow = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_death: foreign_key (SoundEntries) uint32
            let sound_death = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_stun: foreign_key (SoundEntries) uint32
            let sound_stun = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_stand: foreign_key (SoundEntries) uint32
            let sound_stand = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_footstep: foreign_key (SoundEntries) uint32
            let sound_footstep = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_aggro: foreign_key (SoundEntries) uint32
            let sound_aggro = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_wing_flap: foreign_key (SoundEntries) uint32
            let sound_wing_flap = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_wing_glide: foreign_key (SoundEntries) uint32
            let sound_wing_glide = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_alert: foreign_key (SoundEntries) uint32
            let sound_alert = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_fidget: foreign_key (SoundEntries) uint32
            let sound_fidget = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // custom_attack: uint32
            let custom_attack = crate::util::read_u32_le(chunk)?;

            // npc_sound: foreign_key (SoundEntries) uint32
            let npc_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // loop_sound: foreign_key (SoundEntries) uint32
            let loop_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // creature_impact_type: int32
            let creature_impact_type = crate::util::read_i32_le(chunk)?;

            // sound_jump_start: foreign_key (SoundEntries) uint32
            let sound_jump_start = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_jump_end: foreign_key (SoundEntries) uint32
            let sound_jump_end = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_pet_attack: foreign_key (SoundEntries) uint32
            let sound_pet_attack = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_pet_order: foreign_key (SoundEntries) uint32
            let sound_pet_order = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_pet_dismiss: foreign_key (SoundEntries) uint32
            let sound_pet_dismiss = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // fidget_delay_seconds_min: int32
            let fidget_delay_seconds_min = crate::util::read_i32_le(chunk)?;

            // fidget_delay_seconds_max: int32
            let fidget_delay_seconds_max = crate::util::read_i32_le(chunk)?;

            // birth_sound: foreign_key (SoundEntries) uint32
            let birth_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // spell_cast_directed_sound: foreign_key (SoundEntries) uint32
            let spell_cast_directed_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // submerge_sound: foreign_key (SoundEntries) uint32
            let submerge_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // submerged_sound: foreign_key (SoundEntries) uint32
            let submerged_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(CreatureSoundDataRow {
                id,
                sound_exertion,
                sound_exertion_critical,
                sound_injury,
                sound_injury_critical,
                sound_injury_crushing_blow,
                sound_death,
                sound_stun,
                sound_stand,
                sound_footstep,
                sound_aggro,
                sound_wing_flap,
                sound_wing_glide,
                sound_alert,
                sound_fidget,
                custom_attack,
                npc_sound,
                loop_sound,
                creature_impact_type,
                sound_jump_start,
                sound_jump_end,
                sound_pet_attack,
                sound_pet_order,
                sound_pet_dismiss,
                fidget_delay_seconds_min,
                fidget_delay_seconds_max,
                birth_sound,
                spell_cast_directed_sound,
                submerge_sound,
                submerged_sound,
            });
        }

        Ok(CreatureSoundData { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureSoundData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_exertion: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_exertion.id as u32).to_le_bytes())?;

            // sound_exertion_critical: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_exertion_critical.id as u32).to_le_bytes())?;

            // sound_injury: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_injury.id as u32).to_le_bytes())?;

            // sound_injury_critical: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_injury_critical.id as u32).to_le_bytes())?;

            // sound_injury_crushing_blow: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_injury_crushing_blow.id as u32).to_le_bytes())?;

            // sound_death: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_death.id as u32).to_le_bytes())?;

            // sound_stun: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_stun.id as u32).to_le_bytes())?;

            // sound_stand: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_stand.id as u32).to_le_bytes())?;

            // sound_footstep: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_footstep.id as u32).to_le_bytes())?;

            // sound_aggro: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_aggro.id as u32).to_le_bytes())?;

            // sound_wing_flap: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_wing_flap.id as u32).to_le_bytes())?;

            // sound_wing_glide: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_wing_glide.id as u32).to_le_bytes())?;

            // sound_alert: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_alert.id as u32).to_le_bytes())?;

            // sound_fidget: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_fidget.id as u32).to_le_bytes())?;

            // custom_attack: uint32
            b.write_all(&row.custom_attack.to_le_bytes())?;

            // npc_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.npc_sound.id as u32).to_le_bytes())?;

            // loop_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.loop_sound.id as u32).to_le_bytes())?;

            // creature_impact_type: int32
            b.write_all(&row.creature_impact_type.to_le_bytes())?;

            // sound_jump_start: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_jump_start.id as u32).to_le_bytes())?;

            // sound_jump_end: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_jump_end.id as u32).to_le_bytes())?;

            // sound_pet_attack: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_pet_attack.id as u32).to_le_bytes())?;

            // sound_pet_order: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_pet_order.id as u32).to_le_bytes())?;

            // sound_pet_dismiss: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_pet_dismiss.id as u32).to_le_bytes())?;

            // fidget_delay_seconds_min: int32
            b.write_all(&row.fidget_delay_seconds_min.to_le_bytes())?;

            // fidget_delay_seconds_max: int32
            b.write_all(&row.fidget_delay_seconds_max.to_le_bytes())?;

            // birth_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.birth_sound.id as u32).to_le_bytes())?;

            // spell_cast_directed_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.spell_cast_directed_sound.id as u32).to_le_bytes())?;

            // submerge_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.submerge_sound.id as u32).to_le_bytes())?;

            // submerged_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.submerged_sound.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for CreatureSoundData {
    type Table = Self;

    fn get(&self, key: &CreatureSoundDataKey) -> Option<&CreatureSoundDataRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CreatureSoundDataKey) -> Option<&mut CreatureSoundDataRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureSoundDataRow {
    pub id: CreatureSoundDataKey,
    pub sound_exertion: SoundEntriesKey,
    pub sound_exertion_critical: SoundEntriesKey,
    pub sound_injury: SoundEntriesKey,
    pub sound_injury_critical: SoundEntriesKey,
    pub sound_injury_crushing_blow: SoundEntriesKey,
    pub sound_death: SoundEntriesKey,
    pub sound_stun: SoundEntriesKey,
    pub sound_stand: SoundEntriesKey,
    pub sound_footstep: SoundEntriesKey,
    pub sound_aggro: SoundEntriesKey,
    pub sound_wing_flap: SoundEntriesKey,
    pub sound_wing_glide: SoundEntriesKey,
    pub sound_alert: SoundEntriesKey,
    pub sound_fidget: SoundEntriesKey,
    pub custom_attack: u32,
    pub npc_sound: SoundEntriesKey,
    pub loop_sound: SoundEntriesKey,
    pub creature_impact_type: i32,
    pub sound_jump_start: SoundEntriesKey,
    pub sound_jump_end: SoundEntriesKey,
    pub sound_pet_attack: SoundEntriesKey,
    pub sound_pet_order: SoundEntriesKey,
    pub sound_pet_dismiss: SoundEntriesKey,
    pub fidget_delay_seconds_min: i32,
    pub fidget_delay_seconds_max: i32,
    pub birth_sound: SoundEntriesKey,
    pub spell_cast_directed_sound: SoundEntriesKey,
    pub submerge_sound: SoundEntriesKey,
    pub submerged_sound: SoundEntriesKey,
}

impl DbcRow for CreatureSoundDataRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_sound_data() {
        let mut file = File::open("../vanilla-dbc/CreatureSoundData.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureSoundData::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureSoundData::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
