use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::taxi_nodes::{
    TaxiNodes, TaxiNodesKey,
};
use std::io::Write;
use super::WrathTable;

pub type TaxiPathKey = crate::PrimaryKey<i32, TaxiPath>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiPath {
    pub rows: Vec<TaxiPathRow>,
}

impl TaxiPath {
    pub const FILENAME: &'static str = "TaxiPath.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, taxi_nodes: &TaxiNodes) -> Result<(), crate::InvalidForeignKeyError<&TaxiPathRow>> {
        for row in &self.rows {
            if row.from_taxi_node.id != 0 && taxi_nodes.get(&row.from_taxi_node).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TaxiPath>(),
                    row,
                    id,
                    row.from_taxi_node.id.into()
                ));
            }

            if row.to_taxi_node.id != 0 && taxi_nodes.get(&row.to_taxi_node).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TaxiPath>(),
                    row,
                    id,
                    row.to_taxi_node.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for TaxiPath {
    fn into(self) -> WrathTable {
        WrathTable::TaxiPath(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TaxiPath {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TaxiPathRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TaxiPathRow] { &mut self.rows }

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

            // id: primary_key (TaxiPath) int32
            let id = TaxiPathKey::new(crate::util::read_i32_le(chunk)?);

            // from_taxi_node: foreign_key (TaxiNodes) int32
            let from_taxi_node = TaxiNodesKey::new(crate::util::read_i32_le(chunk)?.into());

            // to_taxi_node: foreign_key (TaxiNodes) int32
            let to_taxi_node = TaxiNodesKey::new(crate::util::read_i32_le(chunk)?.into());

            // cost: int32
            let cost = crate::util::read_i32_le(chunk)?;


            rows.push(TaxiPathRow {
                id,
                from_taxi_node,
                to_taxi_node,
                cost,
            });
        }

        Ok(TaxiPath { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TaxiPath) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // from_taxi_node: foreign_key (TaxiNodes) int32
            b.write_all(&(row.from_taxi_node.id as i32).to_le_bytes())?;

            // to_taxi_node: foreign_key (TaxiNodes) int32
            b.write_all(&(row.to_taxi_node.id as i32).to_le_bytes())?;

            // cost: int32
            b.write_all(&row.cost.to_le_bytes())?;

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
impl Indexable<i32> for TaxiPath {
    type Table = Self;

    fn get(&self, key: &TaxiPathKey) -> Option<&TaxiPathRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TaxiPathKey) -> Option<&mut TaxiPathRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiPathRow {
    pub id: TaxiPathKey,
    pub from_taxi_node: TaxiNodesKey,
    pub to_taxi_node: TaxiNodesKey,
    pub cost: i32,
}

impl DbcRow for TaxiPathRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn taxi_path() {
        let mut file = File::open("../wrath-dbc/TaxiPath.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TaxiPath::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TaxiPath::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
