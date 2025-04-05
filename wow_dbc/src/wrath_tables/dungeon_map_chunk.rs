use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::dungeon_map::{
    DungeonMap, DungeonMapKey,
};
use crate::wrath_tables::map::{
    Map, MapKey,
};
use crate::wrath_tables::wmo_area_table::{
    WMOAreaTable, WMOAreaTableKey,
};
use std::io::Write;
use super::WrathTable;

pub type DungeonMapChunkKey = crate::PrimaryKey<i32, DungeonMapChunk>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DungeonMapChunk {
    pub rows: Vec<DungeonMapChunkRow>,
}

impl DungeonMapChunk {
    pub const FILENAME: &'static str = "DungeonMapChunk.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

    pub fn verify(&self, dungeon_map: &DungeonMap, map: &Map, wmo_area_table: &WMOAreaTable) -> Result<(), crate::InvalidForeignKeyError<&DungeonMapChunkRow>> {
        for row in &self.rows {
            if row.map_id.id != 0 && map.get(&row.map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<DungeonMapChunk>(),
                    row,
                    id,
                    row.map_id.id.into()
                ));
            }

            if row.w_m_o_group_id.id != 0 && wmo_area_table.get(&row.w_m_o_group_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<DungeonMapChunk>(),
                    row,
                    id,
                    row.w_m_o_group_id.id.into()
                ));
            }

            if row.dungeon_map_id.id != 0 && dungeon_map.get(&row.dungeon_map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<DungeonMapChunk>(),
                    row,
                    id,
                    row.dungeon_map_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for DungeonMapChunk {
    fn into(self) -> WrathTable {
        WrathTable::DungeonMapChunk(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for DungeonMapChunk {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[DungeonMapChunkRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [DungeonMapChunkRow] { &mut self.rows }

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

            // id: primary_key (DungeonMapChunk) int32
            let id = DungeonMapChunkKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // w_m_o_group_id: foreign_key (WMOAreaTable) int32
            let w_m_o_group_id = WMOAreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // dungeon_map_id: foreign_key (DungeonMap) int32
            let dungeon_map_id = DungeonMapKey::new(crate::util::read_i32_le(chunk)?.into());

            // min_z: float
            let min_z = crate::util::read_f32_le(chunk)?;


            rows.push(DungeonMapChunkRow {
                id,
                map_id,
                w_m_o_group_id,
                dungeon_map_id,
                min_z,
            });
        }

        Ok(DungeonMapChunk { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (DungeonMapChunk) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // w_m_o_group_id: foreign_key (WMOAreaTable) int32
            b.write_all(&(row.w_m_o_group_id.id as i32).to_le_bytes())?;

            // dungeon_map_id: foreign_key (DungeonMap) int32
            b.write_all(&(row.dungeon_map_id.id as i32).to_le_bytes())?;

            // min_z: float
            b.write_all(&row.min_z.to_le_bytes())?;

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
impl Indexable<i32> for DungeonMapChunk {
    type Table = Self;

    fn get(&self, key: &DungeonMapChunkKey) -> Option<&DungeonMapChunkRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &DungeonMapChunkKey) -> Option<&mut DungeonMapChunkRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DungeonMapChunkRow {
    pub id: DungeonMapChunkKey,
    pub map_id: MapKey,
    pub w_m_o_group_id: WMOAreaTableKey,
    pub dungeon_map_id: DungeonMapKey,
    pub min_z: f32,
}

impl DbcRow for DungeonMapChunkRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn dungeon_map_chunk() {
        let mut file = File::open("../wrath-dbc/DungeonMapChunk.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DungeonMapChunk::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DungeonMapChunk::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
