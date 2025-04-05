use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;

pub type CharacterCreateCamerasKey = crate::PrimaryKey<u32, CharacterCreateCameras>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharacterCreateCameras {
    pub rows: Vec<CharacterCreateCamerasRow>,
}

impl CharacterCreateCameras {
    pub const FILENAME: &'static str = "CharacterCreateCameras.dbc";
    pub const FIELD_COUNT: usize = 6;
    pub const ROW_SIZE: usize = 24;

}

impl Into<VanillaTable> for CharacterCreateCameras {
    fn into(self) -> VanillaTable {
        VanillaTable::CharacterCreateCameras(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CharacterCreateCameras {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CharacterCreateCamerasRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CharacterCreateCamerasRow] { &mut self.rows }

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

            // id: primary_key (CharacterCreateCameras) uint32
            let id = CharacterCreateCamerasKey::new(crate::util::read_u32_le(chunk)?);

            // unknown: bool32[2]
            let unknown = {
                let mut arr = [bool::default(); 2];
                for i in arr.iter_mut() {
                    *i = crate::util::read_u32_le(chunk)? != 0;
                }

                arr
            };

            // unknown_2: float[3]
            let unknown_2 = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(CharacterCreateCamerasRow {
                id,
                unknown,
                unknown_2,
            });
        }

        Ok(CharacterCreateCameras { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CharacterCreateCameras) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // unknown: bool32[2]
            for i in row.unknown {
                b.write_all(&u32::from(i).to_le_bytes())?;
            }


            // unknown_2: float[3]
            for i in row.unknown_2 {
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
impl Indexable<u32> for CharacterCreateCameras {
    type Table = Self;

    fn get(&self, key: &CharacterCreateCamerasKey) -> Option<&CharacterCreateCamerasRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CharacterCreateCamerasKey) -> Option<&mut CharacterCreateCamerasRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharacterCreateCamerasRow {
    pub id: CharacterCreateCamerasKey,
    pub unknown: [bool; 2],
    pub unknown_2: [f32; 3],
}

impl DbcRow for CharacterCreateCamerasRow {
}

