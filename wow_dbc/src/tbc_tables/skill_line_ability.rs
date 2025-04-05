use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::skill_line::{
    SkillLine, SkillLineKey,
};
use crate::tbc_tables::spell::{
    Spell, SpellKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type SkillLineAbilityKey = crate::PrimaryKey<i32, SkillLineAbility>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLineAbility {
    pub rows: Vec<SkillLineAbilityRow>,
}

impl SkillLineAbility {
    pub const FILENAME: &'static str = "SkillLineAbility.dbc";
    pub const FIELD_COUNT: usize = 15;
    pub const ROW_SIZE: usize = 60;

    pub fn verify(&self, skill_line: &SkillLine, spell: &Spell) -> Result<(), crate::InvalidForeignKeyError<&SkillLineAbilityRow>> {
        for row in &self.rows {
            if row.skill_line.id != 0 && skill_line.get(&row.skill_line).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLineAbility>(),
                    row,
                    id,
                    row.skill_line.id.into()
                ));
            }

            if row.spell.id != 0 && spell.get(&row.spell).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLineAbility>(),
                    row,
                    id,
                    row.spell.id.into()
                ));
            }

            if row.superceded_by_spell.id != 0 && spell.get(&row.superceded_by_spell).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillLineAbility>(),
                    row,
                    id,
                    row.superceded_by_spell.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for SkillLineAbility {
    fn into(self) -> TbcTable {
        TbcTable::SkillLineAbility(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SkillLineAbility {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SkillLineAbilityRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SkillLineAbilityRow] { &mut self.rows }

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

            // id: primary_key (SkillLineAbility) int32
            let id = SkillLineAbilityKey::new(crate::util::read_i32_le(chunk)?);

            // skill_line: foreign_key (SkillLine) int32
            let skill_line = SkillLineKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell: foreign_key (Spell) int32
            let spell = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // race_mask: int32
            let race_mask = crate::util::read_i32_le(chunk)?;

            // class_mask: int32
            let class_mask = crate::util::read_i32_le(chunk)?;

            // exclude_race: int32
            let exclude_race = crate::util::read_i32_le(chunk)?;

            // exclude_class: int32
            let exclude_class = crate::util::read_i32_le(chunk)?;

            // min_skill_line_rank: int32
            let min_skill_line_rank = crate::util::read_i32_le(chunk)?;

            // superceded_by_spell: foreign_key (Spell) int32
            let superceded_by_spell = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // acquire_method: int32
            let acquire_method = crate::util::read_i32_le(chunk)?;

            // trivial_skill_line_rank_high: int32
            let trivial_skill_line_rank_high = crate::util::read_i32_le(chunk)?;

            // trivial_skill_line_rank_low: int32
            let trivial_skill_line_rank_low = crate::util::read_i32_le(chunk)?;

            // abandonable: int32
            let abandonable = crate::util::read_i32_le(chunk)?;

            // character_points: int32[2]
            let character_points = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(SkillLineAbilityRow {
                id,
                skill_line,
                spell,
                race_mask,
                class_mask,
                exclude_race,
                exclude_class,
                min_skill_line_rank,
                superceded_by_spell,
                acquire_method,
                trivial_skill_line_rank_high,
                trivial_skill_line_rank_low,
                abandonable,
                character_points,
            });
        }

        Ok(SkillLineAbility { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SkillLineAbility) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_line: foreign_key (SkillLine) int32
            b.write_all(&(row.skill_line.id as i32).to_le_bytes())?;

            // spell: foreign_key (Spell) int32
            b.write_all(&(row.spell.id as i32).to_le_bytes())?;

            // race_mask: int32
            b.write_all(&row.race_mask.to_le_bytes())?;

            // class_mask: int32
            b.write_all(&row.class_mask.to_le_bytes())?;

            // exclude_race: int32
            b.write_all(&row.exclude_race.to_le_bytes())?;

            // exclude_class: int32
            b.write_all(&row.exclude_class.to_le_bytes())?;

            // min_skill_line_rank: int32
            b.write_all(&row.min_skill_line_rank.to_le_bytes())?;

            // superceded_by_spell: foreign_key (Spell) int32
            b.write_all(&(row.superceded_by_spell.id as i32).to_le_bytes())?;

            // acquire_method: int32
            b.write_all(&row.acquire_method.to_le_bytes())?;

            // trivial_skill_line_rank_high: int32
            b.write_all(&row.trivial_skill_line_rank_high.to_le_bytes())?;

            // trivial_skill_line_rank_low: int32
            b.write_all(&row.trivial_skill_line_rank_low.to_le_bytes())?;

            // abandonable: int32
            b.write_all(&row.abandonable.to_le_bytes())?;

            // character_points: int32[2]
            for i in row.character_points {
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
impl Indexable<i32> for SkillLineAbility {
    type Table = Self;

    fn get(&self, key: &SkillLineAbilityKey) -> Option<&SkillLineAbilityRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SkillLineAbilityKey) -> Option<&mut SkillLineAbilityRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLineAbilityRow {
    pub id: SkillLineAbilityKey,
    pub skill_line: SkillLineKey,
    pub spell: SpellKey,
    pub race_mask: i32,
    pub class_mask: i32,
    pub exclude_race: i32,
    pub exclude_class: i32,
    pub min_skill_line_rank: i32,
    pub superceded_by_spell: SpellKey,
    pub acquire_method: i32,
    pub trivial_skill_line_rank_high: i32,
    pub trivial_skill_line_rank_low: i32,
    pub abandonable: i32,
    pub character_points: [i32; 2],
}

impl DbcRow for SkillLineAbilityRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn skill_line_ability() {
        let mut file = File::open("../tbc-dbc/SkillLineAbility.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SkillLineAbility::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SkillLineAbility::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
