use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::spell_visual_effect_name::{
    SpellVisualEffectName, SpellVisualEffectNameKey,
};
use crate::wrath_tables::spell_visual_kit::{
    SpellVisualKit, SpellVisualKitKey,
};
use std::io::Write;
use super::WrathTable;

pub type SpellVisualKitModelAttachKey = crate::PrimaryKey<i32, SpellVisualKitModelAttach>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualKitModelAttach {
    pub rows: Vec<SpellVisualKitModelAttachRow>,
}

impl SpellVisualKitModelAttach {
    pub const FILENAME: &'static str = "SpellVisualKitModelAttach.dbc";
    pub const FIELD_COUNT: usize = 10;
    pub const ROW_SIZE: usize = 40;

    pub fn verify(&self, spell_visual_effect_name: &SpellVisualEffectName, spell_visual_kit: &SpellVisualKit) -> Result<(), crate::InvalidForeignKeyError<&SpellVisualKitModelAttachRow>> {
        for row in &self.rows {
            if row.parent_spell_visual_kit_id.id != 0 && spell_visual_kit.get(&row.parent_spell_visual_kit_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisualKitModelAttach>(),
                    row,
                    id,
                    row.parent_spell_visual_kit_id.id.into()
                ));
            }

            if row.spell_visual_effect_name_id.id != 0 && spell_visual_effect_name.get(&row.spell_visual_effect_name_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellVisualKitModelAttach>(),
                    row,
                    id,
                    row.spell_visual_effect_name_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for SpellVisualKitModelAttach {
    fn into(self) -> WrathTable {
        WrathTable::SpellVisualKitModelAttach(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellVisualKitModelAttach {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellVisualKitModelAttachRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellVisualKitModelAttachRow] { &mut self.rows }

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

            // id: primary_key (SpellVisualKitModelAttach) int32
            let id = SpellVisualKitModelAttachKey::new(crate::util::read_i32_le(chunk)?);

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            let parent_spell_visual_kit_id = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            let spell_visual_effect_name_id = SpellVisualEffectNameKey::new(crate::util::read_i32_le(chunk)?.into());

            // attachment_id: int32
            let attachment_id = crate::util::read_i32_le(chunk)?;

            // offset: float[3]
            let offset = crate::util::read_array_f32::<3>(chunk)?;

            // yaw: float
            let yaw = crate::util::read_f32_le(chunk)?;

            // pitch: float
            let pitch = crate::util::read_f32_le(chunk)?;

            // roll: float
            let roll = crate::util::read_f32_le(chunk)?;


            rows.push(SpellVisualKitModelAttachRow {
                id,
                parent_spell_visual_kit_id,
                spell_visual_effect_name_id,
                attachment_id,
                offset,
                yaw,
                pitch,
                roll,
            });
        }

        Ok(SpellVisualKitModelAttach { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellVisualKitModelAttach) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            b.write_all(&(row.parent_spell_visual_kit_id.id as i32).to_le_bytes())?;

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            b.write_all(&(row.spell_visual_effect_name_id.id as i32).to_le_bytes())?;

            // attachment_id: int32
            b.write_all(&row.attachment_id.to_le_bytes())?;

            // offset: float[3]
            for i in row.offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // yaw: float
            b.write_all(&row.yaw.to_le_bytes())?;

            // pitch: float
            b.write_all(&row.pitch.to_le_bytes())?;

            // roll: float
            b.write_all(&row.roll.to_le_bytes())?;

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
impl Indexable<i32> for SpellVisualKitModelAttach {
    type Table = Self;

    fn get(&self, key: &SpellVisualKitModelAttachKey) -> Option<&SpellVisualKitModelAttachRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellVisualKitModelAttachKey) -> Option<&mut SpellVisualKitModelAttachRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualKitModelAttachRow {
    pub id: SpellVisualKitModelAttachKey,
    pub parent_spell_visual_kit_id: SpellVisualKitKey,
    pub spell_visual_effect_name_id: SpellVisualEffectNameKey,
    pub attachment_id: i32,
    pub offset: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl DbcRow for SpellVisualKitModelAttachRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_visual_kit_model_attach() {
        let mut file = File::open("../wrath-dbc/SpellVisualKitModelAttach.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellVisualKitModelAttach::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellVisualKitModelAttach::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
