use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type AreaGroupKey = crate::PrimaryKey<i32, AreaGroup>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaGroup {
    pub rows: Vec<AreaGroupRow>,
}

impl AreaGroup {
    pub const FILENAME: &'static str = "AreaGroup.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

    pub fn verify(&self, ) -> Result<(), crate::InvalidForeignKeyError<&AreaGroupRow>> {
        for row in &self.rows {
            if row.next_area_id.id != 0 && self.get(&row.next_area_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaGroup>(),
                    row,
                    id,
                    row.next_area_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for AreaGroup {
    fn into(self) -> WrathTable {
        WrathTable::AreaGroup(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AreaGroup {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AreaGroupRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AreaGroupRow] { &mut self.rows }

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

            // id: primary_key (AreaGroup) int32
            let id = AreaGroupKey::new(crate::util::read_i32_le(chunk)?);

            // area_id: int32[6]
            let area_id = crate::util::read_array_i32::<6>(chunk)?;

            // next_area_id: foreign_key (AreaGroup) int32
            let next_area_id = AreaGroupKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(AreaGroupRow {
                id,
                area_id,
                next_area_id,
            });
        }

        Ok(AreaGroup { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AreaGroup) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // area_id: int32[6]
            for i in row.area_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // next_area_id: foreign_key (AreaGroup) int32
            b.write_all(&(row.next_area_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for AreaGroup {
    type Table = Self;

    fn get(&self, key: &AreaGroupKey) -> Option<&AreaGroupRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AreaGroupKey) -> Option<&mut AreaGroupRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaGroupRow {
    pub id: AreaGroupKey,
    pub area_id: [i32; 6],
    pub next_area_id: AreaGroupKey,
}

impl DbcRow for AreaGroupRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn area_group() {
        let mut file = File::open("../wrath-dbc/AreaGroup.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaGroup::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaGroup::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
