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
use crate::vanilla_tables::spell_visual_kit::{
    SpellVisualKit, SpellVisualKitKey,
};
use std::io::Write;
use super::VanillaTable;

pub type SpellVisualKey = crate::PrimaryKey<u32, SpellVisual>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisual {
    pub rows: Vec<SpellVisualRow>,
}

impl SpellVisual {
    pub const FILENAME: &'static str = "SpellVisual.dbc";
    pub const FIELD_COUNT: usize = 16;
    pub const ROW_SIZE: usize = 64;

    pub fn verify(&self, sound_entries: &SoundEntries, spell_visual_kit: &SpellVisualKit) -> Result<(), crate::InvalidForeignKeyError<&SpellVisualRow>> {
        for row in &self.rows {
            if row.precast_kit.id != 0 && spell_visual_kit.get(&row.precast_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.precast_kit.id.into()
                ));
            }

            if row.cast_kit.id != 0 && spell_visual_kit.get(&row.cast_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.cast_kit.id.into()
                ));
            }

            if row.impact_kit.id != 0 && spell_visual_kit.get(&row.impact_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.impact_kit.id.into()
                ));
            }

            if row.state_kit.id != 0 && spell_visual_kit.get(&row.state_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.state_kit.id.into()
                ));
            }

            if row.state_done_kit.id != 0 && spell_visual_kit.get(&row.state_done_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.state_done_kit.id.into()
                ));
            }

            if row.channel_kit.id != 0 && spell_visual_kit.get(&row.channel_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.channel_kit.id.into()
                ));
            }

            if row.missile_sound.id != 0 && sound_entries.get(&row.missile_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.missile_sound.id.into()
                ));
            }

            if row.anim_event_sound.id != 0 && spell_visual_kit.get(&row.anim_event_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.anim_event_sound.id.into()
                ));
            }

            if row.caster_impact_kit.id != 0 && spell_visual_kit.get(&row.caster_impact_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.caster_impact_kit.id.into()
                ));
            }

            if row.target_impact_kit.id != 0 && spell_visual_kit.get(&row.target_impact_kit).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisual>(),
                    row,
                    id,
                    row.target_impact_kit.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for SpellVisual {
    fn into(self) -> VanillaTable {
        VanillaTable::SpellVisual(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellVisual {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellVisualRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellVisualRow] { &mut self.rows }

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

            // id: primary_key (SpellVisual) uint32
            let id = SpellVisualKey::new(crate::util::read_u32_le(chunk)?);

            // precast_kit: foreign_key (SpellVisualKit) uint32
            let precast_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // cast_kit: foreign_key (SpellVisualKit) uint32
            let cast_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // impact_kit: foreign_key (SpellVisualKit) uint32
            let impact_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // state_kit: foreign_key (SpellVisualKit) uint32
            let state_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // state_done_kit: foreign_key (SpellVisualKit) uint32
            let state_done_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // channel_kit: foreign_key (SpellVisualKit) uint32
            let channel_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // has_missile: uint32
            let has_missile = crate::util::read_u32_le(chunk)?;

            // missile_model: int32
            let missile_model = crate::util::read_i32_le(chunk)?;

            // missile_path_type: int32
            let missile_path_type = crate::util::read_i32_le(chunk)?;

            // missile_destination_attachment: int32
            let missile_destination_attachment = crate::util::read_i32_le(chunk)?;

            // missile_sound: foreign_key (SoundEntries) uint32
            let missile_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // anim_event_sound: foreign_key (SpellVisualKit) uint32
            let anim_event_sound = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // caster_impact_kit: foreign_key (SpellVisualKit) uint32
            let caster_impact_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // target_impact_kit: foreign_key (SpellVisualKit) uint32
            let target_impact_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SpellVisualRow {
                id,
                precast_kit,
                cast_kit,
                impact_kit,
                state_kit,
                state_done_kit,
                channel_kit,
                has_missile,
                missile_model,
                missile_path_type,
                missile_destination_attachment,
                missile_sound,
                anim_event_sound,
                flags,
                caster_impact_kit,
                target_impact_kit,
            });
        }

        Ok(SpellVisual { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellVisual) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // precast_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.precast_kit.id as u32).to_le_bytes())?;

            // cast_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.cast_kit.id as u32).to_le_bytes())?;

            // impact_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.impact_kit.id as u32).to_le_bytes())?;

            // state_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.state_kit.id as u32).to_le_bytes())?;

            // state_done_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.state_done_kit.id as u32).to_le_bytes())?;

            // channel_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.channel_kit.id as u32).to_le_bytes())?;

            // has_missile: uint32
            b.write_all(&row.has_missile.to_le_bytes())?;

            // missile_model: int32
            b.write_all(&row.missile_model.to_le_bytes())?;

            // missile_path_type: int32
            b.write_all(&row.missile_path_type.to_le_bytes())?;

            // missile_destination_attachment: int32
            b.write_all(&row.missile_destination_attachment.to_le_bytes())?;

            // missile_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.missile_sound.id as u32).to_le_bytes())?;

            // anim_event_sound: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.anim_event_sound.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // caster_impact_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.caster_impact_kit.id as u32).to_le_bytes())?;

            // target_impact_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.target_impact_kit.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for SpellVisual {
    type Table = Self;

    fn get(&self, key: &SpellVisualKey) -> Option<&SpellVisualRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellVisualKey) -> Option<&mut SpellVisualRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualRow {
    pub id: SpellVisualKey,
    pub precast_kit: SpellVisualKitKey,
    pub cast_kit: SpellVisualKitKey,
    pub impact_kit: SpellVisualKitKey,
    pub state_kit: SpellVisualKitKey,
    pub state_done_kit: SpellVisualKitKey,
    pub channel_kit: SpellVisualKitKey,
    pub has_missile: u32,
    pub missile_model: i32,
    pub missile_path_type: i32,
    pub missile_destination_attachment: i32,
    pub missile_sound: SoundEntriesKey,
    pub anim_event_sound: SpellVisualKitKey,
    pub flags: i32,
    pub caster_impact_kit: SpellVisualKitKey,
    pub target_impact_kit: SpellVisualKitKey,
}

impl DbcRow for SpellVisualRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_visual() {
        let mut file = File::open("../vanilla-dbc/SpellVisual.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellVisual::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellVisual::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
