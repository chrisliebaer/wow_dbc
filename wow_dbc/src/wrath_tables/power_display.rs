use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type PowerDisplayKey = crate::PrimaryKey<i32, PowerDisplay>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerDisplay {
    pub rows: Vec<PowerDisplayRow>,
}

impl PowerDisplay {
    pub const FILENAME: &'static str = "PowerDisplay.dbc";
    pub const FIELD_COUNT: usize = 6;
    pub const ROW_SIZE: usize = 15;

}

impl Into<WrathTable> for PowerDisplay {
    fn into(self) -> WrathTable {
        WrathTable::PowerDisplay(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for PowerDisplay {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[PowerDisplayRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [PowerDisplayRow] { &mut self.rows }

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

            // id: primary_key (PowerDisplay) int32
            let id = PowerDisplayKey::new(crate::util::read_i32_le(chunk)?);

            // actual_type: int32
            let actual_type = crate::util::read_i32_le(chunk)?;

            // global_string_base_tag: string_ref
            let global_string_base_tag = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // red: int8
            let red = crate::util::read_i8_le(chunk)?;

            // green: int8
            let green = crate::util::read_i8_le(chunk)?;

            // blue: int8
            let blue = crate::util::read_i8_le(chunk)?;


            rows.push(PowerDisplayRow {
                id,
                actual_type,
                global_string_base_tag,
                red,
                green,
                blue,
            });
        }

        Ok(PowerDisplay { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (PowerDisplay) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // actual_type: int32
            b.write_all(&row.actual_type.to_le_bytes())?;

            // global_string_base_tag: string_ref
            b.write_all(&string_cache.add_string(&row.global_string_base_tag).to_le_bytes())?;

            // red: int8
            b.write_all(&row.red.to_le_bytes())?;

            // green: int8
            b.write_all(&row.green.to_le_bytes())?;

            // blue: int8
            b.write_all(&row.blue.to_le_bytes())?;

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
impl Indexable<i32> for PowerDisplay {
    type Table = Self;

    fn get(&self, key: &PowerDisplayKey) -> Option<&PowerDisplayRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &PowerDisplayKey) -> Option<&mut PowerDisplayRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerDisplayRow {
    pub id: PowerDisplayKey,
    pub actual_type: i32,
    pub global_string_base_tag: String,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
}

impl DbcRow for PowerDisplayRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn power_display() {
        let mut file = File::open("../wrath-dbc/PowerDisplay.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = PowerDisplay::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = PowerDisplay::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
