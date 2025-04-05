use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type ScalingStatDistributionKey = crate::PrimaryKey<i32, ScalingStatDistribution>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScalingStatDistribution {
    pub rows: Vec<ScalingStatDistributionRow>,
}

impl ScalingStatDistribution {
    pub const FILENAME: &'static str = "ScalingStatDistribution.dbc";
    pub const FIELD_COUNT: usize = 22;
    pub const ROW_SIZE: usize = 88;

}

impl Into<WrathTable> for ScalingStatDistribution {
    fn into(self) -> WrathTable {
        WrathTable::ScalingStatDistribution(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ScalingStatDistribution {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ScalingStatDistributionRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ScalingStatDistributionRow] { &mut self.rows }

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

            // id: primary_key (ScalingStatDistribution) int32
            let id = ScalingStatDistributionKey::new(crate::util::read_i32_le(chunk)?);

            // stat_id: int32[10]
            let stat_id = crate::util::read_array_i32::<10>(chunk)?;

            // bonus: int32[10]
            let bonus = crate::util::read_array_i32::<10>(chunk)?;

            // maxlevel: int32
            let maxlevel = crate::util::read_i32_le(chunk)?;


            rows.push(ScalingStatDistributionRow {
                id,
                stat_id,
                bonus,
                maxlevel,
            });
        }

        Ok(ScalingStatDistribution { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ScalingStatDistribution) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // stat_id: int32[10]
            for i in row.stat_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // bonus: int32[10]
            for i in row.bonus {
                b.write_all(&i.to_le_bytes())?;
            }


            // maxlevel: int32
            b.write_all(&row.maxlevel.to_le_bytes())?;

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
impl Indexable<i32> for ScalingStatDistribution {
    type Table = Self;

    fn get(&self, key: &ScalingStatDistributionKey) -> Option<&ScalingStatDistributionRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ScalingStatDistributionKey) -> Option<&mut ScalingStatDistributionRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScalingStatDistributionRow {
    pub id: ScalingStatDistributionKey,
    pub stat_id: [i32; 10],
    pub bonus: [i32; 10],
    pub maxlevel: i32,
}

impl DbcRow for ScalingStatDistributionRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn scaling_stat_distribution() {
        let mut file = File::open("../wrath-dbc/ScalingStatDistribution.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ScalingStatDistribution::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ScalingStatDistribution::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
