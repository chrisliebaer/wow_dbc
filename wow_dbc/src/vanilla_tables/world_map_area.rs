use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::area_table::AreaTableKey;
use crate::vanilla_tables::world_map_continent::WorldMapContinentKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapArea {
    pub rows: Vec<WorldMapAreaRow>,
}

impl DbcTable for WorldMapArea {
    type Row = WorldMapAreaRow;

    const FILENAME: &'static str = "WorldMapArea.dbc";
    const FIELD_COUNT: usize = 8;
    const ROW_SIZE: usize = 32;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldMapArea) uint32
            let id = WorldMapAreaKey::new(crate::util::read_u32_le(chunk)?);

            // world_map_continent: foreign_key (WorldMapContinent) uint32
            let world_map_continent = WorldMapContinentKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_name: string_ref
            let area_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // location_left: float
            let location_left = crate::util::read_f32_le(chunk)?;

            // location_right: float
            let location_right = crate::util::read_f32_le(chunk)?;

            // location_top: float
            let location_top = crate::util::read_f32_le(chunk)?;

            // location_bottom: float
            let location_bottom = crate::util::read_f32_le(chunk)?;


            rows.push(WorldMapAreaRow {
                id,
                world_map_continent,
                area_table,
                area_name,
                location_left,
                location_right,
                location_top,
                location_bottom,
            });
        }

        Ok(WorldMapArea { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: 32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldMapArea) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // world_map_continent: foreign_key (WorldMapContinent) uint32
            b.write_all(&(row.world_map_continent.id as u32).to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // area_name: string_ref
            if !row.area_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.area_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // location_left: float
            b.write_all(&row.location_left.to_le_bytes())?;

            // location_right: float
            b.write_all(&row.location_right.to_le_bytes())?;

            // location_top: float
            b.write_all(&row.location_top.to_le_bytes())?;

            // location_bottom: float
            b.write_all(&row.location_bottom.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for WorldMapArea {
    type PrimaryKey = WorldMapAreaKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl WorldMapArea {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.area_name.is_empty() { b.write_all(row.area_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.area_name.is_empty() { sum += row.area_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapAreaKey {
    pub id: u32
}

impl WorldMapAreaKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WorldMapAreaKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WorldMapAreaKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for WorldMapAreaKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for WorldMapAreaKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WorldMapAreaKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for WorldMapAreaKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for WorldMapAreaKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for WorldMapAreaKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WorldMapAreaKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WorldMapAreaKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapAreaRow {
    pub id: WorldMapAreaKey,
    pub world_map_continent: WorldMapContinentKey,
    pub area_table: AreaTableKey,
    pub area_name: String,
    pub location_left: f32,
    pub location_right: f32,
    pub location_top: f32,
    pub location_bottom: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_map_area() {
        let contents = include_bytes!("../../../vanilla-dbc/WorldMapArea.dbc");
        let actual = WorldMapArea::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapArea::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
