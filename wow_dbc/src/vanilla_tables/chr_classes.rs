use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::Power;

pub type ChrClassesKey = crate::PrimaryKey<u32, ChrClasses>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrClasses {
    pub rows: Vec<ChrClassesRow>,
}

impl ChrClasses {
    pub const FILENAME: &'static str = "ChrClasses.dbc";
    pub const FIELD_COUNT: usize = 17;
    pub const ROW_SIZE: usize = 68;

}

impl Into<VanillaTable> for ChrClasses {
    fn into(self) -> VanillaTable {
        VanillaTable::ChrClasses(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ChrClasses {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ChrClassesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ChrClassesRow] { &mut self.rows }

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

            // id: primary_key (ChrClasses) uint32
            let id = ChrClassesKey::new(crate::util::read_u32_le(chunk)?);

            // player_class: uint32
            let player_class = crate::util::read_u32_le(chunk)?;

            // damage_bonus_stat: int32
            let damage_bonus_stat = crate::util::read_i32_le(chunk)?;

            // power_type: Power
            let power_type = crate::util::read_i32_le(chunk)?.try_into()?;

            // pet_name_token: string_ref
            let pet_name_token = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // filename: string_ref
            let filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // class_mask: int32
            let class_mask = crate::util::read_i32_le(chunk)?;

            // hybrid_class: bool32
            let hybrid_class = crate::util::read_u32_le(chunk)? != 0;


            rows.push(ChrClassesRow {
                id,
                player_class,
                damage_bonus_stat,
                power_type,
                pet_name_token,
                name,
                filename,
                class_mask,
                hybrid_class,
            });
        }

        Ok(ChrClasses { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ChrClasses) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // player_class: uint32
            b.write_all(&row.player_class.to_le_bytes())?;

            // damage_bonus_stat: int32
            b.write_all(&row.damage_bonus_stat.to_le_bytes())?;

            // power_type: Power
            b.write_all(&(row.power_type.as_int() as i32).to_le_bytes())?;

            // pet_name_token: string_ref
            b.write_all(&string_cache.add_string(&row.pet_name_token).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // filename: string_ref
            b.write_all(&string_cache.add_string(&row.filename).to_le_bytes())?;

            // class_mask: int32
            b.write_all(&row.class_mask.to_le_bytes())?;

            // hybrid_class: bool32
            b.write_all(&u32::from(row.hybrid_class).to_le_bytes())?;

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
impl Indexable<u32> for ChrClasses {
    type Table = Self;

    fn get(&self, key: &ChrClassesKey) -> Option<&ChrClassesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ChrClassesKey) -> Option<&mut ChrClassesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrClassesRow {
    pub id: ChrClassesKey,
    pub player_class: u32,
    pub damage_bonus_stat: i32,
    pub power_type: Power,
    pub pet_name_token: String,
    pub name: LocalizedString,
    pub filename: String,
    pub class_mask: i32,
    pub hybrid_class: bool,
}

impl DbcRow for ChrClassesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn chr_classes() {
        let mut file = File::open("../vanilla-dbc/ChrClasses.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ChrClasses::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ChrClasses::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
