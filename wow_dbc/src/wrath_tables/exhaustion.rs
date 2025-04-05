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

pub type ExhaustionKey = crate::PrimaryKey<i32, Exhaustion>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exhaustion {
    pub rows: Vec<ExhaustionRow>,
}

impl Exhaustion {
    pub const FILENAME: &'static str = "Exhaustion.dbc";
    pub const FIELD_COUNT: usize = 23;
    pub const ROW_SIZE: usize = 92;

}

impl Into<WrathTable> for Exhaustion {
    fn into(self) -> WrathTable {
        WrathTable::Exhaustion(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Exhaustion {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ExhaustionRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ExhaustionRow] { &mut self.rows }

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

            // id: primary_key (Exhaustion) int32
            let id = ExhaustionKey::new(crate::util::read_i32_le(chunk)?);

            // xp: int32
            let xp = crate::util::read_i32_le(chunk)?;

            // factor: float
            let factor = crate::util::read_f32_le(chunk)?;

            // outdoor_hours: float
            let outdoor_hours = crate::util::read_f32_le(chunk)?;

            // inn_hours: float
            let inn_hours = crate::util::read_f32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // threshold: float
            let threshold = crate::util::read_f32_le(chunk)?;


            rows.push(ExhaustionRow {
                id,
                xp,
                factor,
                outdoor_hours,
                inn_hours,
                name_lang,
                threshold,
            });
        }

        Ok(Exhaustion { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Exhaustion) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // xp: int32
            b.write_all(&row.xp.to_le_bytes())?;

            // factor: float
            b.write_all(&row.factor.to_le_bytes())?;

            // outdoor_hours: float
            b.write_all(&row.outdoor_hours.to_le_bytes())?;

            // inn_hours: float
            b.write_all(&row.inn_hours.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // threshold: float
            b.write_all(&row.threshold.to_le_bytes())?;

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
impl Indexable<i32> for Exhaustion {
    type Table = Self;

    fn get(&self, key: &ExhaustionKey) -> Option<&ExhaustionRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ExhaustionKey) -> Option<&mut ExhaustionRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExhaustionRow {
    pub id: ExhaustionKey,
    pub xp: i32,
    pub factor: f32,
    pub outdoor_hours: f32,
    pub inn_hours: f32,
    pub name_lang: ExtendedLocalizedString,
    pub threshold: f32,
}

impl DbcRow for ExhaustionRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn exhaustion() {
        let mut file = File::open("../wrath-dbc/Exhaustion.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Exhaustion::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Exhaustion::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
