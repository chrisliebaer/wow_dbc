use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::item_visuals::{
    ItemVisuals, ItemVisualsKey,
};
use crate::wrath_tables::skill_line::{
    SkillLine, SkillLineKey,
};
use crate::wrath_tables::spell_item_enchantment_condition::{
    SpellItemEnchantmentCondition, SpellItemEnchantmentConditionKey,
};
use std::io::Write;
use super::WrathTable;

pub type SpellItemEnchantmentKey = crate::PrimaryKey<i32, SpellItemEnchantment>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellItemEnchantment {
    pub rows: Vec<SpellItemEnchantmentRow>,
}

impl SpellItemEnchantment {
    pub const FILENAME: &'static str = "SpellItemEnchantment.dbc";
    pub const FIELD_COUNT: usize = 38;
    pub const ROW_SIZE: usize = 152;

    pub fn verify(&self, item_visuals: &ItemVisuals, skill_line: &SkillLine, spell_item_enchantment_condition: &SpellItemEnchantmentCondition) -> Result<(), crate::InvalidForeignKeyError<&SpellItemEnchantmentRow>> {
        for row in &self.rows {
            if row.item_visual.id != 0 && item_visuals.get(&row.item_visual).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellItemEnchantment>(),
                    row,
                    id,
                    row.item_visual.id.into()
                ));
            }

            if row.condition_id.id != 0 && spell_item_enchantment_condition.get(&row.condition_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellItemEnchantment>(),
                    row,
                    id,
                    row.condition_id.id.into()
                ));
            }

            if row.required_skill_id.id != 0 && skill_line.get(&row.required_skill_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellItemEnchantment>(),
                    row,
                    id,
                    row.required_skill_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for SpellItemEnchantment {
    fn into(self) -> WrathTable {
        WrathTable::SpellItemEnchantment(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellItemEnchantment {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellItemEnchantmentRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellItemEnchantmentRow] { &mut self.rows }

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

            // id: primary_key (SpellItemEnchantment) int32
            let id = SpellItemEnchantmentKey::new(crate::util::read_i32_le(chunk)?);

            // charges: int32
            let charges = crate::util::read_i32_le(chunk)?;

            // effect: int32[3]
            let effect = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_min: int32[3]
            let effect_points_min = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_max: int32[3]
            let effect_points_max = crate::util::read_array_i32::<3>(chunk)?;

            // effect_arg: int32[3]
            let effect_arg = crate::util::read_array_i32::<3>(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // item_visual: foreign_key (ItemVisuals) int32
            let item_visual = ItemVisualsKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // src_item_id: int32
            let src_item_id = crate::util::read_i32_le(chunk)?;

            // condition_id: foreign_key (SpellItemEnchantmentCondition) int32
            let condition_id = SpellItemEnchantmentConditionKey::new(crate::util::read_i32_le(chunk)?.into());

            // required_skill_id: foreign_key (SkillLine) int32
            let required_skill_id = SkillLineKey::new(crate::util::read_i32_le(chunk)?.into());

            // required_skill_rank: int32
            let required_skill_rank = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;


            rows.push(SpellItemEnchantmentRow {
                id,
                charges,
                effect,
                effect_points_min,
                effect_points_max,
                effect_arg,
                name_lang,
                item_visual,
                flags,
                src_item_id,
                condition_id,
                required_skill_id,
                required_skill_rank,
                min_level,
            });
        }

        Ok(SpellItemEnchantment { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellItemEnchantment) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // charges: int32
            b.write_all(&row.charges.to_le_bytes())?;

            // effect: int32[3]
            for i in row.effect {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_points_min: int32[3]
            for i in row.effect_points_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_points_max: int32[3]
            for i in row.effect_points_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_arg: int32[3]
            for i in row.effect_arg {
                b.write_all(&i.to_le_bytes())?;
            }


            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // item_visual: foreign_key (ItemVisuals) int32
            b.write_all(&(row.item_visual.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // src_item_id: int32
            b.write_all(&row.src_item_id.to_le_bytes())?;

            // condition_id: foreign_key (SpellItemEnchantmentCondition) int32
            b.write_all(&(row.condition_id.id as i32).to_le_bytes())?;

            // required_skill_id: foreign_key (SkillLine) int32
            b.write_all(&(row.required_skill_id.id as i32).to_le_bytes())?;

            // required_skill_rank: int32
            b.write_all(&row.required_skill_rank.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

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
impl Indexable<i32> for SpellItemEnchantment {
    type Table = Self;

    fn get(&self, key: &SpellItemEnchantmentKey) -> Option<&SpellItemEnchantmentRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellItemEnchantmentKey) -> Option<&mut SpellItemEnchantmentRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellItemEnchantmentRow {
    pub id: SpellItemEnchantmentKey,
    pub charges: i32,
    pub effect: [i32; 3],
    pub effect_points_min: [i32; 3],
    pub effect_points_max: [i32; 3],
    pub effect_arg: [i32; 3],
    pub name_lang: ExtendedLocalizedString,
    pub item_visual: ItemVisualsKey,
    pub flags: i32,
    pub src_item_id: i32,
    pub condition_id: SpellItemEnchantmentConditionKey,
    pub required_skill_id: SkillLineKey,
    pub required_skill_rank: i32,
    pub min_level: i32,
}

impl DbcRow for SpellItemEnchantmentRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_item_enchantment() {
        let mut file = File::open("../wrath-dbc/SpellItemEnchantment.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellItemEnchantment::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellItemEnchantment::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
