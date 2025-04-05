use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::sound_filter::{
    SoundFilter, SoundFilterKey,
};
use std::io::Write;
use super::WrathTable;

pub type SoundFilterElemKey = crate::PrimaryKey<i32, SoundFilterElem>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundFilterElem {
    pub rows: Vec<SoundFilterElemRow>,
}

impl SoundFilterElem {
    pub const FILENAME: &'static str = "SoundFilterElem.dbc";
    pub const FIELD_COUNT: usize = 13;
    pub const ROW_SIZE: usize = 52;

    pub fn verify(&self, sound_filter: &SoundFilter) -> Result<(), crate::InvalidForeignKeyError<&SoundFilterElemRow>> {
        for row in &self.rows {
            if row.sound_filter_id.id != 0 && sound_filter.get(&row.sound_filter_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SoundFilterElem>(),
                    row,
                    id,
                    row.sound_filter_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for SoundFilterElem {
    fn into(self) -> WrathTable {
        WrathTable::SoundFilterElem(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SoundFilterElem {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SoundFilterElemRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SoundFilterElemRow] { &mut self.rows }

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

            // id: primary_key (SoundFilterElem) int32
            let id = SoundFilterElemKey::new(crate::util::read_i32_le(chunk)?);

            // sound_filter_id: foreign_key (SoundFilter) int32
            let sound_filter_id = SoundFilterKey::new(crate::util::read_i32_le(chunk)?.into());

            // order_index: int32
            let order_index = crate::util::read_i32_le(chunk)?;

            // filter_type: int32
            let filter_type = crate::util::read_i32_le(chunk)?;

            // params: float[9]
            let params = crate::util::read_array_f32::<9>(chunk)?;


            rows.push(SoundFilterElemRow {
                id,
                sound_filter_id,
                order_index,
                filter_type,
                params,
            });
        }

        Ok(SoundFilterElem { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SoundFilterElem) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_filter_id: foreign_key (SoundFilter) int32
            b.write_all(&(row.sound_filter_id.id as i32).to_le_bytes())?;

            // order_index: int32
            b.write_all(&row.order_index.to_le_bytes())?;

            // filter_type: int32
            b.write_all(&row.filter_type.to_le_bytes())?;

            // params: float[9]
            for i in row.params {
                b.write_all(&i.to_le_bytes())?;
            }


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
impl Indexable<i32> for SoundFilterElem {
    type Table = Self;

    fn get(&self, key: &SoundFilterElemKey) -> Option<&SoundFilterElemRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SoundFilterElemKey) -> Option<&mut SoundFilterElemRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundFilterElemRow {
    pub id: SoundFilterElemKey,
    pub sound_filter_id: SoundFilterKey,
    pub order_index: i32,
    pub filter_type: i32,
    pub params: [f32; 9],
}

impl DbcRow for SoundFilterElemRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_filter_elem() {
        let mut file = File::open("../wrath-dbc/SoundFilterElem.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundFilterElem::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundFilterElem::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
