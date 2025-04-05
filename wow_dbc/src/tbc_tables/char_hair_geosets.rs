use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type CharHairGeosetsKey = crate::PrimaryKey<i32, CharHairGeosets>;

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
            if row.race_id.id != 0 && chr_races.get(&row.race_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CharHairGeosets>(),
                    row,
                    id,
                    row.race_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for CharHairGeosets {
    fn into(self) -> TbcTable {
        TbcTable::CharHairGeosets(self)
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

            // id: primary_key (CharHairGeosets) int32
            let id = CharHairGeosetsKey::new(crate::util::read_i32_le(chunk)?);

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex_id: int32
            let sex_id = crate::util::read_i32_le(chunk)?;

            // variation_id: int32
            let variation_id = crate::util::read_i32_le(chunk)?;

            // geoset_id: int32
            let geoset_id = crate::util::read_i32_le(chunk)?;

            // showscalp: int32
            let showscalp = crate::util::read_i32_le(chunk)?;


            rows.push(CharHairGeosetsRow {
                id,
                race_id,
                sex_id,
                variation_id,
                geoset_id,
                showscalp,
            });
        }

        Ok(CharHairGeosets { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CharHairGeosets) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // variation_id: int32
            b.write_all(&row.variation_id.to_le_bytes())?;

            // geoset_id: int32
            b.write_all(&row.geoset_id.to_le_bytes())?;

            // showscalp: int32
            b.write_all(&row.showscalp.to_le_bytes())?;

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
impl Indexable<i32> for CharHairGeosets {
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
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub variation_id: i32,
    pub geoset_id: i32,
    pub showscalp: i32,
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
        let mut file = File::open("../tbc-dbc/CharHairGeosets.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharHairGeosets::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharHairGeosets::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
