use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::map::{
    Map, MapKey,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type WorldSafeLocsKey = crate::PrimaryKey<i32, WorldSafeLocs>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldSafeLocs {
    pub rows: Vec<WorldSafeLocsRow>,
}

impl WorldSafeLocs {
    pub const FILENAME: &'static str = "WorldSafeLocs.dbc";
    pub const FIELD_COUNT: usize = 22;
    pub const ROW_SIZE: usize = 88;

    pub fn verify(&self, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&WorldSafeLocsRow>> {
        for row in &self.rows {
            if row.continent.id != 0 && map.get(&row.continent).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldSafeLocs>(),
                    row,
                    id,
                    row.continent.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for WorldSafeLocs {
    fn into(self) -> TbcTable {
        TbcTable::WorldSafeLocs(self)
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

            // id: primary_key (WorldSafeLocs) int32
            let id = WorldSafeLocsKey::new(crate::util::read_i32_le(chunk)?);

            // continent: foreign_key (Map) int32
            let continent = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // loc: float[3]
            let loc = crate::util::read_array_f32::<3>(chunk)?;

            // area_name_lang: string_ref_loc (Extended)
            let area_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(WorldSafeLocsRow {
                id,
                continent,
                loc,
                area_name_lang,
            });
        }

        Ok(WorldSafeLocs { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldSafeLocs) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent: foreign_key (Map) int32
            b.write_all(&(row.continent.id as i32).to_le_bytes())?;

            // loc: float[3]
            for i in row.loc {
                b.write_all(&i.to_le_bytes())?;
            }


            // area_name_lang: string_ref_loc (Extended)
            b.write_all(&row.area_name_lang.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<i32> for WorldSafeLocs {
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
    pub continent: MapKey,
    pub loc: [f32; 3],
    pub area_name_lang: ExtendedLocalizedString,
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
        let mut file = File::open("../tbc-dbc/WorldSafeLocs.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldSafeLocs::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldSafeLocs::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
