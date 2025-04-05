use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::chr_classes::{
    ChrClasses, ChrClassesKey,
};
use crate::vanilla_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use crate::vanilla_tables::skill_costs_data::{
    SkillCostsData, SkillCostsDataKey,
};
use crate::vanilla_tables::skill_line::{
    SkillLine, SkillLineKey,
};
use crate::vanilla_tables::skill_tiers::{
    SkillTiers, SkillTiersKey,
};
use std::io::Write;
use super::VanillaTable;

pub type SkillRaceClassInfoKey = crate::PrimaryKey<u32, SkillRaceClassInfo>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillRaceClassInfo {
    pub rows: Vec<SkillRaceClassInfoRow>,
}

impl SkillRaceClassInfo {
    pub const FILENAME: &'static str = "SkillRaceClassInfo.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

    pub fn verify(&self, chr_classes: &ChrClasses, chr_races: &ChrRaces, skill_costs_data: &SkillCostsData, skill_line: &SkillLine, skill_tiers: &SkillTiers) -> Result<(), crate::InvalidForeignKeyError<&SkillRaceClassInfoRow>> {
        for row in &self.rows {
            if row.skill_line.id != 0 && skill_line.get(&row.skill_line).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillRaceClassInfo>(),
                    row,
                    id,
                    row.skill_line.id.into()
                ));
            }

            if row.race_mask.id != 0 && chr_races.get(&row.race_mask).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillRaceClassInfo>(),
                    row,
                    id,
                    row.race_mask.id.into()
                ));
            }

            if row.class_mask.id != 0 && chr_classes.get(&row.class_mask).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillRaceClassInfo>(),
                    row,
                    id,
                    row.class_mask.id.into()
                ));
            }

            if row.skill_tier.id != 0 && skill_tiers.get(&row.skill_tier).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillRaceClassInfo>(),
                    row,
                    id,
                    row.skill_tier.id.into()
                ));
            }

            if row.skill_cost.id != 0 && skill_costs_data.get(&row.skill_cost).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<SkillRaceClassInfo>(),
                    row,
                    id,
                    row.skill_cost.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for SkillRaceClassInfo {
    fn into(self) -> VanillaTable {
        VanillaTable::SkillRaceClassInfo(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SkillRaceClassInfo {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SkillRaceClassInfoRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SkillRaceClassInfoRow] { &mut self.rows }

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

            // id: primary_key (SkillRaceClassInfo) uint32
            let id = SkillRaceClassInfoKey::new(crate::util::read_u32_le(chunk)?);

            // skill_line: foreign_key (SkillLine) uint32
            let skill_line = SkillLineKey::new(crate::util::read_u32_le(chunk)?.into());

            // race_mask: foreign_key (ChrRaces) uint32
            let race_mask = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // class_mask: foreign_key (ChrClasses) uint32
            let class_mask = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // skill_tier: foreign_key (SkillTiers) uint32
            let skill_tier = SkillTiersKey::new(crate::util::read_u32_le(chunk)?.into());

            // skill_cost: foreign_key (SkillCostsData) uint32
            let skill_cost = SkillCostsDataKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SkillRaceClassInfoRow {
                id,
                skill_line,
                race_mask,
                class_mask,
                flags,
                min_level,
                skill_tier,
                skill_cost,
            });
        }

        Ok(SkillRaceClassInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SkillRaceClassInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_line: foreign_key (SkillLine) uint32
            b.write_all(&(row.skill_line.id as u32).to_le_bytes())?;

            // race_mask: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race_mask.id as u32).to_le_bytes())?;

            // class_mask: foreign_key (ChrClasses) uint32
            b.write_all(&(row.class_mask.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // skill_tier: foreign_key (SkillTiers) uint32
            b.write_all(&(row.skill_tier.id as u32).to_le_bytes())?;

            // skill_cost: foreign_key (SkillCostsData) uint32
            b.write_all(&(row.skill_cost.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for SkillRaceClassInfo {
    type Table = Self;

    fn get(&self, key: &SkillRaceClassInfoKey) -> Option<&SkillRaceClassInfoRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SkillRaceClassInfoKey) -> Option<&mut SkillRaceClassInfoRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillRaceClassInfoRow {
    pub id: SkillRaceClassInfoKey,
    pub skill_line: SkillLineKey,
    pub race_mask: ChrRacesKey,
    pub class_mask: ChrClassesKey,
    pub flags: i32,
    pub min_level: i32,
    pub skill_tier: SkillTiersKey,
    pub skill_cost: SkillCostsDataKey,
}

impl DbcRow for SkillRaceClassInfoRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn skill_race_class_info() {
        let mut file = File::open("../vanilla-dbc/SkillRaceClassInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SkillRaceClassInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SkillRaceClassInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
