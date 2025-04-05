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

pub type PetPersonalityKey = crate::PrimaryKey<i32, PetPersonality>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PetPersonality {
    pub rows: Vec<PetPersonalityRow>,
}

impl PetPersonality {
    pub const FILENAME: &'static str = "PetPersonality.dbc";
    pub const FIELD_COUNT: usize = 24;
    pub const ROW_SIZE: usize = 96;

}

impl Into<WrathTable> for PetPersonality {
    fn into(self) -> WrathTable {
        WrathTable::PetPersonality(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for PetPersonality {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[PetPersonalityRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [PetPersonalityRow] { &mut self.rows }

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

            // id: primary_key (PetPersonality) int32
            let id = PetPersonalityKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // happiness_threshold: int32[3]
            let happiness_threshold = crate::util::read_array_i32::<3>(chunk)?;

            // happiness_damage: float[3]
            let happiness_damage = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(PetPersonalityRow {
                id,
                name_lang,
                happiness_threshold,
                happiness_damage,
            });
        }

        Ok(PetPersonality { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (PetPersonality) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // happiness_threshold: int32[3]
            for i in row.happiness_threshold {
                b.write_all(&i.to_le_bytes())?;
            }


            // happiness_damage: float[3]
            for i in row.happiness_damage {
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
impl Indexable<i32> for PetPersonality {
    type Table = Self;

    fn get(&self, key: &PetPersonalityKey) -> Option<&PetPersonalityRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &PetPersonalityKey) -> Option<&mut PetPersonalityRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name_lang: ExtendedLocalizedString,
    pub happiness_threshold: [i32; 3],
    pub happiness_damage: [f32; 3],
}

impl DbcRow for PetPersonalityRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn pet_personality() {
        let mut file = File::open("../wrath-dbc/PetPersonality.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = PetPersonality::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = PetPersonality::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
