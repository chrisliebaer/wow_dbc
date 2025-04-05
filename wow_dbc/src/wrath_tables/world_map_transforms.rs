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
use std::io::Write;
use super::WrathTable;

pub type WorldMapTransformsKey = crate::PrimaryKey<i32, WorldMapTransforms>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapTransforms {
    pub rows: Vec<WorldMapTransformsRow>,
}

impl WorldMapTransforms {
    pub const FILENAME: &'static str = "WorldMapTransforms.dbc";
    pub const FIELD_COUNT: usize = 10;
    pub const ROW_SIZE: usize = 40;

    pub fn verify(&self, dungeon_map: &DungeonMap, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&WorldMapTransformsRow>> {
        for row in &self.rows {
            if row.map_id.id != 0 && map.get(&row.map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapTransforms>(),
                    row,
                    id,
                    row.map_id.id.into()
                ));
            }

            if row.new_map_id.id != 0 && map.get(&row.new_map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapTransforms>(),
                    row,
                    id,
                    row.new_map_id.id.into()
                ));
            }

            if row.new_dungeon_map_id.id != 0 && dungeon_map.get(&row.new_dungeon_map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapTransforms>(),
                    row,
                    id,
                    row.new_dungeon_map_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for WorldMapTransforms {
    fn into(self) -> WrathTable {
        WrathTable::WorldMapTransforms(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for WorldMapTransforms {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WorldMapTransformsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WorldMapTransformsRow] { &mut self.rows }

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

            // id: primary_key (WorldMapTransforms) int32
            let id = WorldMapTransformsKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // region_min: float[2]
            let region_min = crate::util::read_array_f32::<2>(chunk)?;

            // region_max: float[2]
            let region_max = crate::util::read_array_f32::<2>(chunk)?;

            // new_map_id: foreign_key (Map) int32
            let new_map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // region_offset: float[2]
            let region_offset = crate::util::read_array_f32::<2>(chunk)?;

            // new_dungeon_map_id: foreign_key (DungeonMap) int32
            let new_dungeon_map_id = DungeonMapKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WorldMapTransformsRow {
                id,
                map_id,
                region_min,
                region_max,
                new_map_id,
                region_offset,
                new_dungeon_map_id,
            });
        }

        Ok(WorldMapTransforms { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldMapTransforms) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // region_min: float[2]
            for i in row.region_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // region_max: float[2]
            for i in row.region_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // new_map_id: foreign_key (Map) int32
            b.write_all(&(row.new_map_id.id as i32).to_le_bytes())?;

            // region_offset: float[2]
            for i in row.region_offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // new_dungeon_map_id: foreign_key (DungeonMap) int32
            b.write_all(&(row.new_dungeon_map_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for WorldMapTransforms {
    type Table = Self;

    fn get(&self, key: &WorldMapTransformsKey) -> Option<&WorldMapTransformsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WorldMapTransformsKey) -> Option<&mut WorldMapTransformsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapTransformsRow {
    pub id: WorldMapTransformsKey,
    pub map_id: MapKey,
    pub region_min: [f32; 2],
    pub region_max: [f32; 2],
    pub new_map_id: MapKey,
    pub region_offset: [f32; 2],
    pub new_dungeon_map_id: DungeonMapKey,
}

impl DbcRow for WorldMapTransformsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_map_transforms() {
        let mut file = File::open("../wrath-dbc/WorldMapTransforms.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldMapTransforms::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapTransforms::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
