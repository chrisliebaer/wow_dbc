use crate::{
    DbcRow, DbcTable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttackAnimTypes {
    pub rows: Vec<AttackAnimTypesRow>,
}

impl AttackAnimTypes {
    pub const FILENAME: &'static str = "AttackAnimTypes.dbc";
    pub const FIELD_COUNT: usize = 2;
    pub const ROW_SIZE: usize = 8;

}

impl Into<WrathTable> for AttackAnimTypes {
    fn into(self) -> WrathTable {
        WrathTable::AttackAnimTypes(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AttackAnimTypes {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AttackAnimTypesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AttackAnimTypesRow] { &mut self.rows }

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

            // anim_id: int32
            let anim_id = crate::util::read_i32_le(chunk)?;

            // anim_name: string_ref
            let anim_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(AttackAnimTypesRow {
                anim_id,
                anim_name,
            });
        }

        Ok(AttackAnimTypes { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // anim_id: int32
            b.write_all(&row.anim_id.to_le_bytes())?;

            // anim_name: string_ref
            b.write_all(&string_cache.add_string(&row.anim_name).to_le_bytes())?;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttackAnimTypesRow {
    pub anim_id: i32,
    pub anim_name: String,
}

impl DbcRow for AttackAnimTypesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn attack_anim_types() {
        let mut file = File::open("../wrath-dbc/AttackAnimTypes.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AttackAnimTypes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AttackAnimTypes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
