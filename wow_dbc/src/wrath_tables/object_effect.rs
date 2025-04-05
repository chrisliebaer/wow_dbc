use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::object_effect_group::{
    ObjectEffectGroup, ObjectEffectGroupKey,
};
use crate::wrath_tables::object_effect_modifier::{
    ObjectEffectModifier, ObjectEffectModifierKey,
};
use std::io::Write;
use super::WrathTable;

pub type ObjectEffectKey = crate::PrimaryKey<i32, ObjectEffect>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectEffect {
    pub rows: Vec<ObjectEffectRow>,
}

impl ObjectEffect {
    pub const FILENAME: &'static str = "ObjectEffect.dbc";
    pub const FIELD_COUNT: usize = 12;
    pub const ROW_SIZE: usize = 48;

    pub fn verify(&self, object_effect_group: &ObjectEffectGroup, object_effect_modifier: &ObjectEffectModifier) -> Result<(), crate::InvalidForeignKeyError<&ObjectEffectRow>> {
        for row in &self.rows {
            if row.object_effect_group_id.id != 0 && object_effect_group.get(&row.object_effect_group_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ObjectEffect>(),
                    row,
                    id,
                    row.object_effect_group_id.id.into()
                ));
            }

            if row.object_effect_modifier_id.id != 0 && object_effect_modifier.get(&row.object_effect_modifier_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ObjectEffect>(),
                    row,
                    id,
                    row.object_effect_modifier_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for ObjectEffect {
    fn into(self) -> WrathTable {
        WrathTable::ObjectEffect(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ObjectEffect {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ObjectEffectRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ObjectEffectRow] { &mut self.rows }

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

            // id: primary_key (ObjectEffect) int32
            let id = ObjectEffectKey::new(crate::util::read_i32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            let object_effect_group_id = ObjectEffectGroupKey::new(crate::util::read_i32_le(chunk)?.into());

            // trigger_type: int32
            let trigger_type = crate::util::read_i32_le(chunk)?;

            // event_type: int32
            let event_type = crate::util::read_i32_le(chunk)?;

            // effect_rec_type: int32
            let effect_rec_type = crate::util::read_i32_le(chunk)?;

            // effect_rec_id: foreign_key (SoundKit) int32
            let effect_rec_id = crate::util::read_i32_le(chunk)?;

            // attachment: int32
            let attachment = crate::util::read_i32_le(chunk)?;

            // offset: float[3]
            let offset = crate::util::read_array_f32::<3>(chunk)?;

            // object_effect_modifier_id: foreign_key (ObjectEffectModifier) int32
            let object_effect_modifier_id = ObjectEffectModifierKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(ObjectEffectRow {
                id,
                name,
                object_effect_group_id,
                trigger_type,
                event_type,
                effect_rec_type,
                effect_rec_id,
                attachment,
                offset,
                object_effect_modifier_id,
            });
        }

        Ok(ObjectEffect { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ObjectEffect) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            b.write_all(&(row.object_effect_group_id.id as i32).to_le_bytes())?;

            // trigger_type: int32
            b.write_all(&row.trigger_type.to_le_bytes())?;

            // event_type: int32
            b.write_all(&row.event_type.to_le_bytes())?;

            // effect_rec_type: int32
            b.write_all(&row.effect_rec_type.to_le_bytes())?;

            // effect_rec_id: foreign_key (SoundKit) int32
            b.write_all(&row.effect_rec_id.to_le_bytes())?;

            // attachment: int32
            b.write_all(&row.attachment.to_le_bytes())?;

            // offset: float[3]
            for i in row.offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // object_effect_modifier_id: foreign_key (ObjectEffectModifier) int32
            b.write_all(&(row.object_effect_modifier_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for ObjectEffect {
    type Table = Self;

    fn get(&self, key: &ObjectEffectKey) -> Option<&ObjectEffectRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ObjectEffectKey) -> Option<&mut ObjectEffectRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectEffectRow {
    pub id: ObjectEffectKey,
    pub name: String,
    pub object_effect_group_id: ObjectEffectGroupKey,
    pub trigger_type: i32,
    pub event_type: i32,
    pub effect_rec_type: i32,
    pub effect_rec_id: i32,
    pub attachment: i32,
    pub offset: [f32; 3],
    pub object_effect_modifier_id: ObjectEffectModifierKey,
}

impl DbcRow for ObjectEffectRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn object_effect() {
        let mut file = File::open("../wrath-dbc/ObjectEffect.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ObjectEffect::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ObjectEffect::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
