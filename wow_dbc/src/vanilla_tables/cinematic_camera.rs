use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::sound_entries::{
    SoundEntries, SoundEntriesKey,
};
use std::io::Write;
use super::VanillaTable;

pub type CinematicCameraKey = crate::PrimaryKey<u32, CinematicCamera>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CinematicCamera {
    pub rows: Vec<CinematicCameraRow>,
}

impl CinematicCamera {
    pub const FILENAME: &'static str = "CinematicCamera.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, sound_entries: &SoundEntries) -> Result<(), crate::InvalidForeignKeyError<&CinematicCameraRow>> {
        for row in &self.rows {
            if row.sound_entry.id != 0 && sound_entries.get(&row.sound_entry).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CinematicCamera>(),
                    row,
                    id,
                    row.sound_entry.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CinematicCamera {
    fn into(self) -> VanillaTable {
        VanillaTable::CinematicCamera(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CinematicCamera {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CinematicCameraRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CinematicCameraRow] { &mut self.rows }

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

            // id: primary_key (CinematicCamera) uint32
            let id = CinematicCameraKey::new(crate::util::read_u32_le(chunk)?);

            // model: string_ref
            let model = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // sound_entry: foreign_key (SoundEntries) uint32
            let sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // rotation: float
            let rotation = crate::util::read_f32_le(chunk)?;


            rows.push(CinematicCameraRow {
                id,
                model,
                sound_entry,
                location_x,
                location_y,
                location_z,
                rotation,
            });
        }

        Ok(CinematicCamera { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CinematicCamera) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model: string_ref
            b.write_all(&string_cache.add_string(&row.model).to_le_bytes())?;

            // sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_entry.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // rotation: float
            b.write_all(&row.rotation.to_le_bytes())?;

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
impl Indexable<u32> for CinematicCamera {
    type Table = Self;

    fn get(&self, key: &CinematicCameraKey) -> Option<&CinematicCameraRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CinematicCameraKey) -> Option<&mut CinematicCameraRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CinematicCameraRow {
    pub id: CinematicCameraKey,
    pub model: String,
    pub sound_entry: SoundEntriesKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub rotation: f32,
}

impl DbcRow for CinematicCameraRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn cinematic_camera() {
        let mut file = File::open("../vanilla-dbc/CinematicCamera.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CinematicCamera::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CinematicCamera::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
