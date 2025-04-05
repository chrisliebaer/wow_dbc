use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::spell::{
    Spell, SpellKey,
};
use std::io::Write;
use super::WrathTable;

pub type TalentKey = crate::PrimaryKey<i32, Talent>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Talent {
    pub rows: Vec<TalentRow>,
}

impl Talent {
    pub const FILENAME: &'static str = "Talent.dbc";
    pub const FIELD_COUNT: usize = 23;
    pub const ROW_SIZE: usize = 92;

    pub fn verify(&self, spell: &Spell) -> Result<(), crate::InvalidForeignKeyError<&TalentRow>> {
        for row in &self.rows {
            if row.required_spell_id.id != 0 && spell.get(&row.required_spell_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Talent>(),
                    row,
                    id,
                    row.required_spell_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for Talent {
    fn into(self) -> WrathTable {
        WrathTable::Talent(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Talent {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TalentRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TalentRow] { &mut self.rows }

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

            // id: primary_key (Talent) int32
            let id = TalentKey::new(crate::util::read_i32_le(chunk)?);

            // tab_id: int32
            let tab_id = crate::util::read_i32_le(chunk)?;

            // tier_id: int32
            let tier_id = crate::util::read_i32_le(chunk)?;

            // column_index: int32
            let column_index = crate::util::read_i32_le(chunk)?;

            // spell_rank: int32[9]
            let spell_rank = crate::util::read_array_i32::<9>(chunk)?;

            // prereq_talent: int32[3]
            let prereq_talent = crate::util::read_array_i32::<3>(chunk)?;

            // prereq_rank: int32[3]
            let prereq_rank = crate::util::read_array_i32::<3>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // required_spell_id: foreign_key (Spell) int32
            let required_spell_id = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // category_mask: int32[2]
            let category_mask = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(TalentRow {
                id,
                tab_id,
                tier_id,
                column_index,
                spell_rank,
                prereq_talent,
                prereq_rank,
                flags,
                required_spell_id,
                category_mask,
            });
        }

        Ok(Talent { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Talent) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // tab_id: int32
            b.write_all(&row.tab_id.to_le_bytes())?;

            // tier_id: int32
            b.write_all(&row.tier_id.to_le_bytes())?;

            // column_index: int32
            b.write_all(&row.column_index.to_le_bytes())?;

            // spell_rank: int32[9]
            for i in row.spell_rank {
                b.write_all(&i.to_le_bytes())?;
            }


            // prereq_talent: int32[3]
            for i in row.prereq_talent {
                b.write_all(&i.to_le_bytes())?;
            }


            // prereq_rank: int32[3]
            for i in row.prereq_rank {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // required_spell_id: foreign_key (Spell) int32
            b.write_all(&(row.required_spell_id.id as i32).to_le_bytes())?;

            // category_mask: int32[2]
            for i in row.category_mask {
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
impl Indexable<i32> for Talent {
    type Table = Self;

    fn get(&self, key: &TalentKey) -> Option<&TalentRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TalentKey) -> Option<&mut TalentRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TalentRow {
    pub id: TalentKey,
    pub tab_id: i32,
    pub tier_id: i32,
    pub column_index: i32,
    pub spell_rank: [i32; 9],
    pub prereq_talent: [i32; 3],
    pub prereq_rank: [i32; 3],
    pub flags: i32,
    pub required_spell_id: SpellKey,
    pub category_mask: [i32; 2],
}

impl DbcRow for TalentRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn talent() {
        let mut file = File::open("../wrath-dbc/Talent.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Talent::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Talent::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
