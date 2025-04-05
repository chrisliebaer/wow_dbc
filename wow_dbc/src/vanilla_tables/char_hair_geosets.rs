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
    Gender, Scalp,
};

pub type CharHairGeosetsKey = crate::PrimaryKey<u32, CharHairGeosets>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharHairGeosets {
    pub rows: Vec<CharHairGeosetsRow>,
}

impl CharHairGeosets {
    pub const FILENAME: &'static str = "CharHairGeosets.dbc";
    pub const FIELD_COUNT: usize = 6;
    pub const ROW_SIZE: usize = 24;

    pub fn verify(&self, chr_races: &ChrRaces) -> Result<(), crate::InvalidForeignKeyError<&CharHairGeosetsRow>> {
        for row in &self.rows {
            if row.race.id != 0 && chr_races.get(&row.race).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CharHairGeosets>(),
                    row,
                    id,
                    row.race.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CharHairGeosets {
    fn into(self) -> VanillaTable {
        VanillaTable::CharHairGeosets(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CharHairGeosets {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CharHairGeosetsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CharHairGeosetsRow] { &mut self.rows }

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

            // id: primary_key (CharHairGeosets) uint32
            let id = CharHairGeosetsKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i32_le(chunk)?.try_into()?;

            // variation: uint32
            let variation = crate::util::read_u32_le(chunk)?;

            // geoset: int32
            let geoset = crate::util::read_i32_le(chunk)?;

            // show_scalp: Scalp
            let show_scalp = crate::util::read_i32_le(chunk)?.try_into()?;


            rows.push(CharHairGeosetsRow {
                id,
                race,
                gender,
                variation,
                geoset,
                show_scalp,
            });
        }

        Ok(CharHairGeosets { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CharHairGeosets) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // variation: uint32
            b.write_all(&row.variation.to_le_bytes())?;

            // geoset: int32
            b.write_all(&row.geoset.to_le_bytes())?;

            // show_scalp: Scalp
            b.write_all(&(row.show_scalp.as_int() as i32).to_le_bytes())?;

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
impl Indexable<u32> for CharHairGeosets {
    type Table = Self;

    fn get(&self, key: &CharHairGeosetsKey) -> Option<&CharHairGeosetsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CharHairGeosetsKey) -> Option<&mut CharHairGeosetsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharHairGeosetsRow {
    pub id: CharHairGeosetsKey,
    pub race: ChrRacesKey,
    pub gender: Gender,
    pub variation: u32,
    pub geoset: i32,
    pub show_scalp: Scalp,
}

impl DbcRow for CharHairGeosetsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_hair_geosets() {
        let mut file = File::open("../vanilla-dbc/CharHairGeosets.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharHairGeosets::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharHairGeosets::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
