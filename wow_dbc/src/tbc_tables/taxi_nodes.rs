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

pub type TaxiNodesKey = crate::PrimaryKey<i32, TaxiNodes>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiNodes {
    pub rows: Vec<TaxiNodesRow>,
}

impl TaxiNodes {
    pub const FILENAME: &'static str = "TaxiNodes.dbc";
    pub const FIELD_COUNT: usize = 24;
    pub const ROW_SIZE: usize = 96;

    pub fn verify(&self, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&TaxiNodesRow>> {
        for row in &self.rows {
            if row.continent_id.id != 0 && map.get(&row.continent_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TaxiNodes>(),
                    row,
                    id,
                    row.continent_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for TaxiNodes {
    fn into(self) -> TbcTable {
        TbcTable::TaxiNodes(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TaxiNodes {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TaxiNodesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TaxiNodesRow] { &mut self.rows }

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

            // id: primary_key (TaxiNodes) int32
            let id = TaxiNodesKey::new(crate::util::read_i32_le(chunk)?);

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // pos: float[3]
            let pos = crate::util::read_array_f32::<3>(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // mount_creature_id: int32[2]
            let mount_creature_id = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(TaxiNodesRow {
                id,
                continent_id,
                pos,
                name_lang,
                mount_creature_id,
            });
        }

        Ok(TaxiNodes { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TaxiNodes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // pos: float[3]
            for i in row.pos {
                b.write_all(&i.to_le_bytes())?;
            }


            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // mount_creature_id: int32[2]
            for i in row.mount_creature_id {
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
impl Indexable<i32> for TaxiNodes {
    type Table = Self;

    fn get(&self, key: &TaxiNodesKey) -> Option<&TaxiNodesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TaxiNodesKey) -> Option<&mut TaxiNodesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiNodesRow {
    pub id: TaxiNodesKey,
    pub continent_id: MapKey,
    pub pos: [f32; 3],
    pub name_lang: ExtendedLocalizedString,
    pub mount_creature_id: [i32; 2],
}

impl DbcRow for TaxiNodesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn taxi_nodes() {
        let mut file = File::open("../tbc-dbc/TaxiNodes.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TaxiNodes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TaxiNodes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
