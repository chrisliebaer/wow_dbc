use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type TransportPhysicsKey = crate::PrimaryKey<i32, TransportPhysics>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransportPhysics {
    pub rows: Vec<TransportPhysicsRow>,
}

impl TransportPhysics {
    pub const FILENAME: &'static str = "TransportPhysics.dbc";
    pub const FIELD_COUNT: usize = 11;
    pub const ROW_SIZE: usize = 44;

}

impl Into<WrathTable> for TransportPhysics {
    fn into(self) -> WrathTable {
        WrathTable::TransportPhysics(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for TransportPhysics {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[TransportPhysicsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [TransportPhysicsRow] { &mut self.rows }

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

            // id: primary_key (TransportPhysics) int32
            let id = TransportPhysicsKey::new(crate::util::read_i32_le(chunk)?);

            // wave_amp: float
            let wave_amp = crate::util::read_f32_le(chunk)?;

            // wave_time_scale: float
            let wave_time_scale = crate::util::read_f32_le(chunk)?;

            // roll_amp: float
            let roll_amp = crate::util::read_f32_le(chunk)?;

            // roll_time_scale: float
            let roll_time_scale = crate::util::read_f32_le(chunk)?;

            // pitch_amp: float
            let pitch_amp = crate::util::read_f32_le(chunk)?;

            // pitch_time_scale: float
            let pitch_time_scale = crate::util::read_f32_le(chunk)?;

            // max_bank: float
            let max_bank = crate::util::read_f32_le(chunk)?;

            // max_bank_turn_speed: float
            let max_bank_turn_speed = crate::util::read_f32_le(chunk)?;

            // speed_damp_thresh: float
            let speed_damp_thresh = crate::util::read_f32_le(chunk)?;

            // speed_damp: float
            let speed_damp = crate::util::read_f32_le(chunk)?;


            rows.push(TransportPhysicsRow {
                id,
                wave_amp,
                wave_time_scale,
                roll_amp,
                roll_time_scale,
                pitch_amp,
                pitch_time_scale,
                max_bank,
                max_bank_turn_speed,
                speed_damp_thresh,
                speed_damp,
            });
        }

        Ok(TransportPhysics { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (TransportPhysics) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // wave_amp: float
            b.write_all(&row.wave_amp.to_le_bytes())?;

            // wave_time_scale: float
            b.write_all(&row.wave_time_scale.to_le_bytes())?;

            // roll_amp: float
            b.write_all(&row.roll_amp.to_le_bytes())?;

            // roll_time_scale: float
            b.write_all(&row.roll_time_scale.to_le_bytes())?;

            // pitch_amp: float
            b.write_all(&row.pitch_amp.to_le_bytes())?;

            // pitch_time_scale: float
            b.write_all(&row.pitch_time_scale.to_le_bytes())?;

            // max_bank: float
            b.write_all(&row.max_bank.to_le_bytes())?;

            // max_bank_turn_speed: float
            b.write_all(&row.max_bank_turn_speed.to_le_bytes())?;

            // speed_damp_thresh: float
            b.write_all(&row.speed_damp_thresh.to_le_bytes())?;

            // speed_damp: float
            b.write_all(&row.speed_damp.to_le_bytes())?;

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
impl Indexable<i32> for TransportPhysics {
    type Table = Self;

    fn get(&self, key: &TransportPhysicsKey) -> Option<&TransportPhysicsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &TransportPhysicsKey) -> Option<&mut TransportPhysicsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransportPhysicsRow {
    pub id: TransportPhysicsKey,
    pub wave_amp: f32,
    pub wave_time_scale: f32,
    pub roll_amp: f32,
    pub roll_time_scale: f32,
    pub pitch_amp: f32,
    pub pitch_time_scale: f32,
    pub max_bank: f32,
    pub max_bank_turn_speed: f32,
    pub speed_damp_thresh: f32,
    pub speed_damp: f32,
}

impl DbcRow for TransportPhysicsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn transport_physics() {
        let mut file = File::open("../wrath-dbc/TransportPhysics.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TransportPhysics::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TransportPhysics::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
