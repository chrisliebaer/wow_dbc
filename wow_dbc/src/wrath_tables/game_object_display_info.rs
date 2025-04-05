use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::object_effect_package::{
    ObjectEffectPackage, ObjectEffectPackageKey,
};
use std::io::Write;
use super::WrathTable;

pub type GameObjectDisplayInfoKey = crate::PrimaryKey<i32, GameObjectDisplayInfo>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameObjectDisplayInfo {
    pub rows: Vec<GameObjectDisplayInfoRow>,
}

impl GameObjectDisplayInfo {
    pub const FILENAME: &'static str = "GameObjectDisplayInfo.dbc";
    pub const FIELD_COUNT: usize = 19;
    pub const ROW_SIZE: usize = 76;

    pub fn verify(&self, object_effect_package: &ObjectEffectPackage) -> Result<(), crate::InvalidForeignKeyError<&GameObjectDisplayInfoRow>> {
        for row in &self.rows {
            if row.object_effect_package_id.id != 0 && object_effect_package.get(&row.object_effect_package_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<GameObjectDisplayInfo>(),
                    row,
                    id,
                    row.object_effect_package_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for GameObjectDisplayInfo {
    fn into(self) -> WrathTable {
        WrathTable::GameObjectDisplayInfo(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GameObjectDisplayInfo {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GameObjectDisplayInfoRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GameObjectDisplayInfoRow] { &mut self.rows }

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

            // id: primary_key (GameObjectDisplayInfo) int32
            let id = GameObjectDisplayInfoKey::new(crate::util::read_i32_le(chunk)?);

            // model_name: string_ref
            let model_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // sound: int32[10]
            let sound = crate::util::read_array_i32::<10>(chunk)?;

            // geo_box_min: float[3]
            let geo_box_min = crate::util::read_array_f32::<3>(chunk)?;

            // geo_box_max: float[3]
            let geo_box_max = crate::util::read_array_f32::<3>(chunk)?;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            let object_effect_package_id = ObjectEffectPackageKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(GameObjectDisplayInfoRow {
                id,
                model_name,
                sound,
                geo_box_min,
                geo_box_max,
                object_effect_package_id,
            });
        }

        Ok(GameObjectDisplayInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GameObjectDisplayInfo) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_name: string_ref
            b.write_all(&string_cache.add_string(&row.model_name).to_le_bytes())?;

            // sound: int32[10]
            for i in row.sound {
                b.write_all(&i.to_le_bytes())?;
            }


            // geo_box_min: float[3]
            for i in row.geo_box_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // geo_box_max: float[3]
            for i in row.geo_box_max {
                b.write_all(&i.to_le_bytes())?;
            }


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
impl Indexable<i32> for GameObjectDisplayInfo {
    type Table = Self;

    fn get(&self, key: &GameObjectDisplayInfoKey) -> Option<&GameObjectDisplayInfoRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GameObjectDisplayInfoKey) -> Option<&mut GameObjectDisplayInfoRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameObjectDisplayInfoRow {
    pub id: GameObjectDisplayInfoKey,
    pub model_name: String,
    pub sound: [i32; 10],
    pub geo_box_min: [f32; 3],
    pub geo_box_max: [f32; 3],
    pub object_effect_package_id: ObjectEffectPackageKey,
}

impl DbcRow for GameObjectDisplayInfoRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn game_object_display_info() {
        let mut file = File::open("../wrath-dbc/GameObjectDisplayInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GameObjectDisplayInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GameObjectDisplayInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
