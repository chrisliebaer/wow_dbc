use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::chr_classes::{
    ChrClasses, ChrClassesKey,
};
use crate::vanilla_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use crate::vanilla_tables::spell_icon::{
    SpellIcon, SpellIconKey,
};
use std::io::Write;
use super::VanillaTable;

pub type TalentTabKey = crate::PrimaryKey<u32, TalentTab>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TalentTab {
    pub rows: Vec<TalentTabRow>,
}

impl TalentTab {
    pub const FILENAME: &'static str = "TalentTab.dbc";
    pub const FIELD_COUNT: usize = 15;
    pub const ROW_SIZE: usize = 60;

    pub fn verify(&self, chr_classes: &ChrClasses, chr_races: &ChrRaces, spell_icon: &SpellIcon) -> Result<(), crate::InvalidForeignKeyError<&TalentTabRow>> {
        for row in &self.rows {
            if row.spell_icon.id != 0 && spell_icon.get(&row.spell_icon).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TalentTab>(),
                    row,
                    id,
                    row.spell_icon.id.into()
                ));
            }

            if row.race_mask.id != 0 && chr_races.get(&row.race_mask).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TalentTab>(),
                    row,
                    id,
                    row.race_mask.id.into()
                ));
            }

            if row.class_mask.id != 0 && chr_classes.get(&row.class_mask).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TalentTab>(),
                    row,
                    id,
                    row.class_mask.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for TalentTab {
    fn into(self) -> VanillaTable {
        VanillaTable::TalentTab(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TalentTab {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TalentTabRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TalentTabRow] { &mut self.rows }

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

            // id: primary_key (TalentTab) uint32
            let id = TalentTabKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(crate::util::read_u32_le(chunk)?.into());

            // race_mask: foreign_key (ChrRaces) uint32
            let race_mask = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // class_mask: foreign_key (ChrClasses) uint32
            let class_mask = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // order_index: uint32
            let order_index = crate::util::read_u32_le(chunk)?;

            // background_file: string_ref
            let background_file = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(TalentTabRow {
                id,
                name,
                spell_icon,
                race_mask,
                class_mask,
                order_index,
                background_file,
            });
        }

        Ok(TalentTab { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TalentTab) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // spell_icon: foreign_key (SpellIcon) uint32
            b.write_all(&(row.spell_icon.id as u32).to_le_bytes())?;

            // race_mask: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race_mask.id as u32).to_le_bytes())?;

            // class_mask: foreign_key (ChrClasses) uint32
            b.write_all(&(row.class_mask.id as u32).to_le_bytes())?;

            // order_index: uint32
            b.write_all(&row.order_index.to_le_bytes())?;

            // background_file: string_ref
            b.write_all(&string_cache.add_string(&row.background_file).to_le_bytes())?;

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
impl Indexable<u32> for TalentTab {
    type Table = Self;

    fn get(&self, key: &TalentTabKey) -> Option<&TalentTabRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TalentTabKey) -> Option<&mut TalentTabRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TalentTabRow {
    pub id: TalentTabKey,
    pub name: LocalizedString,
    pub spell_icon: SpellIconKey,
    pub race_mask: ChrRacesKey,
    pub class_mask: ChrClassesKey,
    pub order_index: u32,
    pub background_file: String,
}

impl DbcRow for TalentTabRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn talent_tab() {
        let mut file = File::open("../vanilla-dbc/TalentTab.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TalentTab::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TalentTab::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
