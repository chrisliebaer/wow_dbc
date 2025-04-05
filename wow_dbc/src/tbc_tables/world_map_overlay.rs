use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::world_map_area::{
    WorldMapArea, WorldMapAreaKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type WorldMapOverlayKey = crate::PrimaryKey<i32, WorldMapOverlay>;

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
            if row.map_area_id.id != 0 && world_map_area.get(&row.map_area_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldMapOverlay>(),
                    row,
                    id,
                    row.map_area_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for WorldMapOverlay {
    fn into(self) -> TbcTable {
        TbcTable::WorldMapOverlay(self)
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

            // id: primary_key (WorldMapOverlay) int32
            let id = WorldMapOverlayKey::new(crate::util::read_i32_le(chunk)?);

            // map_area_id: foreign_key (WorldMapArea) int32
            let map_area_id = WorldMapAreaKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_id: int32[4]
            let area_id = crate::util::read_array_i32::<4>(chunk)?;

            // map_point_x: int32
            let map_point_x = crate::util::read_i32_le(chunk)?;

            // map_point_y: int32
            let map_point_y = crate::util::read_i32_le(chunk)?;

            // texture_name: string_ref
            let texture_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // texture_width: int32
            let texture_width = crate::util::read_i32_le(chunk)?;

            // texture_height: int32
            let texture_height = crate::util::read_i32_le(chunk)?;

            // offset_x: int32
            let offset_x = crate::util::read_i32_le(chunk)?;

            // offset_y: int32
            let offset_y = crate::util::read_i32_le(chunk)?;

            // hit_rect_top: int32
            let hit_rect_top = crate::util::read_i32_le(chunk)?;

            // hit_rect_left: int32
            let hit_rect_left = crate::util::read_i32_le(chunk)?;

            // hit_rect_bottom: int32
            let hit_rect_bottom = crate::util::read_i32_le(chunk)?;

            // hit_rect_right: int32
            let hit_rect_right = crate::util::read_i32_le(chunk)?;


            rows.push(WorldMapOverlayRow {
                id,
                map_area_id,
                area_id,
                map_point_x,
                map_point_y,
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
            // id: primary_key (WorldMapOverlay) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_area_id: foreign_key (WorldMapArea) int32
            b.write_all(&(row.map_area_id.id as i32).to_le_bytes())?;

            // area_id: int32[4]
            for i in row.area_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // map_point_x: int32
            b.write_all(&row.map_point_x.to_le_bytes())?;

            // map_point_y: int32
            b.write_all(&row.map_point_y.to_le_bytes())?;

            // texture_name: string_ref
            b.write_all(&string_cache.add_string(&row.texture_name).to_le_bytes())?;

            // texture_width: int32
            b.write_all(&row.texture_width.to_le_bytes())?;

            // texture_height: int32
            b.write_all(&row.texture_height.to_le_bytes())?;

            // offset_x: int32
            b.write_all(&row.offset_x.to_le_bytes())?;

            // offset_y: int32
            b.write_all(&row.offset_y.to_le_bytes())?;

            // hit_rect_top: int32
            b.write_all(&row.hit_rect_top.to_le_bytes())?;

            // hit_rect_left: int32
            b.write_all(&row.hit_rect_left.to_le_bytes())?;

            // hit_rect_bottom: int32
            b.write_all(&row.hit_rect_bottom.to_le_bytes())?;

            // hit_rect_right: int32
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
impl Indexable<i32> for WorldMapOverlay {
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
    pub map_area_id: WorldMapAreaKey,
    pub area_id: [i32; 4],
    pub map_point_x: i32,
    pub map_point_y: i32,
    pub texture_name: String,
    pub texture_width: i32,
    pub texture_height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub hit_rect_top: i32,
    pub hit_rect_left: i32,
    pub hit_rect_bottom: i32,
    pub hit_rect_right: i32,
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
        let mut file = File::open("../tbc-dbc/WorldMapOverlay.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldMapOverlay::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapOverlay::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
