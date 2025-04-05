use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::{
    Gender, SelectionType,
};

pub type CharSectionsKey = crate::PrimaryKey<u32, CharSections>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharSections {
    pub rows: Vec<CharSectionsRow>,
}

impl CharSections {
    pub const FILENAME: &'static str = "CharSections.dbc";
    pub const FIELD_COUNT: usize = 10;
    pub const ROW_SIZE: usize = 40;

    pub fn verify(&self, chr_races: &ChrRaces) -> Result<(), crate::InvalidForeignKeyError<&CharSectionsRow>> {
        for row in &self.rows {
            if row.race.id != 0 && chr_races.get(&row.race).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CharSections>(),
                    row,
                    id,
                    row.race.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CharSections {
    fn into(self) -> VanillaTable {
        VanillaTable::CharSections(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CharSections {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CharSectionsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CharSectionsRow] { &mut self.rows }

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

            // id: primary_key (CharSections) uint32
            let id = CharSectionsKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i32_le(chunk)?.try_into()?;

            // ty: SelectionType
            let ty = crate::util::read_i32_le(chunk)?.try_into()?;

            // variation_index: int32
            let variation_index = crate::util::read_i32_le(chunk)?;

            // colour_index: int32
            let colour_index = crate::util::read_i32_le(chunk)?;

            // texture_name: string_ref[3]
            let texture_name = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // npc_only: bool32
            let npc_only = crate::util::read_u32_le(chunk)? != 0;


            rows.push(CharSectionsRow {
                id,
                race,
                gender,
                ty,
                variation_index,
                colour_index,
                texture_name,
                npc_only,
            });
        }

        Ok(CharSections { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CharSections) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // ty: SelectionType
            b.write_all(&(row.ty.as_int() as i32).to_le_bytes())?;

            // variation_index: int32
            b.write_all(&row.variation_index.to_le_bytes())?;

            // colour_index: int32
            b.write_all(&row.colour_index.to_le_bytes())?;

            // texture_name: string_ref[3]
            for i in &row.texture_name {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // npc_only: bool32
            b.write_all(&u32::from(row.npc_only).to_le_bytes())?;

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
impl Indexable<u32> for CharSections {
    type Table = Self;

    fn get(&self, key: &CharSectionsKey) -> Option<&CharSectionsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CharSectionsKey) -> Option<&mut CharSectionsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharSectionsRow {
    pub id: CharSectionsKey,
    pub race: ChrRacesKey,
    pub gender: Gender,
    pub ty: SelectionType,
    pub variation_index: i32,
    pub colour_index: i32,
    pub texture_name: [String; 3],
    pub npc_only: bool,
}

impl DbcRow for CharSectionsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_sections() {
        let mut file = File::open("../vanilla-dbc/CharSections.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharSections::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharSections::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
