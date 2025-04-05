use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::spell::{
    Spell, SpellKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::OceanType;

pub type LiquidTypeKey = crate::PrimaryKey<u32, LiquidType>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidType {
    pub rows: Vec<LiquidTypeRow>,
}

impl LiquidType {
    pub const FILENAME: &'static str = "LiquidType.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, spell: &Spell) -> Result<(), crate::InvalidForeignKeyError<&LiquidTypeRow>> {
        for row in &self.rows {
            if row.spell.id != 0 && spell.get(&row.spell).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<LiquidType>(),
                    row,
                    id,
                    row.spell.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for LiquidType {
    fn into(self) -> VanillaTable {
        VanillaTable::LiquidType(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for LiquidType {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[LiquidTypeRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [LiquidTypeRow] { &mut self.rows }

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

            // id: primary_key (LiquidType) uint32
            let id = LiquidTypeKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // ty: OceanType
            let ty = crate::util::read_i32_le(chunk)?.try_into()?;

            // spell: foreign_key (Spell) uint32
            let spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(LiquidTypeRow {
                id,
                name,
                ty,
                spell,
            });
        }

        Ok(LiquidType { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LiquidType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // ty: OceanType
            b.write_all(&(row.ty.as_int() as i32).to_le_bytes())?;

            // spell: foreign_key (Spell) uint32
            b.write_all(&(row.spell.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for LiquidType {
    type Table = Self;

    fn get(&self, key: &LiquidTypeKey) -> Option<&LiquidTypeRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &LiquidTypeKey) -> Option<&mut LiquidTypeRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidTypeRow {
    pub id: LiquidTypeKey,
    pub name: String,
    pub ty: OceanType,
    pub spell: SpellKey,
}

impl DbcRow for LiquidTypeRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn liquid_type() {
        let mut file = File::open("../vanilla-dbc/LiquidType.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LiquidType::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LiquidType::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
