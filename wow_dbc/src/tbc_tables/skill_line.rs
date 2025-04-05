use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::skill_line_category::{
    SkillLineCategory, SkillLineCategoryKey,
};
use crate::tbc_tables::spell_icon::{
    SpellIcon, SpellIconKey,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type SkillLineKey = crate::PrimaryKey<i32, SkillLine>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLine {
    pub rows: Vec<SkillLineRow>,
}

impl SkillLine {
    pub const FILENAME: &'static str = "SkillLine.dbc";
    pub const FIELD_COUNT: usize = 38;
    pub const ROW_SIZE: usize = 152;

    pub fn verify(&self, skill_line_category: &SkillLineCategory, spell_icon: &SpellIcon) -> Result<(), crate::InvalidForeignKeyError<&SkillLineRow>> {
        for row in &self.rows {
            if row.category_id.id != 0 && skill_line_category.get(&row.category_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLine>(),
                    row,
                    id,
                    row.category_id.id.into()
                ));
            }

            if row.spell_icon_id.id != 0 && spell_icon.get(&row.spell_icon_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLine>(),
                    row,
                    id,
                    row.spell_icon_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for SkillLine {
    fn into(self) -> TbcTable {
        TbcTable::SkillLine(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SkillLine {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SkillLineRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SkillLineRow] { &mut self.rows }

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

            // id: primary_key (SkillLine) int32
            let id = SkillLineKey::new(crate::util::read_i32_le(chunk)?);

            // category_id: foreign_key (SkillLineCategory) int32
            let category_id = SkillLineCategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // skill_costs_id: int32
            let skill_costs_id = crate::util::read_i32_le(chunk)?;

            // display_name_lang: string_ref_loc (Extended)
            let display_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(SkillLineRow {
                id,
                category_id,
                skill_costs_id,
                display_name_lang,
                description_lang,
                spell_icon_id,
            });
        }

        Ok(SkillLine { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SkillLine) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // category_id: foreign_key (SkillLineCategory) int32
            b.write_all(&(row.category_id.id as i32).to_le_bytes())?;

            // skill_costs_id: int32
            b.write_all(&row.skill_costs_id.to_le_bytes())?;

            // display_name_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_lang.string_indices_as_array(&mut string_cache))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.spell_icon_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for SkillLine {
    type Table = Self;

    fn get(&self, key: &SkillLineKey) -> Option<&SkillLineRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SkillLineKey) -> Option<&mut SkillLineRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLineRow {
    pub id: SkillLineKey,
    pub category_id: SkillLineCategoryKey,
    pub skill_costs_id: i32,
    pub display_name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub spell_icon_id: SpellIconKey,
}

impl DbcRow for SkillLineRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn skill_line() {
        let mut file = File::open("../tbc-dbc/SkillLine.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SkillLine::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SkillLine::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
