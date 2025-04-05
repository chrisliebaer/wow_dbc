use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::skill_line::{
    SkillLine, SkillLineKey,
};
use std::io::Write;
use super::WrathTable;

pub type ItemSetKey = crate::PrimaryKey<i32, ItemSet>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemSet {
    pub rows: Vec<ItemSetRow>,
}

impl ItemSet {
    pub const FILENAME: &'static str = "ItemSet.dbc";
    pub const FIELD_COUNT: usize = 53;
    pub const ROW_SIZE: usize = 212;

    pub fn verify(&self, skill_line: &SkillLine) -> Result<(), crate::InvalidForeignKeyError<&ItemSetRow>> {
        for row in &self.rows {
            if row.required_skill.id != 0 && skill_line.get(&row.required_skill).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ItemSet>(),
                    row,
                    id,
                    row.required_skill.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for ItemSet {
    fn into(self) -> WrathTable {
        WrathTable::ItemSet(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ItemSet {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ItemSetRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ItemSetRow] { &mut self.rows }

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

            // id: primary_key (ItemSet) int32
            let id = ItemSetKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // item_id: int32[17]
            let item_id = crate::util::read_array_i32::<17>(chunk)?;

            // set_spell_id: int32[8]
            let set_spell_id = crate::util::read_array_i32::<8>(chunk)?;

            // set_threshold: int32[8]
            let set_threshold = crate::util::read_array_i32::<8>(chunk)?;

            // required_skill: foreign_key (SkillLine) int32
            let required_skill = SkillLineKey::new(crate::util::read_i32_le(chunk)?.into());

            // required_skill_rank: int32
            let required_skill_rank = crate::util::read_i32_le(chunk)?;


            rows.push(ItemSetRow {
                id,
                name_lang,
                item_id,
                set_spell_id,
                set_threshold,
                required_skill,
                required_skill_rank,
            });
        }

        Ok(ItemSet { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ItemSet) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // item_id: int32[17]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // set_spell_id: int32[8]
            for i in row.set_spell_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // set_threshold: int32[8]
            for i in row.set_threshold {
                b.write_all(&i.to_le_bytes())?;
            }


            // required_skill: foreign_key (SkillLine) int32
            b.write_all(&(row.required_skill.id as i32).to_le_bytes())?;

            // required_skill_rank: int32
            b.write_all(&row.required_skill_rank.to_le_bytes())?;

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
impl Indexable<i32> for ItemSet {
    type Table = Self;

    fn get(&self, key: &ItemSetKey) -> Option<&ItemSetRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ItemSetKey) -> Option<&mut ItemSetRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemSetRow {
    pub id: ItemSetKey,
    pub name_lang: ExtendedLocalizedString,
    pub item_id: [i32; 17],
    pub set_spell_id: [i32; 8],
    pub set_threshold: [i32; 8],
    pub required_skill: SkillLineKey,
    pub required_skill_rank: i32,
}

impl DbcRow for ItemSetRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_set() {
        let mut file = File::open("../wrath-dbc/ItemSet.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemSet::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemSet::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
