use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::animation_data::{
    AnimationData, AnimationDataKey,
};
use std::io::Write;
use super::VanillaTable;

pub type TransportAnimationKey = crate::PrimaryKey<u32, TransportAnimation>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransportAnimation {
    pub rows: Vec<TransportAnimationRow>,
}

impl TransportAnimation {
    pub const FILENAME: &'static str = "TransportAnimation.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, animation_data: &AnimationData) -> Result<(), crate::InvalidForeignKeyError<&TransportAnimationRow>> {
        for row in &self.rows {
            if row.sequence.id != 0 && animation_data.get(&row.sequence).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<TransportAnimation>(),
                    row,
                    id,
                    row.sequence.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for TransportAnimation {
    fn into(self) -> VanillaTable {
        VanillaTable::TransportAnimation(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TransportAnimation {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TransportAnimationRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TransportAnimationRow] { &mut self.rows }

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

            // id: primary_key (TransportAnimation) uint32
            let id = TransportAnimationKey::new(crate::util::read_u32_le(chunk)?);

            // transport: uint32
            let transport = crate::util::read_u32_le(chunk)?;

            // time_index: int32
            let time_index = crate::util::read_i32_le(chunk)?;

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // sequence: foreign_key (AnimationData) uint32
            let sequence = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(TransportAnimationRow {
                id,
                transport,
                time_index,
                location_x,
                location_y,
                location_z,
                sequence,
            });
        }

        Ok(TransportAnimation { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TransportAnimation) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // transport: uint32
            b.write_all(&row.transport.to_le_bytes())?;

            // time_index: int32
            b.write_all(&row.time_index.to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // sequence: foreign_key (AnimationData) uint32
            b.write_all(&(row.sequence.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for TransportAnimation {
    type Table = Self;

    fn get(&self, key: &TransportAnimationKey) -> Option<&TransportAnimationRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TransportAnimationKey) -> Option<&mut TransportAnimationRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransportAnimationRow {
    pub id: TransportAnimationKey,
    pub transport: u32,
    pub time_index: i32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub sequence: AnimationDataKey,
}

impl DbcRow for TransportAnimationRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn transport_animation() {
        let mut file = File::open("../vanilla-dbc/TransportAnimation.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TransportAnimation::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TransportAnimation::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
