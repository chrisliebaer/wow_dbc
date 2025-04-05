use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::vehicle_ui_indicator::{
    VehicleUIIndicator, VehicleUIIndicatorKey,
};
use std::io::Write;
use super::WrathTable;

pub type VehicleUIIndSeatKey = crate::PrimaryKey<i32, VehicleUIIndSeat>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VehicleUIIndSeat {
    pub rows: Vec<VehicleUIIndSeatRow>,
}

impl VehicleUIIndSeat {
    pub const FILENAME: &'static str = "VehicleUIIndSeat.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

    pub fn verify(&self, vehicle_ui_indicator: &VehicleUIIndicator) -> Result<(), crate::InvalidForeignKeyError<&VehicleUIIndSeatRow>> {
        for row in &self.rows {
            if row.vehicle_u_i_indicator_id.id != 0 && vehicle_ui_indicator.get(&row.vehicle_u_i_indicator_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<VehicleUIIndSeat>(),
                    row,
                    id,
                    row.vehicle_u_i_indicator_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for VehicleUIIndSeat {
    fn into(self) -> WrathTable {
        WrathTable::VehicleUIIndSeat(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for VehicleUIIndSeat {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[VehicleUIIndSeatRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [VehicleUIIndSeatRow] { &mut self.rows }

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

            // id: primary_key (VehicleUIIndSeat) int32
            let id = VehicleUIIndSeatKey::new(crate::util::read_i32_le(chunk)?);

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            let vehicle_u_i_indicator_id = VehicleUIIndicatorKey::new(crate::util::read_i32_le(chunk)?.into());

            // virtual_seat_index: int32
            let virtual_seat_index = crate::util::read_i32_le(chunk)?;

            // x_pos: float
            let x_pos = crate::util::read_f32_le(chunk)?;

            // y_pos: float
            let y_pos = crate::util::read_f32_le(chunk)?;


            rows.push(VehicleUIIndSeatRow {
                id,
                vehicle_u_i_indicator_id,
                virtual_seat_index,
                x_pos,
                y_pos,
            });
        }

        Ok(VehicleUIIndSeat { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (VehicleUIIndSeat) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            b.write_all(&(row.vehicle_u_i_indicator_id.id as i32).to_le_bytes())?;

            // virtual_seat_index: int32
            b.write_all(&row.virtual_seat_index.to_le_bytes())?;

            // x_pos: float
            b.write_all(&row.x_pos.to_le_bytes())?;

            // y_pos: float
            b.write_all(&row.y_pos.to_le_bytes())?;

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
impl Indexable<i32> for VehicleUIIndSeat {
    type Table = Self;

    fn get(&self, key: &VehicleUIIndSeatKey) -> Option<&VehicleUIIndSeatRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &VehicleUIIndSeatKey) -> Option<&mut VehicleUIIndSeatRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VehicleUIIndSeatRow {
    pub id: VehicleUIIndSeatKey,
    pub vehicle_u_i_indicator_id: VehicleUIIndicatorKey,
    pub virtual_seat_index: i32,
    pub x_pos: f32,
    pub y_pos: f32,
}

impl DbcRow for VehicleUIIndSeatRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn vehicle_ui_ind_seat() {
        let mut file = File::open("../wrath-dbc/VehicleUIIndSeat.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = VehicleUIIndSeat::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = VehicleUIIndSeat::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
