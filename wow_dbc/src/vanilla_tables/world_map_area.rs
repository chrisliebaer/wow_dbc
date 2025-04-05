use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::area_table::{
    AreaTable, AreaTableKey,
};
use crate::vanilla_tables::world_map_continent::{
    WorldMapContinent, WorldMapContinentKey,
};
use std::io::Write;
use super::VanillaTable;

pub type WorldMapAreaKey = crate::PrimaryKey<u32, WorldMapArea>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapArea {
    pub rows: Vec<WorldMapAreaRow>,
}

impl WorldMapArea {
    pub const FILENAME: &'static str = "WorldMapArea.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

    pub fn verify(&self, area_table: &AreaTable, world_map_continent: &WorldMapContinent) -> Result<(), crate::InvalidForeignKeyError<&WorldMapAreaRow>> {
        for row in &self.rows {
            if row.world_map_continent.id != 0 && world_map_continent.get(&row.world_map_continent).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapArea>(),
                    row,
                    id,
                    row.world_map_continent.id.into()
                ));
            }

            if row.area_table.id != 0 && area_table.get(&row.area_table).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapArea>(),
                    row,
                    id,
                    row.area_table.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for WorldMapArea {
    fn into(self) -> VanillaTable {
        VanillaTable::WorldMapArea(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for WorldMapArea {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WorldMapAreaRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WorldMapAreaRow] { &mut self.rows }

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldMapArea) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // world_map_continent: foreign_key (WorldMapContinent) uint32
            b.write_all(&(row.world_map_continent.id as u32).to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // area_name: string_ref
            b.write_all(&string_cache.add_string(&row.area_name).to_le_bytes())?;

            // location_left: float
            b.write_all(&row.location_left.to_le_bytes())?;

            // location_right: float
            b.write_all(&row.location_right.to_le_bytes())?;

            // location_top: float
            b.write_all(&row.location_top.to_le_bytes())?;

            // location_bottom: float
            b.write_all(&row.location_bottom.to_le_bytes())?;

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
impl Indexable<u32> for WorldMapArea {
    type Table = Self;

    fn get(&self, key: &WorldMapAreaKey) -> Option<&WorldMapAreaRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WorldMapAreaKey) -> Option<&mut WorldMapAreaRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
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

impl DbcRow for WorldMapAreaRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_map_area() {
        let mut file = File::open("../vanilla-dbc/WorldMapArea.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldMapArea::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapArea::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
