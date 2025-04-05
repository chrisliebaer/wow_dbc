use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::SoundType;

pub type SoundEntriesKey = crate::PrimaryKey<u32, SoundEntries>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundEntries {
    pub rows: Vec<SoundEntriesRow>,
}

impl SoundEntries {
    pub const FILENAME: &'static str = "SoundEntries.dbc";
    pub const FIELD_COUNT: usize = 29;
    pub const ROW_SIZE: usize = 116;

}

impl Into<VanillaTable> for SoundEntries {
    fn into(self) -> VanillaTable {
        VanillaTable::SoundEntries(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SoundEntries {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SoundEntriesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SoundEntriesRow] { &mut self.rows }

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

            // id: primary_key (SoundEntries) uint32
            let id = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?);

            // sound_type: SoundType
            let sound_type = crate::util::read_i32_le(chunk)?.try_into()?;

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // files: string_ref[10]
            let files = {
                let mut arr = Vec::with_capacity(10);
                for _ in 0..10 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // frequency: uint32[10]
            let frequency = crate::util::read_array_u32::<10>(chunk)?;

            // directory_base: string_ref
            let directory_base = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // volume: float
            let volume = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_distance: float
            let min_distance = crate::util::read_f32_le(chunk)?;

            // distance_cutoff: float
            let distance_cutoff = crate::util::read_f32_le(chunk)?;

            // sound_entries_advanced: int32
            let sound_entries_advanced = crate::util::read_i32_le(chunk)?;


            rows.push(SoundEntriesRow {
                id,
                sound_type,
                name,
                files,
                frequency,
                directory_base,
                volume,
                flags,
                min_distance,
                distance_cutoff,
                sound_entries_advanced,
            });
        }

        Ok(SoundEntries { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SoundEntries) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_type: SoundType
            b.write_all(&(row.sound_type.as_int() as i32).to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // files: string_ref[10]
            for i in &row.files {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // frequency: uint32[10]
            for i in row.frequency {
                b.write_all(&i.to_le_bytes())?;
            }


            // directory_base: string_ref
            b.write_all(&string_cache.add_string(&row.directory_base).to_le_bytes())?;

            // volume: float
            b.write_all(&row.volume.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_distance: float
            b.write_all(&row.min_distance.to_le_bytes())?;

            // distance_cutoff: float
            b.write_all(&row.distance_cutoff.to_le_bytes())?;

            // sound_entries_advanced: int32
            b.write_all(&row.sound_entries_advanced.to_le_bytes())?;

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
impl Indexable<u32> for SoundEntries {
    type Table = Self;

    fn get(&self, key: &SoundEntriesKey) -> Option<&SoundEntriesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SoundEntriesKey) -> Option<&mut SoundEntriesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundEntriesRow {
    pub id: SoundEntriesKey,
    pub sound_type: SoundType,
    pub name: String,
    pub files: [String; 10],
    pub frequency: [u32; 10],
    pub directory_base: String,
    pub volume: f32,
    pub flags: i32,
    pub min_distance: f32,
    pub distance_cutoff: f32,
    pub sound_entries_advanced: i32,
}

impl DbcRow for SoundEntriesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_entries() {
        let mut file = File::open("../vanilla-dbc/SoundEntries.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundEntries::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundEntries::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
