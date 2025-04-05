use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::area_table::{
    AreaTable, AreaTableKey,
};
use crate::wrath_tables::map::{
    Map, MapKey,
};
use std::io::Write;
use super::WrathTable;

pub type DungeonMapKey = crate::PrimaryKey<i32, DungeonMap>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DungeonMap {
    pub rows: Vec<DungeonMapRow>,
}

impl DungeonMap {
    pub const FILENAME: &'static str = "DungeonMap.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

    pub fn verify(&self, area_table: &AreaTable, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&DungeonMapRow>> {
        for row in &self.rows {
            if row.map_id.id != 0 && map.get(&row.map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<DungeonMap>(),
                    row,
                    id,
                    row.map_id.id.into()
                ));
            }

            if row.parent_world_map_id.id != 0 && area_table.get(&row.parent_world_map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<DungeonMap>(),
                    row,
                    id,
                    row.parent_world_map_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for DungeonMap {
    fn into(self) -> WrathTable {
        WrathTable::DungeonMap(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for DungeonMap {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[DungeonMapRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [DungeonMapRow] { &mut self.rows }

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

            // id: primary_key (DungeonMap) int32
            let id = DungeonMapKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // floor_index: int32
            let floor_index = crate::util::read_i32_le(chunk)?;

            // min_x: float
            let min_x = crate::util::read_f32_le(chunk)?;

            // max_x: float
            let max_x = crate::util::read_f32_le(chunk)?;

            // min_y: float
            let min_y = crate::util::read_f32_le(chunk)?;

            // max_y: float
            let max_y = crate::util::read_f32_le(chunk)?;

            // parent_world_map_id: foreign_key (AreaTable) int32
            let parent_world_map_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(DungeonMapRow {
                id,
                map_id,
                floor_index,
                min_x,
                max_x,
                min_y,
                max_y,
                parent_world_map_id,
            });
        }

        Ok(DungeonMap { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (DungeonMap) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // floor_index: int32
            b.write_all(&row.floor_index.to_le_bytes())?;

            // min_x: float
            b.write_all(&row.min_x.to_le_bytes())?;

            // max_x: float
            b.write_all(&row.max_x.to_le_bytes())?;

            // min_y: float
            b.write_all(&row.min_y.to_le_bytes())?;

            // max_y: float
            b.write_all(&row.max_y.to_le_bytes())?;

            // parent_world_map_id: foreign_key (AreaTable) int32
            b.write_all(&(row.parent_world_map_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for DungeonMap {
    type Table = Self;

    fn get(&self, key: &DungeonMapKey) -> Option<&DungeonMapRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &DungeonMapKey) -> Option<&mut DungeonMapRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DungeonMapRow {
    pub id: DungeonMapKey,
    pub map_id: MapKey,
    pub floor_index: i32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub parent_world_map_id: AreaTableKey,
}

impl DbcRow for DungeonMapRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn dungeon_map() {
        let mut file = File::open("../wrath-dbc/DungeonMap.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DungeonMap::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DungeonMap::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
