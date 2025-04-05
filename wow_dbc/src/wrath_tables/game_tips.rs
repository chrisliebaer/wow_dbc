use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type GameTipsKey = crate::PrimaryKey<i32, GameTips>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameTips {
    pub rows: Vec<GameTipsRow>,
}

impl GameTips {
    pub const FILENAME: &'static str = "GameTips.dbc";
    pub const FIELD_COUNT: usize = 18;
    pub const ROW_SIZE: usize = 72;

}

impl Into<WrathTable> for GameTips {
    fn into(self) -> WrathTable {
        WrathTable::GameTips(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GameTips {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GameTipsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GameTipsRow] { &mut self.rows }

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

            // id: primary_key (GameTips) int32
            let id = GameTipsKey::new(crate::util::read_i32_le(chunk)?);

            // text_lang: string_ref_loc (Extended)
            let text_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(GameTipsRow {
                id,
                text_lang,
            });
        }

        Ok(GameTips { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GameTips) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // text_lang: string_ref_loc (Extended)
            b.write_all(&row.text_lang.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<i32> for GameTips {
    type Table = Self;

    fn get(&self, key: &GameTipsKey) -> Option<&GameTipsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GameTipsKey) -> Option<&mut GameTipsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameTipsRow {
    pub id: GameTipsKey,
    pub text_lang: ExtendedLocalizedString,
}

impl DbcRow for GameTipsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn game_tips() {
        let mut file = File::open("../wrath-dbc/GameTips.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GameTips::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GameTips::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
