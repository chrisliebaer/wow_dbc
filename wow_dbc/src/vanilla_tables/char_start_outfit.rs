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
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::Gender;

pub type CharStartOutfitKey = crate::PrimaryKey<u32, CharStartOutfit>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharStartOutfit {
    pub rows: Vec<CharStartOutfitRow>,
}

impl CharStartOutfit {
    pub const FILENAME: &'static str = "CharStartOutfit.dbc";
    pub const FIELD_COUNT: usize = 41;
    pub const ROW_SIZE: usize = 152;

    pub fn verify(&self, chr_classes: &ChrClasses, chr_races: &ChrRaces) -> Result<(), crate::InvalidForeignKeyError<&CharStartOutfitRow>> {
        for row in &self.rows {
            if row.race.id != 0 && chr_races.get(&row.race).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CharStartOutfit>(),
                    row,
                    id,
                    row.race.id.into()
                ));
            }

            if row.class.id != 0 && chr_classes.get(&row.class).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CharStartOutfit>(),
                    row,
                    id,
                    row.class.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CharStartOutfit {
    fn into(self) -> VanillaTable {
        VanillaTable::CharStartOutfit(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CharStartOutfit {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CharStartOutfitRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CharStartOutfitRow] { &mut self.rows }

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

            // id: primary_key (CharStartOutfit) uint32
            let id = CharStartOutfitKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint8
            let race = ChrRacesKey::new(crate::util::read_u8_le(chunk)?.into());

            // class: foreign_key (ChrClasses) uint8
            let class = ChrClassesKey::new(crate::util::read_u8_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i8_le(chunk)?.try_into()?;

            // outfit_id: int8
            let outfit_id = crate::util::read_i8_le(chunk)?;

            // item_id: int32[12]
            let item_id = crate::util::read_array_i32::<12>(chunk)?;

            // display_id: int32[12]
            let display_id = crate::util::read_array_i32::<12>(chunk)?;

            // inv_slot_id: int32[12]
            let inv_slot_id = crate::util::read_array_i32::<12>(chunk)?;


            rows.push(CharStartOutfitRow {
                id,
                race,
                class,
                gender,
                outfit_id,
                item_id,
                display_id,
                inv_slot_id,
            });
        }

        Ok(CharStartOutfit { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CharStartOutfit) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint8
            b.write_all(&(row.race.id as u8).to_le_bytes())?;

            // class: foreign_key (ChrClasses) uint8
            b.write_all(&(row.class.id as u8).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i8).to_le_bytes())?;

            // outfit_id: int8
            b.write_all(&row.outfit_id.to_le_bytes())?;

            // item_id: int32[12]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // display_id: int32[12]
            for i in row.display_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // inv_slot_id: int32[12]
            for i in row.inv_slot_id {
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
impl Indexable<u32> for CharStartOutfit {
    type Table = Self;

    fn get(&self, key: &CharStartOutfitKey) -> Option<&CharStartOutfitRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CharStartOutfitKey) -> Option<&mut CharStartOutfitRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharStartOutfitRow {
    pub id: CharStartOutfitKey,
    pub race: ChrRacesKey,
    pub class: ChrClassesKey,
    pub gender: Gender,
    pub outfit_id: i8,
    pub item_id: [i32; 12],
    pub display_id: [i32; 12],
    pub inv_slot_id: [i32; 12],
}

impl DbcRow for CharStartOutfitRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_start_outfit() {
        let mut file = File::open("../vanilla-dbc/CharStartOutfit.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharStartOutfit::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharStartOutfit::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
