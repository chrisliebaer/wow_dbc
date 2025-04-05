use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type FactionKey = crate::PrimaryKey<i32, Faction>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Faction {
    pub rows: Vec<FactionRow>,
}

impl Faction {
    pub const FILENAME: &'static str = "Faction.dbc";
    pub const FIELD_COUNT: usize = 53;
    pub const ROW_SIZE: usize = 212;

    pub fn verify(&self, ) -> Result<(), crate::InvalidForeignKeyError<&FactionRow>> {
        for row in &self.rows {
            if row.parent_faction_id.id != 0 && self.get(&row.parent_faction_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Faction>(),
                    row,
                    id,
                    row.parent_faction_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for Faction {
    fn into(self) -> TbcTable {
        TbcTable::Faction(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Faction {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[FactionRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [FactionRow] { &mut self.rows }

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

            // id: primary_key (Faction) int32
            let id = FactionKey::new(crate::util::read_i32_le(chunk)?);

            // reputation_index: int32
            let reputation_index = crate::util::read_i32_le(chunk)?;

            // reputation_race_mask: int32[4]
            let reputation_race_mask = crate::util::read_array_i32::<4>(chunk)?;

            // reputation_class_mask: int32[4]
            let reputation_class_mask = crate::util::read_array_i32::<4>(chunk)?;

            // reputation_base: int32[4]
            let reputation_base = crate::util::read_array_i32::<4>(chunk)?;

            // reputation_flags: int32[4]
            let reputation_flags = crate::util::read_array_i32::<4>(chunk)?;

            // parent_faction_id: foreign_key (Faction) int32
            let parent_faction_id = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(FactionRow {
                id,
                reputation_index,
                reputation_race_mask,
                reputation_class_mask,
                reputation_base,
                reputation_flags,
                parent_faction_id,
                name_lang,
                description_lang,
            });
        }

        Ok(Faction { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Faction) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // reputation_index: int32
            b.write_all(&row.reputation_index.to_le_bytes())?;

            // reputation_race_mask: int32[4]
            for i in row.reputation_race_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_class_mask: int32[4]
            for i in row.reputation_class_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_base: int32[4]
            for i in row.reputation_base {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_flags: int32[4]
            for i in row.reputation_flags {
                b.write_all(&i.to_le_bytes())?;
            }


            // parent_faction_id: foreign_key (Faction) int32
            b.write_all(&(row.parent_faction_id.id as i32).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<i32> for Faction {
    type Table = Self;

    fn get(&self, key: &FactionKey) -> Option<&FactionRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &FactionKey) -> Option<&mut FactionRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactionRow {
    pub id: FactionKey,
    pub reputation_index: i32,
    pub reputation_race_mask: [i32; 4],
    pub reputation_class_mask: [i32; 4],
    pub reputation_base: [i32; 4],
    pub reputation_flags: [i32; 4],
    pub parent_faction_id: FactionKey,
    pub name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
}

impl DbcRow for FactionRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn faction() {
        let mut file = File::open("../tbc-dbc/Faction.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Faction::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Faction::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
