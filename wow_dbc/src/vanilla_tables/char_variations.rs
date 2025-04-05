use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::Gender;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharVariations {
    pub rows: Vec<CharVariationsRow>,
}

impl CharVariations {
    pub const FILENAME: &'static str = "CharVariations.dbc";
    pub const FIELD_COUNT: usize = 6;
    pub const ROW_SIZE: usize = 24;

    pub fn verify(&self, chr_races: &ChrRaces) -> Result<(), crate::InvalidForeignKeyError<&CharVariationsRow>> {
        for row in &self.rows {
            if row.id.id != 0 && chr_races.get(&row.id).is_none() {
                let id = None;
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CharVariations>(),
                    row,
                    id,
                    row.id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CharVariations {
    fn into(self) -> VanillaTable {
        VanillaTable::CharVariations(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CharVariations {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CharVariationsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CharVariationsRow] { &mut self.rows }

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

            // id: foreign_key (ChrRaces) uint32
            let id = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i32_le(chunk)?.try_into()?;

            // unknown_1: int32
            let unknown_1 = crate::util::read_i32_le(chunk)?;

            // mask: int32[2]
            let mask = crate::util::read_array_i32::<2>(chunk)?;

            // unknown_2: int32
            let unknown_2 = crate::util::read_i32_le(chunk)?;


            rows.push(CharVariationsRow {
                id,
                gender,
                unknown_1,
                mask,
                unknown_2,
            });
        }

        Ok(CharVariations { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: foreign_key (ChrRaces) uint32
            b.write_all(&(row.id.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // unknown_1: int32
            b.write_all(&row.unknown_1.to_le_bytes())?;

            // mask: int32[2]
            for i in row.mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // unknown_2: int32
            b.write_all(&row.unknown_2.to_le_bytes())?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharVariationsRow {
    pub id: ChrRacesKey,
    pub gender: Gender,
    pub unknown_1: i32,
    pub mask: [i32; 2],
    pub unknown_2: i32,
}

impl DbcRow for CharVariationsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_variations() {
        let mut file = File::open("../vanilla-dbc/CharVariations.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharVariations::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharVariations::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
