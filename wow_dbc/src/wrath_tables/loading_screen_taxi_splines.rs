use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::taxi_path::{
    TaxiPath, TaxiPathKey,
};
use std::io::Write;
use super::WrathTable;

pub type LoadingScreenTaxiSplinesKey = crate::PrimaryKey<i32, LoadingScreenTaxiSplines>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoadingScreenTaxiSplines {
    pub rows: Vec<LoadingScreenTaxiSplinesRow>,
}

impl LoadingScreenTaxiSplines {
    pub const FILENAME: &'static str = "LoadingScreenTaxiSplines.dbc";
    pub const FIELD_COUNT: usize = 19;
    pub const ROW_SIZE: usize = 76;

    pub fn verify(&self, taxi_path: &TaxiPath) -> Result<(), crate::InvalidForeignKeyError<&LoadingScreenTaxiSplinesRow>> {
        for row in &self.rows {
            if row.path_id.id != 0 && taxi_path.get(&row.path_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<LoadingScreenTaxiSplines>(),
                    row,
                    id,
                    row.path_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for LoadingScreenTaxiSplines {
    fn into(self) -> WrathTable {
        WrathTable::LoadingScreenTaxiSplines(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for LoadingScreenTaxiSplines {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[LoadingScreenTaxiSplinesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [LoadingScreenTaxiSplinesRow] { &mut self.rows }

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

            // id: primary_key (LoadingScreenTaxiSplines) int32
            let id = LoadingScreenTaxiSplinesKey::new(crate::util::read_i32_le(chunk)?);

            // path_id: foreign_key (TaxiPath) int32
            let path_id = TaxiPathKey::new(crate::util::read_i32_le(chunk)?.into());

            // locx: float[8]
            let locx = crate::util::read_array_f32::<8>(chunk)?;

            // locy: float[8]
            let locy = crate::util::read_array_f32::<8>(chunk)?;

            // leg_index: int32
            let leg_index = crate::util::read_i32_le(chunk)?;


            rows.push(LoadingScreenTaxiSplinesRow {
                id,
                path_id,
                locx,
                locy,
                leg_index,
            });
        }

        Ok(LoadingScreenTaxiSplines { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LoadingScreenTaxiSplines) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // path_id: foreign_key (TaxiPath) int32
            b.write_all(&(row.path_id.id as i32).to_le_bytes())?;

            // locx: float[8]
            for i in row.locx {
                b.write_all(&i.to_le_bytes())?;
            }


            // locy: float[8]
            for i in row.locy {
                b.write_all(&i.to_le_bytes())?;
            }


            // leg_index: int32
            b.write_all(&row.leg_index.to_le_bytes())?;

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
impl Indexable<i32> for LoadingScreenTaxiSplines {
    type Table = Self;

    fn get(&self, key: &LoadingScreenTaxiSplinesKey) -> Option<&LoadingScreenTaxiSplinesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &LoadingScreenTaxiSplinesKey) -> Option<&mut LoadingScreenTaxiSplinesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoadingScreenTaxiSplinesRow {
    pub id: LoadingScreenTaxiSplinesKey,
    pub path_id: TaxiPathKey,
    pub locx: [f32; 8],
    pub locy: [f32; 8],
    pub leg_index: i32,
}

impl DbcRow for LoadingScreenTaxiSplinesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn loading_screen_taxi_splines() {
        let mut file = File::open("../wrath-dbc/LoadingScreenTaxiSplines.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LoadingScreenTaxiSplines::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LoadingScreenTaxiSplines::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
