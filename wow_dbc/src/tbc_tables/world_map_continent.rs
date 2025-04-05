use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::map::{
    Map, MapKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type WorldMapContinentKey = crate::PrimaryKey<i32, WorldMapContinent>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapContinent {
    pub rows: Vec<WorldMapContinentRow>,
}

impl WorldMapContinent {
    pub const FILENAME: &'static str = "WorldMapContinent.dbc";
    pub const FIELD_COUNT: usize = 13;
    pub const ROW_SIZE: usize = 52;

    pub fn verify(&self, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&WorldMapContinentRow>> {
        for row in &self.rows {
            if row.map_id.id != 0 && map.get(&row.map_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapContinent>(),
                    row,
                    id,
                    row.map_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for WorldMapContinent {
    fn into(self) -> TbcTable {
        TbcTable::WorldMapContinent(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for WorldMapContinent {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WorldMapContinentRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WorldMapContinentRow] { &mut self.rows }

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

            // id: primary_key (WorldMapContinent) int32
            let id = WorldMapContinentKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // left_boundary: int32
            let left_boundary = crate::util::read_i32_le(chunk)?;

            // right_boundary: int32
            let right_boundary = crate::util::read_i32_le(chunk)?;

            // top_boundary: int32
            let top_boundary = crate::util::read_i32_le(chunk)?;

            // bottom_boundary: int32
            let bottom_boundary = crate::util::read_i32_le(chunk)?;

            // continent_offset: float[2]
            let continent_offset = crate::util::read_array_f32::<2>(chunk)?;

            // scale: float
            let scale = crate::util::read_f32_le(chunk)?;

            // taxi_min: float[2]
            let taxi_min = crate::util::read_array_f32::<2>(chunk)?;

            // taxi_max: float[2]
            let taxi_max = crate::util::read_array_f32::<2>(chunk)?;


            rows.push(WorldMapContinentRow {
                id,
                map_id,
                left_boundary,
                right_boundary,
                top_boundary,
                bottom_boundary,
                continent_offset,
                scale,
                taxi_min,
                taxi_max,
            });
        }

        Ok(WorldMapContinent { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldMapContinent) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // left_boundary: int32
            b.write_all(&row.left_boundary.to_le_bytes())?;

            // right_boundary: int32
            b.write_all(&row.right_boundary.to_le_bytes())?;

            // top_boundary: int32
            b.write_all(&row.top_boundary.to_le_bytes())?;

            // bottom_boundary: int32
            b.write_all(&row.bottom_boundary.to_le_bytes())?;

            // continent_offset: float[2]
            for i in row.continent_offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // scale: float
            b.write_all(&row.scale.to_le_bytes())?;

            // taxi_min: float[2]
            for i in row.taxi_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // taxi_max: float[2]
            for i in row.taxi_max {
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
impl Indexable<i32> for WorldMapContinent {
    type Table = Self;

    fn get(&self, key: &WorldMapContinentKey) -> Option<&WorldMapContinentRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WorldMapContinentKey) -> Option<&mut WorldMapContinentRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapContinentRow {
    pub id: WorldMapContinentKey,
    pub map_id: MapKey,
    pub left_boundary: i32,
    pub right_boundary: i32,
    pub top_boundary: i32,
    pub bottom_boundary: i32,
    pub continent_offset: [f32; 2],
    pub scale: f32,
    pub taxi_min: [f32; 2],
    pub taxi_max: [f32; 2],
}

impl DbcRow for WorldMapContinentRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_map_continent() {
        let mut file = File::open("../tbc-dbc/WorldMapContinent.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldMapContinent::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapContinent::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
