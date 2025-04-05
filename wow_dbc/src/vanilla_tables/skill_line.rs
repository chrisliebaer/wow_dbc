use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::skill_costs_data::{
    SkillCostsData, SkillCostsDataKey,
};
use crate::vanilla_tables::skill_line_category::{
    SkillLineCategory, SkillLineCategoryKey,
};
use crate::vanilla_tables::spell_icon::{
    SpellIcon, SpellIconKey,
};
use std::io::Write;
use super::VanillaTable;

pub type SkillLineKey = crate::PrimaryKey<u32, SkillLine>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLine {
    pub rows: Vec<SkillLineRow>,
}

impl SkillLine {
    pub const FILENAME: &'static str = "SkillLine.dbc";
    pub const FIELD_COUNT: usize = 22;
    pub const ROW_SIZE: usize = 88;

    pub fn verify(&self, skill_costs_data: &SkillCostsData, skill_line_category: &SkillLineCategory, spell_icon: &SpellIcon) -> Result<(), crate::InvalidForeignKeyError<&SkillLineRow>> {
        for row in &self.rows {
            if row.category.id != 0 && skill_line_category.get(&row.category).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLine>(),
                    row,
                    id,
                    row.category.id.into()
                ));
            }

            if row.skill_costs.id != 0 && skill_costs_data.get(&row.skill_costs).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLine>(),
                    row,
                    id,
                    row.skill_costs.id.into()
                ));
            }

            if row.spell_icon.id != 0 && spell_icon.get(&row.spell_icon).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLine>(),
                    row,
                    id,
                    row.spell_icon.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for SkillLine {
    fn into(self) -> VanillaTable {
        VanillaTable::SkillLine(self)
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

            // id: primary_key (SkillLine) uint32
            let id = SkillLineKey::new(crate::util::read_u32_le(chunk)?);

            // category: foreign_key (SkillLineCategory) uint32
            let category = SkillLineCategoryKey::new(crate::util::read_u32_le(chunk)?.into());

            // skill_costs: foreign_key (SkillCostsData) uint32
            let skill_costs = SkillCostsDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // display_name: string_ref_loc
            let display_name = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SkillLineRow {
                id,
                category,
                skill_costs,
                display_name,
                description,
                spell_icon,
            });
        }

        Ok(SkillLine { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SkillLine) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // category: foreign_key (SkillLineCategory) uint32
            b.write_all(&(row.category.id as u32).to_le_bytes())?;

            // skill_costs: foreign_key (SkillCostsData) uint32
            b.write_all(&(row.skill_costs.id as u32).to_le_bytes())?;

            // display_name: string_ref_loc
            b.write_all(&row.display_name.string_indices_as_array(&mut string_cache))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_cache))?;

            // spell_icon: foreign_key (SpellIcon) uint32
            b.write_all(&(row.spell_icon.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for SkillLine {
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
    pub category: SkillLineCategoryKey,
    pub skill_costs: SkillCostsDataKey,
    pub display_name: LocalizedString,
    pub description: LocalizedString,
    pub spell_icon: SpellIconKey,
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
        let mut file = File::open("../vanilla-dbc/SkillLine.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SkillLine::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SkillLine::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
