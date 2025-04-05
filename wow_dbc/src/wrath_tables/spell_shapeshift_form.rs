use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::creature_type::{
    CreatureType, CreatureTypeKey,
};
use crate::wrath_tables::spell_icon::{
    SpellIcon, SpellIconKey,
};
use std::io::Write;
use super::WrathTable;

pub type SpellShapeshiftFormKey = crate::PrimaryKey<i32, SpellShapeshiftForm>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellShapeshiftForm {
    pub rows: Vec<SpellShapeshiftFormRow>,
}

impl SpellShapeshiftForm {
    pub const FILENAME: &'static str = "SpellShapeshiftForm.dbc";
    pub const FIELD_COUNT: usize = 35;
    pub const ROW_SIZE: usize = 140;

    pub fn verify(&self, creature_type: &CreatureType, spell_icon: &SpellIcon) -> Result<(), crate::InvalidForeignKeyError<&SpellShapeshiftFormRow>> {
        for row in &self.rows {
            if row.creature_type.id != 0 && creature_type.get(&row.creature_type).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellShapeshiftForm>(),
                    row,
                    id,
                    row.creature_type.id.into()
                ));
            }

            if row.attack_icon_id.id != 0 && spell_icon.get(&row.attack_icon_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SpellShapeshiftForm>(),
                    row,
                    id,
                    row.attack_icon_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for SpellShapeshiftForm {
    fn into(self) -> WrathTable {
        WrathTable::SpellShapeshiftForm(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellShapeshiftForm {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellShapeshiftFormRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellShapeshiftFormRow] { &mut self.rows }

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

            // id: primary_key (SpellShapeshiftForm) int32
            let id = SpellShapeshiftFormKey::new(crate::util::read_i32_le(chunk)?);

            // bonus_action_bar: int32
            let bonus_action_bar = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // creature_type: foreign_key (CreatureType) int32
            let creature_type = CreatureTypeKey::new(crate::util::read_i32_le(chunk)?.into());

            // attack_icon_id: foreign_key (SpellIcon) int32
            let attack_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // combat_round_time: int32
            let combat_round_time = crate::util::read_i32_le(chunk)?;

            // creature_display_id: int32[4]
            let creature_display_id = crate::util::read_array_i32::<4>(chunk)?;

            // preset_spell_id: int32[8]
            let preset_spell_id = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(SpellShapeshiftFormRow {
                id,
                bonus_action_bar,
                name_lang,
                flags,
                creature_type,
                attack_icon_id,
                combat_round_time,
                creature_display_id,
                preset_spell_id,
            });
        }

        Ok(SpellShapeshiftForm { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellShapeshiftForm) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // bonus_action_bar: int32
            b.write_all(&row.bonus_action_bar.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // creature_type: foreign_key (CreatureType) int32
            b.write_all(&(row.creature_type.id as i32).to_le_bytes())?;

            // attack_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.attack_icon_id.id as i32).to_le_bytes())?;

            // combat_round_time: int32
            b.write_all(&row.combat_round_time.to_le_bytes())?;

            // creature_display_id: int32[4]
            for i in row.creature_display_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // preset_spell_id: int32[8]
            for i in row.preset_spell_id {
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
impl Indexable<i32> for SpellShapeshiftForm {
    type Table = Self;

    fn get(&self, key: &SpellShapeshiftFormKey) -> Option<&SpellShapeshiftFormRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellShapeshiftFormKey) -> Option<&mut SpellShapeshiftFormRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellShapeshiftFormRow {
    pub id: SpellShapeshiftFormKey,
    pub bonus_action_bar: i32,
    pub name_lang: ExtendedLocalizedString,
    pub flags: i32,
    pub creature_type: CreatureTypeKey,
    pub attack_icon_id: SpellIconKey,
    pub combat_round_time: i32,
    pub creature_display_id: [i32; 4],
    pub preset_spell_id: [i32; 8],
}

impl DbcRow for SpellShapeshiftFormRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_shapeshift_form() {
        let mut file = File::open("../wrath-dbc/SpellShapeshiftForm.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellShapeshiftForm::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellShapeshiftForm::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
