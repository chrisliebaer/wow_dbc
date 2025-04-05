use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::material::{
    Material, MaterialKey,
};
use std::io::Write;
use super::WrathTable;

pub type SheatheSoundLookupsKey = crate::PrimaryKey<i32, SheatheSoundLookups>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookups {
    pub rows: Vec<SheatheSoundLookupsRow>,
}

impl SheatheSoundLookups {
    pub const FILENAME: &'static str = "SheatheSoundLookups.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, material: &Material) -> Result<(), crate::InvalidForeignKeyError<&SheatheSoundLookupsRow>> {
        for row in &self.rows {
            if row.material.id != 0 && material.get(&row.material).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SheatheSoundLookups>(),
                    row,
                    id,
                    row.material.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for SheatheSoundLookups {
    fn into(self) -> WrathTable {
        WrathTable::SheatheSoundLookups(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SheatheSoundLookups {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SheatheSoundLookupsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SheatheSoundLookupsRow] { &mut self.rows }

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

            // id: primary_key (SheatheSoundLookups) int32
            let id = SheatheSoundLookupsKey::new(crate::util::read_i32_le(chunk)?);

            // class_id: int32
            let class_id = crate::util::read_i32_le(chunk)?;

            // subclass_id: int32
            let subclass_id = crate::util::read_i32_le(chunk)?;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // check_material: int32
            let check_material = crate::util::read_i32_le(chunk)?;

            // sheathe_sound: int32
            let sheathe_sound = crate::util::read_i32_le(chunk)?;

            // unsheathe_sound: int32
            let unsheathe_sound = crate::util::read_i32_le(chunk)?;


            rows.push(SheatheSoundLookupsRow {
                id,
                class_id,
                subclass_id,
                material,
                check_material,
                sheathe_sound,
                unsheathe_sound,
            });
        }

        Ok(SheatheSoundLookups { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SheatheSoundLookups) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // class_id: int32
            b.write_all(&row.class_id.to_le_bytes())?;

            // subclass_id: int32
            b.write_all(&row.subclass_id.to_le_bytes())?;

            // material: foreign_key (Material) int32
            b.write_all(&(row.material.id as i32).to_le_bytes())?;

            // check_material: int32
            b.write_all(&row.check_material.to_le_bytes())?;

            // sheathe_sound: int32
            b.write_all(&row.sheathe_sound.to_le_bytes())?;

            // unsheathe_sound: int32
            b.write_all(&row.unsheathe_sound.to_le_bytes())?;

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
impl Indexable<i32> for SheatheSoundLookups {
    type Table = Self;

    fn get(&self, key: &SheatheSoundLookupsKey) -> Option<&SheatheSoundLookupsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SheatheSoundLookupsKey) -> Option<&mut SheatheSoundLookupsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookupsRow {
    pub id: SheatheSoundLookupsKey,
    pub class_id: i32,
    pub subclass_id: i32,
    pub material: MaterialKey,
    pub check_material: i32,
    pub sheathe_sound: i32,
    pub unsheathe_sound: i32,
}

impl DbcRow for SheatheSoundLookupsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sheathe_sound_lookups() {
        let mut file = File::open("../wrath-dbc/SheatheSoundLookups.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SheatheSoundLookups::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SheatheSoundLookups::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
