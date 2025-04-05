use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::creature_display_info_extra::{
    CreatureDisplayInfoExtra, CreatureDisplayInfoExtraKey,
};
use crate::wrath_tables::creature_model_data::{
    CreatureModelData, CreatureModelDataKey,
};
use crate::wrath_tables::creature_sound_data::{
    CreatureSoundData, CreatureSoundDataKey,
};
use crate::wrath_tables::npc_sounds::{
    NPCSounds, NPCSoundsKey,
};
use crate::wrath_tables::object_effect_package::{
    ObjectEffectPackage, ObjectEffectPackageKey,
};
use crate::wrath_tables::particle_color::{
    ParticleColor, ParticleColorKey,
};
use crate::wrath_tables::unit_blood::{
    UnitBlood, UnitBloodKey,
};
use std::io::Write;
use super::WrathTable;

pub type CreatureDisplayInfoKey = crate::PrimaryKey<i32, CreatureDisplayInfo>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfo {
    pub rows: Vec<CreatureDisplayInfoRow>,
}

impl CreatureDisplayInfo {
    pub const FILENAME: &'static str = "CreatureDisplayInfo.dbc";
    pub const FIELD_COUNT: usize = 16;
    pub const ROW_SIZE: usize = 64;

    pub fn verify(&self, creature_display_info_extra: &CreatureDisplayInfoExtra, creature_model_data: &CreatureModelData, creature_sound_data: &CreatureSoundData, npc_sounds: &NPCSounds, object_effect_package: &ObjectEffectPackage, particle_color: &ParticleColor, unit_blood: &UnitBlood) -> Result<(), crate::InvalidForeignKeyError<&CreatureDisplayInfoRow>> {
        for row in &self.rows {
            if row.model_id.id != 0 && creature_model_data.get(&row.model_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.model_id.id.into()
                ));
            }

            if row.sound_id.id != 0 && creature_sound_data.get(&row.sound_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.sound_id.id.into()
                ));
            }

            if row.extended_display_info_id.id != 0 && creature_display_info_extra.get(&row.extended_display_info_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.extended_display_info_id.id.into()
                ));
            }

            if row.blood_id.id != 0 && unit_blood.get(&row.blood_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.blood_id.id.into()
                ));
            }

            if row.n_p_c_sound_id.id != 0 && npc_sounds.get(&row.n_p_c_sound_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.n_p_c_sound_id.id.into()
                ));
            }

            if row.particle_color_id.id != 0 && particle_color.get(&row.particle_color_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.particle_color_id.id.into()
                ));
            }

            if row.object_effect_package_id.id != 0 && object_effect_package.get(&row.object_effect_package_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfo>(),
                    row,
                    id,
                    row.object_effect_package_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for CreatureDisplayInfo {
    fn into(self) -> WrathTable {
        WrathTable::CreatureDisplayInfo(self)
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

            // id: primary_key (CreatureDisplayInfo) int32
            let id = CreatureDisplayInfoKey::new(crate::util::read_i32_le(chunk)?);

            // model_id: foreign_key (CreatureModelData) int32
            let model_id = CreatureModelDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_id: foreign_key (CreatureSoundData) int32
            let sound_id = CreatureSoundDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // extended_display_info_id: foreign_key (CreatureDisplayInfoExtra) int32
            let extended_display_info_id = CreatureDisplayInfoExtraKey::new(crate::util::read_i32_le(chunk)?.into());

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

            // portrait_texture_name: string_ref
            let portrait_texture_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // size_class: int32
            let size_class = crate::util::read_i32_le(chunk)?;

            // blood_id: foreign_key (UnitBlood) int32
            let blood_id = UnitBloodKey::new(crate::util::read_i32_le(chunk)?.into());

            // n_p_c_sound_id: foreign_key (NPCSounds) int32
            let n_p_c_sound_id = NPCSoundsKey::new(crate::util::read_i32_le(chunk)?.into());

            // particle_color_id: foreign_key (ParticleColor) int32
            let particle_color_id = ParticleColorKey::new(crate::util::read_i32_le(chunk)?.into());

            // creature_geoset_data: int32
            let creature_geoset_data = crate::util::read_i32_le(chunk)?;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            let object_effect_package_id = ObjectEffectPackageKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(CreatureDisplayInfoRow {
                id,
                model_id,
                sound_id,
                extended_display_info_id,
                creature_model_scale,
                creature_model_alpha,
                texture_variation,
                portrait_texture_name,
                size_class,
                blood_id,
                n_p_c_sound_id,
                particle_color_id,
                creature_geoset_data,
                object_effect_package_id,
            });
        }

        Ok(CreatureDisplayInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureDisplayInfo) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_id: foreign_key (CreatureModelData) int32
            b.write_all(&(row.model_id.id as i32).to_le_bytes())?;

            // sound_id: foreign_key (CreatureSoundData) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // extended_display_info_id: foreign_key (CreatureDisplayInfoExtra) int32
            b.write_all(&(row.extended_display_info_id.id as i32).to_le_bytes())?;

            // creature_model_scale: float
            b.write_all(&row.creature_model_scale.to_le_bytes())?;

            // creature_model_alpha: int32
            b.write_all(&row.creature_model_alpha.to_le_bytes())?;

            // texture_variation: string_ref[3]
            for i in &row.texture_variation {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // portrait_texture_name: string_ref
            b.write_all(&string_cache.add_string(&row.portrait_texture_name).to_le_bytes())?;

            // size_class: int32
            b.write_all(&row.size_class.to_le_bytes())?;

            // blood_id: foreign_key (UnitBlood) int32
            b.write_all(&(row.blood_id.id as i32).to_le_bytes())?;

            // n_p_c_sound_id: foreign_key (NPCSounds) int32
            b.write_all(&(row.n_p_c_sound_id.id as i32).to_le_bytes())?;

            // particle_color_id: foreign_key (ParticleColor) int32
            b.write_all(&(row.particle_color_id.id as i32).to_le_bytes())?;

            // creature_geoset_data: int32
            b.write_all(&row.creature_geoset_data.to_le_bytes())?;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            b.write_all(&(row.object_effect_package_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for CreatureDisplayInfo {
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
    pub model_id: CreatureModelDataKey,
    pub sound_id: CreatureSoundDataKey,
    pub extended_display_info_id: CreatureDisplayInfoExtraKey,
    pub creature_model_scale: f32,
    pub creature_model_alpha: i32,
    pub texture_variation: [String; 3],
    pub portrait_texture_name: String,
    pub size_class: i32,
    pub blood_id: UnitBloodKey,
    pub n_p_c_sound_id: NPCSoundsKey,
    pub particle_color_id: ParticleColorKey,
    pub creature_geoset_data: i32,
    pub object_effect_package_id: ObjectEffectPackageKey,
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
        let mut file = File::open("../wrath-dbc/CreatureDisplayInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureDisplayInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureDisplayInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
