use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::WeaponFlags;

pub type AnimationDataKey = crate::PrimaryKey<u32, AnimationData>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationData {
    pub rows: Vec<AnimationDataRow>,
}

impl AnimationData {
    pub const FILENAME: &'static str = "AnimationData.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, ) -> Result<(), crate::InvalidForeignKeyError<&AnimationDataRow>> {
        for row in &self.rows {
            if row.fallback.id != 0 && self.get(&row.fallback).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AnimationData>(),
                    row,
                    id,
                    row.fallback.id.into()
                ));
            }

            if row.behaviour.id != 0 && self.get(&row.behaviour).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AnimationData>(),
                    row,
                    id,
                    row.behaviour.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for AnimationData {
    fn into(self) -> VanillaTable {
        VanillaTable::AnimationData(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AnimationData {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AnimationDataRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AnimationDataRow] { &mut self.rows }

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

            // id: primary_key (AnimationData) uint32
            let id = AnimationDataKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // weapon_flags: WeaponFlags
            let weapon_flags = WeaponFlags::new(crate::util::read_i32_le(chunk)? as _);

            // body_flags: int32
            let body_flags = crate::util::read_i32_le(chunk)?;

            // unknown: int32
            let unknown = crate::util::read_i32_le(chunk)?;

            // fallback: foreign_key (AnimationData) uint32
            let fallback = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // behaviour: foreign_key (AnimationData) uint32
            let behaviour = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(AnimationDataRow {
                id,
                name,
                weapon_flags,
                body_flags,
                unknown,
                fallback,
                behaviour,
            });
        }

        Ok(AnimationData { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AnimationData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // weapon_flags: WeaponFlags
            b.write_all(&(row.weapon_flags.as_int() as i32).to_le_bytes())?;

            // body_flags: int32
            b.write_all(&row.body_flags.to_le_bytes())?;

            // unknown: int32
            b.write_all(&row.unknown.to_le_bytes())?;

            // fallback: foreign_key (AnimationData) uint32
            b.write_all(&(row.fallback.id as u32).to_le_bytes())?;

            // behaviour: foreign_key (AnimationData) uint32
            b.write_all(&(row.behaviour.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for AnimationData {
    type Table = Self;

    fn get(&self, key: &AnimationDataKey) -> Option<&AnimationDataRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AnimationDataKey) -> Option<&mut AnimationDataRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationDataRow {
    pub id: AnimationDataKey,
    pub name: String,
    pub weapon_flags: WeaponFlags,
    pub body_flags: i32,
    pub unknown: i32,
    pub fallback: AnimationDataKey,
    pub behaviour: AnimationDataKey,
}

impl DbcRow for AnimationDataRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn animation_data() {
        let mut file = File::open("../vanilla-dbc/AnimationData.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AnimationData::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AnimationData::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
