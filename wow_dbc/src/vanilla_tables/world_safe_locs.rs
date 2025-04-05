use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::map::{
    Map, MapKey,
};
use std::io::Write;
use super::VanillaTable;

pub type WorldSafeLocsKey = crate::PrimaryKey<u32, WorldSafeLocs>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldSafeLocs {
    pub rows: Vec<WorldSafeLocsRow>,
}

impl WorldSafeLocs {
    pub const FILENAME: &'static str = "WorldSafeLocs.dbc";
    pub const FIELD_COUNT: usize = 14;
    pub const ROW_SIZE: usize = 56;

    pub fn verify(&self, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&WorldSafeLocsRow>> {
        for row in &self.rows {
            if row.map.id != 0 && map.get(&row.map).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldSafeLocs>(),
                    row,
                    id,
                    row.map.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for WorldSafeLocs {
    fn into(self) -> VanillaTable {
        VanillaTable::WorldSafeLocs(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for WorldSafeLocs {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WorldSafeLocsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WorldSafeLocsRow] { &mut self.rows }

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

            // id: primary_key (WorldSafeLocs) uint32
            let id = WorldSafeLocsKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // area_name: string_ref_loc
            let area_name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(WorldSafeLocsRow {
                id,
                map,
                location_x,
                location_y,
                location_z,
                area_name,
            });
        }

        Ok(WorldSafeLocs { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldSafeLocs) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // area_name: string_ref_loc
            b.write_all(&row.area_name.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<u32> for WorldSafeLocs {
    type Table = Self;

    fn get(&self, key: &WorldSafeLocsKey) -> Option<&WorldSafeLocsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WorldSafeLocsKey) -> Option<&mut WorldSafeLocsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldSafeLocsRow {
    pub id: WorldSafeLocsKey,
    pub map: MapKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub area_name: LocalizedString,
}

impl DbcRow for WorldSafeLocsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_safe_locs() {
        let mut file = File::open("../vanilla-dbc/WorldSafeLocs.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldSafeLocs::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldSafeLocs::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
