use crate::{
    DbcTable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;
use wow_world_base::vanilla::{
    ServerCategory, ServerRegion,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cfg_Categories {
    pub rows: Vec<Cfg_CategoriesRow>,
}

impl DbcTable for Cfg_Categories {
    type Row = Cfg_CategoriesRow;

    const FILENAME: &'static str = "Cfg_Categories.dbc";
    const FIELD_COUNT: usize = 11;
    const ROW_SIZE: usize = 44;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // category: ServerCategory
            let category = crate::util::read_i32_le(chunk)?.try_into()?;

            // region: ServerRegion
            let region = crate::util::read_i32_le(chunk)?.try_into()?;

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(Cfg_CategoriesRow {
                category,
                region,
                name,
            });
        }

        Ok(Cfg_Categories { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: 44,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // category: ServerCategory
            b.write_all(&(row.category.as_int() as i32).to_le_bytes())?;

            // region: ServerRegion
            b.write_all(&(row.region.as_int() as i32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Cfg_Categories {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cfg_CategoriesRow {
    pub category: ServerCategory,
    pub region: ServerRegion,
    pub name: LocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cfg_categories() {
        let contents = include_bytes!("../../../vanilla-dbc/Cfg_Categories.dbc");
        let actual = Cfg_Categories::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Cfg_Categories::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
