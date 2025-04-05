use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::faction::{
    Faction, FactionKey,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type LFGDungeonsKey = crate::PrimaryKey<i32, LFGDungeons>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeons {
    pub rows: Vec<LFGDungeonsRow>,
}

impl LFGDungeons {
    pub const FILENAME: &'static str = "LFGDungeons.dbc";
    pub const FIELD_COUNT: usize = 24;
    pub const ROW_SIZE: usize = 96;

    pub fn verify(&self, faction: &Faction) -> Result<(), crate::InvalidForeignKeyError<&LFGDungeonsRow>> {
        for row in &self.rows {
            if row.faction.id != 0 && faction.get(&row.faction).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<LFGDungeons>(),
                    row,
                    id,
                    row.faction.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for LFGDungeons {
    fn into(self) -> TbcTable {
        TbcTable::LFGDungeons(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for LFGDungeons {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[LFGDungeonsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [LFGDungeonsRow] { &mut self.rows }

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

            // id: primary_key (LFGDungeons) int32
            let id = LFGDungeonsKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // type_id: int32
            let type_id = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture_filename: string_ref
            let texture_filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // expansion_level: int32
            let expansion_level = crate::util::read_i32_le(chunk)?;


            rows.push(LFGDungeonsRow {
                id,
                name_lang,
                min_level,
                max_level,
                type_id,
                faction,
                texture_filename,
                expansion_level,
            });
        }

        Ok(LFGDungeons { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LFGDungeons) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // type_id: int32
            b.write_all(&row.type_id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // texture_filename: string_ref
            b.write_all(&string_cache.add_string(&row.texture_filename).to_le_bytes())?;

            // expansion_level: int32
            b.write_all(&row.expansion_level.to_le_bytes())?;

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
impl Indexable<i32> for LFGDungeons {
    type Table = Self;

    fn get(&self, key: &LFGDungeonsKey) -> Option<&LFGDungeonsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &LFGDungeonsKey) -> Option<&mut LFGDungeonsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonsRow {
    pub id: LFGDungeonsKey,
    pub name_lang: ExtendedLocalizedString,
    pub min_level: i32,
    pub max_level: i32,
    pub type_id: i32,
    pub faction: FactionKey,
    pub texture_filename: String,
    pub expansion_level: i32,
}

impl DbcRow for LFGDungeonsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn lfg_dungeons() {
        let mut file = File::open("../tbc-dbc/LFGDungeons.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LFGDungeons::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LFGDungeons::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
