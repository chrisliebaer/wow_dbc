use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type CreatureMovementInfoKey = crate::PrimaryKey<i32, CreatureMovementInfo>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureMovementInfo {
    pub rows: Vec<CreatureMovementInfoRow>,
}

impl CreatureMovementInfo {
    pub const FILENAME: &'static str = "CreatureMovementInfo.dbc";
    pub const FIELD_COUNT: usize = 2;
    pub const ROW_SIZE: usize = 8;

}

impl Into<WrathTable> for CreatureMovementInfo {
    fn into(self) -> WrathTable {
        WrathTable::CreatureMovementInfo(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CreatureMovementInfo {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CreatureMovementInfoRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CreatureMovementInfoRow] { &mut self.rows }

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

            // id: primary_key (CreatureMovementInfo) int32
            let id = CreatureMovementInfoKey::new(crate::util::read_i32_le(chunk)?);

            // smooth_facing_chase_rate: float
            let smooth_facing_chase_rate = crate::util::read_f32_le(chunk)?;


            rows.push(CreatureMovementInfoRow {
                id,
                smooth_facing_chase_rate,
            });
        }

        Ok(CreatureMovementInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureMovementInfo) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // smooth_facing_chase_rate: float
            b.write_all(&row.smooth_facing_chase_rate.to_le_bytes())?;

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
impl Indexable<i32> for CreatureMovementInfo {
    type Table = Self;

    fn get(&self, key: &CreatureMovementInfoKey) -> Option<&CreatureMovementInfoRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CreatureMovementInfoKey) -> Option<&mut CreatureMovementInfoRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureMovementInfoRow {
    pub id: CreatureMovementInfoKey,
    pub smooth_facing_chase_rate: f32,
}

impl DbcRow for CreatureMovementInfoRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_movement_info() {
        let mut file = File::open("../wrath-dbc/CreatureMovementInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureMovementInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureMovementInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
