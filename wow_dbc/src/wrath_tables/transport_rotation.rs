use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type TransportRotationKey = crate::PrimaryKey<i32, TransportRotation>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransportRotation {
    pub rows: Vec<TransportRotationRow>,
}

impl TransportRotation {
    pub const FILENAME: &'static str = "TransportRotation.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

}

impl Into<WrathTable> for TransportRotation {
    fn into(self) -> WrathTable {
        WrathTable::TransportRotation(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TransportRotation {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TransportRotationRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TransportRotationRow] { &mut self.rows }

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

            // id: primary_key (TransportRotation) int32
            let id = TransportRotationKey::new(crate::util::read_i32_le(chunk)?);

            // game_objects_id: foreign_key (GameObjects) int32
            let game_objects_id = crate::util::read_i32_le(chunk)?;

            // time_index: int32
            let time_index = crate::util::read_i32_le(chunk)?;

            // rot: float[4]
            let rot = crate::util::read_array_f32::<4>(chunk)?;


            rows.push(TransportRotationRow {
                id,
                game_objects_id,
                time_index,
                rot,
            });
        }

        Ok(TransportRotation { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TransportRotation) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // game_objects_id: foreign_key (GameObjects) int32
            b.write_all(&row.game_objects_id.to_le_bytes())?;

            // time_index: int32
            b.write_all(&row.time_index.to_le_bytes())?;

            // rot: float[4]
            for i in row.rot {
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
impl Indexable<i32> for TransportRotation {
    type Table = Self;

    fn get(&self, key: &TransportRotationKey) -> Option<&TransportRotationRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TransportRotationKey) -> Option<&mut TransportRotationRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransportRotationRow {
    pub id: TransportRotationKey,
    pub game_objects_id: i32,
    pub time_index: i32,
    pub rot: [f32; 4],
}

impl DbcRow for TransportRotationRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn transport_rotation() {
        let mut file = File::open("../wrath-dbc/TransportRotation.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TransportRotation::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TransportRotation::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
