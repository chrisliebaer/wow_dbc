use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use std::io::Write;
use super::WrathTable;

pub type BarberShopStyleKey = crate::PrimaryKey<i32, BarberShopStyle>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarberShopStyle {
    pub rows: Vec<BarberShopStyleRow>,
}

impl BarberShopStyle {
    pub const FILENAME: &'static str = "BarberShopStyle.dbc";
    pub const FIELD_COUNT: usize = 40;
    pub const ROW_SIZE: usize = 160;

    pub fn verify(&self, chr_races: &ChrRaces) -> Result<(), crate::InvalidForeignKeyError<&BarberShopStyleRow>> {
        for row in &self.rows {
            if row.race.id != 0 && chr_races.get(&row.race).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<BarberShopStyle>(),
                    row,
                    id,
                    row.race.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for BarberShopStyle {
    fn into(self) -> WrathTable {
        WrathTable::BarberShopStyle(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for BarberShopStyle {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[BarberShopStyleRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [BarberShopStyleRow] { &mut self.rows }

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

            // id: primary_key (BarberShopStyle) int32
            let id = BarberShopStyleKey::new(crate::util::read_i32_le(chunk)?);

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // display_name_lang: string_ref_loc (Extended)
            let display_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // cost_modifier: float
            let cost_modifier = crate::util::read_f32_le(chunk)?;

            // race: foreign_key (ChrRaces) int32
            let race = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex: int32
            let sex = crate::util::read_i32_le(chunk)?;

            // data: int32
            let data = crate::util::read_i32_le(chunk)?;


            rows.push(BarberShopStyleRow {
                id,
                ty,
                display_name_lang,
                description_lang,
                cost_modifier,
                race,
                sex,
                data,
            });
        }

        Ok(BarberShopStyle { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (BarberShopStyle) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // display_name_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_lang.string_indices_as_array(&mut string_cache))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

            // cost_modifier: float
            b.write_all(&row.cost_modifier.to_le_bytes())?;

            // race: foreign_key (ChrRaces) int32
            b.write_all(&(row.race.id as i32).to_le_bytes())?;

            // sex: int32
            b.write_all(&row.sex.to_le_bytes())?;

            // data: int32
            b.write_all(&row.data.to_le_bytes())?;

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
impl Indexable<i32> for BarberShopStyle {
    type Table = Self;

    fn get(&self, key: &BarberShopStyleKey) -> Option<&BarberShopStyleRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &BarberShopStyleKey) -> Option<&mut BarberShopStyleRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarberShopStyleRow {
    pub id: BarberShopStyleKey,
    pub ty: i32,
    pub display_name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub cost_modifier: f32,
    pub race: ChrRacesKey,
    pub sex: i32,
    pub data: i32,
}

impl DbcRow for BarberShopStyleRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn barber_shop_style() {
        let mut file = File::open("../wrath-dbc/BarberShopStyle.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = BarberShopStyle::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = BarberShopStyle::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
