use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::cinematic_sequences::{
    CinematicSequences, CinematicSequencesKey,
};
use std::io::Write;
use super::WrathTable;

pub type ChrClassesKey = crate::PrimaryKey<i32, ChrClasses>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrClasses {
    pub rows: Vec<ChrClassesRow>,
}

impl ChrClasses {
    pub const FILENAME: &'static str = "ChrClasses.dbc";
    pub const FIELD_COUNT: usize = 60;
    pub const ROW_SIZE: usize = 240;

    pub fn verify(&self, cinematic_sequences: &CinematicSequences) -> Result<(), crate::InvalidForeignKeyError<&ChrClassesRow>> {
        for row in &self.rows {
            if row.cinematic_sequence_id.id != 0 && cinematic_sequences.get(&row.cinematic_sequence_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ChrClasses>(),
                    row,
                    id,
                    row.cinematic_sequence_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for ChrClasses {
    fn into(self) -> WrathTable {
        WrathTable::ChrClasses(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ChrClasses {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ChrClassesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ChrClassesRow] { &mut self.rows }

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

            // id: primary_key (ChrClasses) int32
            let id = ChrClassesKey::new(crate::util::read_i32_le(chunk)?);

            // damage_bonus_stat: int32
            let damage_bonus_stat = crate::util::read_i32_le(chunk)?;

            // display_power: foreign_key (PowerType) int32
            let display_power = crate::util::read_i32_le(chunk)?;

            // pet_name_token: string_ref
            let pet_name_token = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_female_lang: string_ref_loc (Extended)
            let name_female_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_male_lang: string_ref_loc (Extended)
            let name_male_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // filename: string_ref
            let filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // spell_class_set: int32
            let spell_class_set = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            let cinematic_sequence_id = CinematicSequencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // required_expansion: int32
            let required_expansion = crate::util::read_i32_le(chunk)?;


            rows.push(ChrClassesRow {
                id,
                damage_bonus_stat,
                display_power,
                pet_name_token,
                name_lang,
                name_female_lang,
                name_male_lang,
                filename,
                spell_class_set,
                flags,
                cinematic_sequence_id,
                required_expansion,
            });
        }

        Ok(ChrClasses { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ChrClasses) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // damage_bonus_stat: int32
            b.write_all(&row.damage_bonus_stat.to_le_bytes())?;

            // display_power: foreign_key (PowerType) int32
            b.write_all(&row.display_power.to_le_bytes())?;

            // pet_name_token: string_ref
            b.write_all(&string_cache.add_string(&row.pet_name_token).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // name_female_lang: string_ref_loc (Extended)
            b.write_all(&row.name_female_lang.string_indices_as_array(&mut string_cache))?;

            // name_male_lang: string_ref_loc (Extended)
            b.write_all(&row.name_male_lang.string_indices_as_array(&mut string_cache))?;

            // filename: string_ref
            b.write_all(&string_cache.add_string(&row.filename).to_le_bytes())?;

            // spell_class_set: int32
            b.write_all(&row.spell_class_set.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            b.write_all(&(row.cinematic_sequence_id.id as i32).to_le_bytes())?;

            // required_expansion: int32
            b.write_all(&row.required_expansion.to_le_bytes())?;

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
impl Indexable<i32> for ChrClasses {
    type Table = Self;

    fn get(&self, key: &ChrClassesKey) -> Option<&ChrClassesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ChrClassesKey) -> Option<&mut ChrClassesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrClassesRow {
    pub id: ChrClassesKey,
    pub damage_bonus_stat: i32,
    pub display_power: i32,
    pub pet_name_token: String,
    pub name_lang: ExtendedLocalizedString,
    pub name_female_lang: ExtendedLocalizedString,
    pub name_male_lang: ExtendedLocalizedString,
    pub filename: String,
    pub spell_class_set: i32,
    pub flags: i32,
    pub cinematic_sequence_id: CinematicSequencesKey,
    pub required_expansion: i32,
}

impl DbcRow for ChrClassesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn chr_classes() {
        let mut file = File::open("../wrath-dbc/ChrClasses.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ChrClasses::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ChrClasses::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
