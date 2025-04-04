use crate::DbcTable;
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct gtChanceToSpellCritBase {
    pub rows: Vec<gtChanceToSpellCritBaseRow>,
}

impl DbcTable for gtChanceToSpellCritBase {
    type Row = gtChanceToSpellCritBaseRow;

    const FILENAME: &'static str = "gtChanceToSpellCritBase.dbc";
    const FIELD_COUNT: usize = 1;
    const ROW_SIZE: usize = 4;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

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

            // data: float
            let data = crate::util::read_f32_le(chunk)?;


            rows.push(gtChanceToSpellCritBaseRow {
                data,
            });
        }

        Ok(gtChanceToSpellCritBase { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: 4,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // data: float
            b.write_all(&row.data.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct gtChanceToSpellCritBaseRow {
    pub data: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gt_chance_to_spell_crit_base() {
        let contents = include_bytes!("../../../wrath-dbc/gtChanceToSpellCritBase.dbc");
        let actual = gtChanceToSpellCritBase::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = gtChanceToSpellCritBase::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
