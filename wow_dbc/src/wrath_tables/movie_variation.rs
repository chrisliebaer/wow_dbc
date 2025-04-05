use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::file_data::{
    FileData, FileDataKey,
};
use crate::wrath_tables::movie::{
    Movie, MovieKey,
};
use std::io::Write;
use super::WrathTable;

pub type MovieVariationKey = crate::PrimaryKey<i32, MovieVariation>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MovieVariation {
    pub rows: Vec<MovieVariationRow>,
}

impl MovieVariation {
    pub const FILENAME: &'static str = "MovieVariation.dbc";
    pub const FIELD_COUNT: usize = 3;
    pub const ROW_SIZE: usize = 12;

    pub fn verify(&self, file_data: &FileData, movie: &Movie) -> Result<(), crate::InvalidForeignKeyError<&MovieVariationRow>> {
        for row in &self.rows {
            if row.movie_id.id != 0 && movie.get(&row.movie_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<MovieVariation>(),
                    row,
                    id,
                    row.movie_id.id.into()
                ));
            }

            if row.file_data_id.id != 0 && file_data.get(&row.file_data_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<MovieVariation>(),
                    row,
                    id,
                    row.file_data_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for MovieVariation {
    fn into(self) -> WrathTable {
        WrathTable::MovieVariation(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for MovieVariation {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[MovieVariationRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [MovieVariationRow] { &mut self.rows }

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

            // id: primary_key (MovieVariation) int32
            let id = MovieVariationKey::new(crate::util::read_i32_le(chunk)?);

            // movie_id: foreign_key (Movie) int32
            let movie_id = MovieKey::new(crate::util::read_i32_le(chunk)?.into());

            // file_data_id: foreign_key (FileData) int32
            let file_data_id = FileDataKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(MovieVariationRow {
                id,
                movie_id,
                file_data_id,
            });
        }

        Ok(MovieVariation { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (MovieVariation) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // movie_id: foreign_key (Movie) int32
            b.write_all(&(row.movie_id.id as i32).to_le_bytes())?;

            // file_data_id: foreign_key (FileData) int32
            b.write_all(&(row.file_data_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for MovieVariation {
    type Table = Self;

    fn get(&self, key: &MovieVariationKey) -> Option<&MovieVariationRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &MovieVariationKey) -> Option<&mut MovieVariationRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MovieVariationRow {
    pub id: MovieVariationKey,
    pub movie_id: MovieKey,
    pub file_data_id: FileDataKey,
}

impl DbcRow for MovieVariationRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn movie_variation() {
        let mut file = File::open("../wrath-dbc/MovieVariation.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = MovieVariation::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = MovieVariation::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
