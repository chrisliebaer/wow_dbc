use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::world_map_area::{
    WorldMapArea, WorldMapAreaKey,
};
use std::io::Write;
use super::VanillaTable;

pub type WorldMapOverlayKey = crate::PrimaryKey<u32, WorldMapOverlay>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapOverlay {
    pub rows: Vec<WorldMapOverlayRow>,
}

impl WorldMapOverlay {
    pub const FILENAME: &'static str = "WorldMapOverlay.dbc";
    pub const FIELD_COUNT: usize = 17;
    pub const ROW_SIZE: usize = 68;

    pub fn verify(&self, world_map_area: &WorldMapArea) -> Result<(), crate::InvalidForeignKeyError<&WorldMapOverlayRow>> {
        for row in &self.rows {
            if row.world_map_area.id != 0 && world_map_area.get(&row.world_map_area).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapOverlay>(),
                    row,
                    id,
                    row.world_map_area.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for WorldMapOverlay {
    fn into(self) -> VanillaTable {
        VanillaTable::WorldMapOverlay(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for WorldMapOverlay {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WorldMapOverlayRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WorldMapOverlayRow] { &mut self.rows }

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

            // id: primary_key (WorldMapOverlay) uint32
            let id = WorldMapOverlayKey::new(crate::util::read_u32_le(chunk)?);

            // world_map_area: foreign_key (WorldMapArea) uint32
            let world_map_area = WorldMapAreaKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_table: uint32[4]
            let area_table = crate::util::read_array_u32::<4>(chunk)?;

            // location_x: uint32
            let location_x = crate::util::read_u32_le(chunk)?;

            // location_y: uint32
            let location_y = crate::util::read_u32_le(chunk)?;

            // texture_name: string_ref
            let texture_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // texture_width: uint32
            let texture_width = crate::util::read_u32_le(chunk)?;

            // texture_height: uint32
            let texture_height = crate::util::read_u32_le(chunk)?;

            // offset_x: uint32
            let offset_x = crate::util::read_u32_le(chunk)?;

            // offset_y: uint32
            let offset_y = crate::util::read_u32_le(chunk)?;

            // hit_rect_top: uint32
            let hit_rect_top = crate::util::read_u32_le(chunk)?;

            // hit_rect_left: uint32
            let hit_rect_left = crate::util::read_u32_le(chunk)?;

            // hit_rect_bottom: uint32
            let hit_rect_bottom = crate::util::read_u32_le(chunk)?;

            // hit_rect_right: uint32
            let hit_rect_right = crate::util::read_u32_le(chunk)?;


            rows.push(WorldMapOverlayRow {
                id,
                world_map_area,
                area_table,
                location_x,
                location_y,
                texture_name,
                texture_width,
                texture_height,
                offset_x,
                offset_y,
                hit_rect_top,
                hit_rect_left,
                hit_rect_bottom,
                hit_rect_right,
            });
        }

        Ok(WorldMapOverlay { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldMapOverlay) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // world_map_area: foreign_key (WorldMapArea) uint32
            b.write_all(&(row.world_map_area.id as u32).to_le_bytes())?;

            // area_table: uint32[4]
            for i in row.area_table {
                b.write_all(&i.to_le_bytes())?;
            }


            // location_x: uint32
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: uint32
            b.write_all(&row.location_y.to_le_bytes())?;

            // texture_name: string_ref
            b.write_all(&string_cache.add_string(&row.texture_name).to_le_bytes())?;

            // texture_width: uint32
            b.write_all(&row.texture_width.to_le_bytes())?;

            // texture_height: uint32
            b.write_all(&row.texture_height.to_le_bytes())?;

            // offset_x: uint32
            b.write_all(&row.offset_x.to_le_bytes())?;

            // offset_y: uint32
            b.write_all(&row.offset_y.to_le_bytes())?;

            // hit_rect_top: uint32
            b.write_all(&row.hit_rect_top.to_le_bytes())?;

            // hit_rect_left: uint32
            b.write_all(&row.hit_rect_left.to_le_bytes())?;

            // hit_rect_bottom: uint32
            b.write_all(&row.hit_rect_bottom.to_le_bytes())?;

            // hit_rect_right: uint32
            b.write_all(&row.hit_rect_right.to_le_bytes())?;

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
impl Indexable<u32> for WorldMapOverlay {
    type Table = Self;

    fn get(&self, key: &WorldMapOverlayKey) -> Option<&WorldMapOverlayRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WorldMapOverlayKey) -> Option<&mut WorldMapOverlayRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapOverlayRow {
    pub id: WorldMapOverlayKey,
    pub world_map_area: WorldMapAreaKey,
    pub area_table: [u32; 4],
    pub location_x: u32,
    pub location_y: u32,
    pub texture_name: String,
    pub texture_width: u32,
    pub texture_height: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub hit_rect_top: u32,
    pub hit_rect_left: u32,
    pub hit_rect_bottom: u32,
    pub hit_rect_right: u32,
}

impl DbcRow for WorldMapOverlayRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_map_overlay() {
        let mut file = File::open("../vanilla-dbc/WorldMapOverlay.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldMapOverlay::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapOverlay::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
