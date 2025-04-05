use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::declined_word::{
    DeclinedWord, DeclinedWordKey,
};
use std::io::Write;
use super::WrathTable;

pub type DeclinedWordCasesKey = crate::PrimaryKey<i32, DeclinedWordCases>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeclinedWordCases {
    pub rows: Vec<DeclinedWordCasesRow>,
}

impl DeclinedWordCases {
    pub const FILENAME: &'static str = "DeclinedWordCases.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, declined_word: &DeclinedWord) -> Result<(), crate::InvalidForeignKeyError<&DeclinedWordCasesRow>> {
        for row in &self.rows {
            if row.declined_word_id.id != 0 && declined_word.get(&row.declined_word_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<DeclinedWordCases>(),
                    row,
                    id,
                    row.declined_word_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for DeclinedWordCases {
    fn into(self) -> WrathTable {
        WrathTable::DeclinedWordCases(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for DeclinedWordCases {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[DeclinedWordCasesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [DeclinedWordCasesRow] { &mut self.rows }

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

            // id: primary_key (DeclinedWordCases) int32
            let id = DeclinedWordCasesKey::new(crate::util::read_i32_le(chunk)?);

            // declined_word_id: foreign_key (DeclinedWord) int32
            let declined_word_id = DeclinedWordKey::new(crate::util::read_i32_le(chunk)?.into());

            // case_index: int32
            let case_index = crate::util::read_i32_le(chunk)?;

            // declined_word: string_ref
            let declined_word = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(DeclinedWordCasesRow {
                id,
                declined_word_id,
                case_index,
                declined_word,
            });
        }

        Ok(DeclinedWordCases { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (DeclinedWordCases) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // declined_word_id: foreign_key (DeclinedWord) int32
            b.write_all(&(row.declined_word_id.id as i32).to_le_bytes())?;

            // case_index: int32
            b.write_all(&row.case_index.to_le_bytes())?;

            // declined_word: string_ref
            b.write_all(&string_cache.add_string(&row.declined_word).to_le_bytes())?;

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
impl Indexable<i32> for DeclinedWordCases {
    type Table = Self;

    fn get(&self, key: &DeclinedWordCasesKey) -> Option<&DeclinedWordCasesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &DeclinedWordCasesKey) -> Option<&mut DeclinedWordCasesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeclinedWordCasesRow {
    pub id: DeclinedWordCasesKey,
    pub declined_word_id: DeclinedWordKey,
    pub case_index: i32,
    pub declined_word: String,
}

impl DbcRow for DeclinedWordCasesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn declined_word_cases() {
        let mut file = File::open("../wrath-dbc/DeclinedWordCases.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DeclinedWordCases::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DeclinedWordCases::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
