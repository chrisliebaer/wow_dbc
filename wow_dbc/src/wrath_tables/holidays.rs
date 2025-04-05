use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::holiday_descriptions::{
    HolidayDescriptions, HolidayDescriptionsKey,
};
use crate::wrath_tables::holiday_names::{
    HolidayNames, HolidayNamesKey,
};
use std::io::Write;
use super::WrathTable;

pub type HolidaysKey = crate::PrimaryKey<i32, Holidays>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Holidays {
    pub rows: Vec<HolidaysRow>,
}

impl Holidays {
    pub const FILENAME: &'static str = "Holidays.dbc";
    pub const FIELD_COUNT: usize = 55;
    pub const ROW_SIZE: usize = 220;

    pub fn verify(&self, holiday_descriptions: &HolidayDescriptions, holiday_names: &HolidayNames) -> Result<(), crate::InvalidForeignKeyError<&HolidaysRow>> {
        for row in &self.rows {
            if row.holiday_name_id.id != 0 && holiday_names.get(&row.holiday_name_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Holidays>(),
                    row,
                    id,
                    row.holiday_name_id.id.into()
                ));
            }

            if row.holiday_description_id.id != 0 && holiday_descriptions.get(&row.holiday_description_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Holidays>(),
                    row,
                    id,
                    row.holiday_description_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for Holidays {
    fn into(self) -> WrathTable {
        WrathTable::Holidays(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Holidays {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[HolidaysRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [HolidaysRow] { &mut self.rows }

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

            // id: primary_key (Holidays) int32
            let id = HolidaysKey::new(crate::util::read_i32_le(chunk)?);

            // duration: int32[10]
            let duration = crate::util::read_array_i32::<10>(chunk)?;

            // date: int32[26]
            let date = crate::util::read_array_i32::<26>(chunk)?;

            // region: int32
            let region = crate::util::read_i32_le(chunk)?;

            // looping: int32
            let looping = crate::util::read_i32_le(chunk)?;

            // calendar_flags: int32[10]
            let calendar_flags = crate::util::read_array_i32::<10>(chunk)?;

            // holiday_name_id: foreign_key (HolidayNames) int32
            let holiday_name_id = HolidayNamesKey::new(crate::util::read_i32_le(chunk)?.into());

            // holiday_description_id: foreign_key (HolidayDescriptions) int32
            let holiday_description_id = HolidayDescriptionsKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture_file_name: string_ref
            let texture_file_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // priority: int32
            let priority = crate::util::read_i32_le(chunk)?;

            // calendar_filter_type: int32
            let calendar_filter_type = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(HolidaysRow {
                id,
                duration,
                date,
                region,
                looping,
                calendar_flags,
                holiday_name_id,
                holiday_description_id,
                texture_file_name,
                priority,
                calendar_filter_type,
                flags,
            });
        }

        Ok(Holidays { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Holidays) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // duration: int32[10]
            for i in row.duration {
                b.write_all(&i.to_le_bytes())?;
            }


            // date: int32[26]
            for i in row.date {
                b.write_all(&i.to_le_bytes())?;
            }


            // region: int32
            b.write_all(&row.region.to_le_bytes())?;

            // looping: int32
            b.write_all(&row.looping.to_le_bytes())?;

            // calendar_flags: int32[10]
            for i in row.calendar_flags {
                b.write_all(&i.to_le_bytes())?;
            }


            // holiday_name_id: foreign_key (HolidayNames) int32
            b.write_all(&(row.holiday_name_id.id as i32).to_le_bytes())?;

            // holiday_description_id: foreign_key (HolidayDescriptions) int32
            b.write_all(&(row.holiday_description_id.id as i32).to_le_bytes())?;

            // texture_file_name: string_ref
            b.write_all(&string_cache.add_string(&row.texture_file_name).to_le_bytes())?;

            // priority: int32
            b.write_all(&row.priority.to_le_bytes())?;

            // calendar_filter_type: int32
            b.write_all(&row.calendar_filter_type.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

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
impl Indexable<i32> for Holidays {
    type Table = Self;

    fn get(&self, key: &HolidaysKey) -> Option<&HolidaysRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &HolidaysKey) -> Option<&mut HolidaysRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HolidaysRow {
    pub id: HolidaysKey,
    pub duration: [i32; 10],
    pub date: [i32; 26],
    pub region: i32,
    pub looping: i32,
    pub calendar_flags: [i32; 10],
    pub holiday_name_id: HolidayNamesKey,
    pub holiday_description_id: HolidayDescriptionsKey,
    pub texture_file_name: String,
    pub priority: i32,
    pub calendar_filter_type: i32,
    pub flags: i32,
}

impl DbcRow for HolidaysRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn holidays() {
        let mut file = File::open("../wrath-dbc/Holidays.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Holidays::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Holidays::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
