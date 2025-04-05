use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::map::{
    Map, MapKey,
};
use crate::tbc_tables::taxi_path::{
    TaxiPath, TaxiPathKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type TaxiPathNodeKey = crate::PrimaryKey<i32, TaxiPathNode>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiPathNode {
    pub rows: Vec<TaxiPathNodeRow>,
}

impl TaxiPathNode {
    pub const FILENAME: &'static str = "TaxiPathNode.dbc";
    pub const FIELD_COUNT: usize = 11;
    pub const ROW_SIZE: usize = 44;

    pub fn verify(&self, map: &Map, taxi_path: &TaxiPath) -> Result<(), crate::InvalidForeignKeyError<&TaxiPathNodeRow>> {
        for row in &self.rows {
            if row.path_id.id != 0 && taxi_path.get(&row.path_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TaxiPathNode>(),
                    row,
                    id,
                    row.path_id.id.into()
                ));
            }

            if row.continent_id.id != 0 && map.get(&row.continent_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TaxiPathNode>(),
                    row,
                    id,
                    row.continent_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for TaxiPathNode {
    fn into(self) -> TbcTable {
        TbcTable::TaxiPathNode(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TaxiPathNode {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TaxiPathNodeRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TaxiPathNodeRow] { &mut self.rows }

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

            // id: primary_key (TaxiPathNode) int32
            let id = TaxiPathNodeKey::new(crate::util::read_i32_le(chunk)?);

            // path_id: foreign_key (TaxiPath) int32
            let path_id = TaxiPathKey::new(crate::util::read_i32_le(chunk)?.into());

            // node_index: int32
            let node_index = crate::util::read_i32_le(chunk)?;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // loc: float[3]
            let loc = crate::util::read_array_f32::<3>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // delay: int32
            let delay = crate::util::read_i32_le(chunk)?;

            // arrival_event_id: int32
            let arrival_event_id = crate::util::read_i32_le(chunk)?;

            // departure_event_id: int32
            let departure_event_id = crate::util::read_i32_le(chunk)?;


            rows.push(TaxiPathNodeRow {
                id,
                path_id,
                node_index,
                continent_id,
                loc,
                flags,
                delay,
                arrival_event_id,
                departure_event_id,
            });
        }

        Ok(TaxiPathNode { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TaxiPathNode) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // path_id: foreign_key (TaxiPath) int32
            b.write_all(&(row.path_id.id as i32).to_le_bytes())?;

            // node_index: int32
            b.write_all(&row.node_index.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // loc: float[3]
            for i in row.loc {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // delay: int32
            b.write_all(&row.delay.to_le_bytes())?;

            // arrival_event_id: int32
            b.write_all(&row.arrival_event_id.to_le_bytes())?;

            // departure_event_id: int32
            b.write_all(&row.departure_event_id.to_le_bytes())?;

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
impl Indexable<i32> for TaxiPathNode {
    type Table = Self;

    fn get(&self, key: &TaxiPathNodeKey) -> Option<&TaxiPathNodeRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TaxiPathNodeKey) -> Option<&mut TaxiPathNodeRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiPathNodeRow {
    pub id: TaxiPathNodeKey,
    pub path_id: TaxiPathKey,
    pub node_index: i32,
    pub continent_id: MapKey,
    pub loc: [f32; 3],
    pub flags: i32,
    pub delay: i32,
    pub arrival_event_id: i32,
    pub departure_event_id: i32,
}

impl DbcRow for TaxiPathNodeRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn taxi_path_node() {
        let mut file = File::open("../tbc-dbc/TaxiPathNode.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TaxiPathNode::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TaxiPathNode::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
