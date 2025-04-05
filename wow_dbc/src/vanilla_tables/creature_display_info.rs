use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::creature_display_info_extra::{
    CreatureDisplayInfoExtra, CreatureDisplayInfoExtraKey,
};
use crate::vanilla_tables::creature_model_data::{
    CreatureModelData, CreatureModelDataKey,
};
use crate::vanilla_tables::creature_sound_data::{
    CreatureSoundData, CreatureSoundDataKey,
};
use crate::vanilla_tables::npc_sounds::{
    NPCSounds, NPCSoundsKey,
};
use crate::vanilla_tables::unit_blood::{
    UnitBlood, UnitBloodKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::SizeClass;

pub type CreatureDisplayInfoKey = crate::PrimaryKey<u32, CreatureDisplayInfo>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfo {
    pub rows: Vec<CreatureDisplayInfoRow>,
}

impl CreatureDisplayInfo {
    pub const FILENAME: &'static str = "CreatureDisplayInfo.dbc";
    pub const FIELD_COUNT: usize = 12;
    pub const ROW_SIZE: usize = 48;

    pub fn verify(&self, creature_display_info_extra: &CreatureDisplayInfoExtra, creature_model_data: &CreatureModelData, creature_sound_data: &CreatureSoundData, npc_sounds: &NPCSounds, unit_blood: &UnitBlood) -> Result<(), crate::InvalidForeignKeyError<&CreatureDisplayInfoRow>> {
        for row in &self.rows {
            if row.model.id != 0 && creature_model_data.get(&row.model).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.model.id.into()
                ));
            }

            if row.sound.id != 0 && creature_sound_data.get(&row.sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.sound.id.into()
                ));
            }

            if row.extended_display_info.id != 0 && creature_display_info_extra.get(&row.extended_display_info).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.extended_display_info.id.into()
                ));
            }

            if row.blood.id != 0 && unit_blood.get(&row.blood).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.blood.id.into()
                ));
            }

            if row.npc_sound.id != 0 && npc_sounds.get(&row.npc_sound).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.npc_sound.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CreatureDisplayInfo {
    fn into(self) -> VanillaTable {
        VanillaTable::CreatureDisplayInfo(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CreatureDisplayInfo {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CreatureDisplayInfoRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CreatureDisplayInfoRow] { &mut self.rows }

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

            // id: primary_key (CreatureDisplayInfo) uint32
            let id = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?);

            // model: foreign_key (CreatureModelData) uint32
            let model = CreatureModelDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound: foreign_key (CreatureSoundData) uint32
            let sound = CreatureSoundDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // extended_display_info: foreign_key (CreatureDisplayInfoExtra) uint32
            let extended_display_info = CreatureDisplayInfoExtraKey::new(crate::util::read_u32_le(chunk)?.into());

            // creature_model_scale: float
            let creature_model_scale = crate::util::read_f32_le(chunk)?;

            // creature_model_alpha: int32
            let creature_model_alpha = crate::util::read_i32_le(chunk)?;

            // texture_variation: string_ref[3]
            let texture_variation = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // size: SizeClass
            let size = crate::util::read_i32_le(chunk)?.try_into()?;

            // blood: foreign_key (UnitBlood) uint32
            let blood = UnitBloodKey::new(crate::util::read_u32_le(chunk)?.into());

            // npc_sound: foreign_key (NPCSounds) uint32
            let npc_sound = NPCSoundsKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(CreatureDisplayInfoRow {
                id,
                model,
                sound,
                extended_display_info,
                creature_model_scale,
                creature_model_alpha,
                texture_variation,
                size,
                blood,
                npc_sound,
            });
        }

        Ok(CreatureDisplayInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureDisplayInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model: foreign_key (CreatureModelData) uint32
            b.write_all(&(row.model.id as u32).to_le_bytes())?;

            // sound: foreign_key (CreatureSoundData) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

            // extended_display_info: foreign_key (CreatureDisplayInfoExtra) uint32
            b.write_all(&(row.extended_display_info.id as u32).to_le_bytes())?;

            // creature_model_scale: float
            b.write_all(&row.creature_model_scale.to_le_bytes())?;

            // creature_model_alpha: int32
            b.write_all(&row.creature_model_alpha.to_le_bytes())?;

            // texture_variation: string_ref[3]
            for i in &row.texture_variation {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // size: SizeClass
            b.write_all(&(row.size.as_int() as i32).to_le_bytes())?;

            // blood: foreign_key (UnitBlood) uint32
            b.write_all(&(row.blood.id as u32).to_le_bytes())?;

            // npc_sound: foreign_key (NPCSounds) uint32
            b.write_all(&(row.npc_sound.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for CreatureDisplayInfo {
    type Table = Self;

    fn get(&self, key: &CreatureDisplayInfoKey) -> Option<&CreatureDisplayInfoRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CreatureDisplayInfoKey) -> Option<&mut CreatureDisplayInfoRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfoRow {
    pub id: CreatureDisplayInfoKey,
    pub model: CreatureModelDataKey,
    pub sound: CreatureSoundDataKey,
    pub extended_display_info: CreatureDisplayInfoExtraKey,
    pub creature_model_scale: f32,
    pub creature_model_alpha: i32,
    pub texture_variation: [String; 3],
    pub size: SizeClass,
    pub blood: UnitBloodKey,
    pub npc_sound: NPCSoundsKey,
}

impl DbcRow for CreatureDisplayInfoRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_display_info() {
        let mut file = File::open("../vanilla-dbc/CreatureDisplayInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureDisplayInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureDisplayInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
