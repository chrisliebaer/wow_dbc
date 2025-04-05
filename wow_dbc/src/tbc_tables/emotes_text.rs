use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::emotes::{
    Emotes, EmotesKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type EmotesTextKey = crate::PrimaryKey<i32, EmotesText>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesText {
    pub rows: Vec<EmotesTextRow>,
}

impl EmotesText {
    pub const FILENAME: &'static str = "EmotesText.dbc";
    pub const FIELD_COUNT: usize = 19;
    pub const ROW_SIZE: usize = 76;

    pub fn verify(&self, emotes: &Emotes) -> Result<(), crate::InvalidForeignKeyError<&EmotesTextRow>> {
        for row in &self.rows {
            if row.emote_id.id != 0 && emotes.get(&row.emote_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<EmotesText>(),
                    row,
                    id,
                    row.emote_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for EmotesText {
    fn into(self) -> TbcTable {
        TbcTable::EmotesText(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for EmotesText {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[EmotesTextRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [EmotesTextRow] { &mut self.rows }

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

            // id: primary_key (EmotesText) int32
            let id = EmotesTextKey::new(crate::util::read_i32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // emote_id: foreign_key (Emotes) int32
            let emote_id = EmotesKey::new(crate::util::read_i32_le(chunk)?.into());

            // emote_text: int32[16]
            let emote_text = crate::util::read_array_i32::<16>(chunk)?;


            rows.push(EmotesTextRow {
                id,
                name,
                emote_id,
                emote_text,
            });
        }

        Ok(EmotesText { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (EmotesText) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // emote_id: foreign_key (Emotes) int32
            b.write_all(&(row.emote_id.id as i32).to_le_bytes())?;

            // emote_text: int32[16]
            for i in row.emote_text {
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
impl Indexable<i32> for EmotesText {
    type Table = Self;

    fn get(&self, key: &EmotesTextKey) -> Option<&EmotesTextRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &EmotesTextKey) -> Option<&mut EmotesTextRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesTextRow {
    pub id: EmotesTextKey,
    pub name: String,
    pub emote_id: EmotesKey,
    pub emote_text: [i32; 16],
}

impl DbcRow for EmotesTextRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn emotes_text() {
        let mut file = File::open("../tbc-dbc/EmotesText.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = EmotesText::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = EmotesText::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
