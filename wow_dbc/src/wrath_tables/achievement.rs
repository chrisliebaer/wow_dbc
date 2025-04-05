use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::achievement_category::{
    Achievement_Category, Achievement_CategoryKey,
};
use crate::wrath_tables::faction::{
    Faction, FactionKey,
};
use crate::wrath_tables::map::{
    Map, MapKey,
};
use crate::wrath_tables::spell_icon::{
    SpellIcon, SpellIconKey,
};
use std::io::Write;
use super::WrathTable;

pub type AchievementKey = crate::PrimaryKey<i32, Achievement>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement {
    pub rows: Vec<AchievementRow>,
}

impl Achievement {
    pub const FILENAME: &'static str = "Achievement.dbc";
    pub const FIELD_COUNT: usize = 62;
    pub const ROW_SIZE: usize = 248;

    pub fn verify(&self, achievement_category: &Achievement_Category, faction: &Faction, map: &Map, spell_icon: &SpellIcon) -> Result<(), crate::InvalidForeignKeyError<&AchievementRow>> {
        for row in &self.rows {
            if row.faction.id != 0 && faction.get(&row.faction).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement>(),
                    row,
                    id,
                    row.faction.id.into()
                ));
            }

            if row.instance_id.id != 0 && map.get(&row.instance_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement>(),
                    row,
                    id,
                    row.instance_id.id.into()
                ));
            }

            if row.supercedes.id != 0 && self.get(&row.supercedes).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement>(),
                    row,
                    id,
                    row.supercedes.id.into()
                ));
            }

            if row.category.id != 0 && achievement_category.get(&row.category).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement>(),
                    row,
                    id,
                    row.category.id.into()
                ));
            }

            if row.icon_id.id != 0 && spell_icon.get(&row.icon_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement>(),
                    row,
                    id,
                    row.icon_id.id.into()
                ));
            }

            if row.shares_criteria.id != 0 && self.get(&row.shares_criteria).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Achievement>(),
                    row,
                    id,
                    row.shares_criteria.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for Achievement {
    fn into(self) -> WrathTable {
        WrathTable::Achievement(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Achievement {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AchievementRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AchievementRow] { &mut self.rows }

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

            // id: primary_key (Achievement) int32
            let id = AchievementKey::new(crate::util::read_i32_le(chunk)?);

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // instance_id: foreign_key (Map) int32
            let instance_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // supercedes: foreign_key (Achievement) int32
            let supercedes = AchievementKey::new(crate::util::read_i32_le(chunk)?.into());

            // title_lang: string_ref_loc (Extended)
            let title_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // category: foreign_key (Achievement_Category) int32
            let category = Achievement_CategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // points: int32
            let points = crate::util::read_i32_le(chunk)?;

            // ui_order: int32
            let ui_order = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // icon_id: foreign_key (SpellIcon) int32
            let icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // reward_lang: string_ref_loc (Extended)
            let reward_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // minimum_criteria: int32
            let minimum_criteria = crate::util::read_i32_le(chunk)?;

            // shares_criteria: foreign_key (Achievement) int32
            let shares_criteria = AchievementKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(AchievementRow {
                id,
                faction,
                instance_id,
                supercedes,
                title_lang,
                description_lang,
                category,
                points,
                ui_order,
                flags,
                icon_id,
                reward_lang,
                minimum_criteria,
                shares_criteria,
            });
        }

        Ok(Achievement { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Achievement) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // instance_id: foreign_key (Map) int32
            b.write_all(&(row.instance_id.id as i32).to_le_bytes())?;

            // supercedes: foreign_key (Achievement) int32
            b.write_all(&(row.supercedes.id as i32).to_le_bytes())?;

            // title_lang: string_ref_loc (Extended)
            b.write_all(&row.title_lang.string_indices_as_array(&mut string_cache))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

            // category: foreign_key (Achievement_Category) int32
            b.write_all(&(row.category.id as i32).to_le_bytes())?;

            // points: int32
            b.write_all(&row.points.to_le_bytes())?;

            // ui_order: int32
            b.write_all(&row.ui_order.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.icon_id.id as i32).to_le_bytes())?;

            // reward_lang: string_ref_loc (Extended)
            b.write_all(&row.reward_lang.string_indices_as_array(&mut string_cache))?;

            // minimum_criteria: int32
            b.write_all(&row.minimum_criteria.to_le_bytes())?;

            // shares_criteria: foreign_key (Achievement) int32
            b.write_all(&(row.shares_criteria.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for Achievement {
    type Table = Self;

    fn get(&self, key: &AchievementKey) -> Option<&AchievementRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AchievementKey) -> Option<&mut AchievementRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AchievementRow {
    pub id: AchievementKey,
    pub faction: FactionKey,
    pub instance_id: MapKey,
    pub supercedes: AchievementKey,
    pub title_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub category: Achievement_CategoryKey,
    pub points: i32,
    pub ui_order: i32,
    pub flags: i32,
    pub icon_id: SpellIconKey,
    pub reward_lang: ExtendedLocalizedString,
    pub minimum_criteria: i32,
    pub shares_criteria: AchievementKey,
}

impl DbcRow for AchievementRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn achievement() {
        let mut file = File::open("../wrath-dbc/Achievement.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Achievement::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Achievement::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
