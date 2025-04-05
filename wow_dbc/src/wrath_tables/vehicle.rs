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

pub type VehicleKey = crate::PrimaryKey<i32, Vehicle>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vehicle {
    pub rows: Vec<VehicleRow>,
}

impl Vehicle {
    pub const FILENAME: &'static str = "Vehicle.dbc";
    pub const FIELD_COUNT: usize = 40;
    pub const ROW_SIZE: usize = 160;

    pub fn verify(&self, vehicle_ui_indicator: &VehicleUIIndicator) -> Result<(), crate::InvalidForeignKeyError<&VehicleRow>> {
        for row in &self.rows {
            if row.vehicle_u_i_indicator_id.id != 0 && vehicle_ui_indicator.get(&row.vehicle_u_i_indicator_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Vehicle>(),
                    row,
                    id,
                    row.vehicle_u_i_indicator_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for Vehicle {
    fn into(self) -> WrathTable {
        WrathTable::Vehicle(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Vehicle {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[VehicleRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [VehicleRow] { &mut self.rows }

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

            // id: primary_key (Vehicle) int32
            let id = VehicleKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // turn_speed: float
            let turn_speed = crate::util::read_f32_le(chunk)?;

            // pitch_speed: float
            let pitch_speed = crate::util::read_f32_le(chunk)?;

            // pitch_min: float
            let pitch_min = crate::util::read_f32_le(chunk)?;

            // pitch_max: float
            let pitch_max = crate::util::read_f32_le(chunk)?;

            // seat_id: int32[8]
            let seat_id = crate::util::read_array_i32::<8>(chunk)?;

            // mouse_look_offset_pitch: float
            let mouse_look_offset_pitch = crate::util::read_f32_le(chunk)?;

            // camera_fade_dist_scalar_min: float
            let camera_fade_dist_scalar_min = crate::util::read_f32_le(chunk)?;

            // camera_fade_dist_scalar_max: float
            let camera_fade_dist_scalar_max = crate::util::read_f32_le(chunk)?;

            // camera_pitch_offset: float
            let camera_pitch_offset = crate::util::read_f32_le(chunk)?;

            // facing_limit_right: float
            let facing_limit_right = crate::util::read_f32_le(chunk)?;

            // facing_limit_left: float
            let facing_limit_left = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_turn_lingering: float
            let mssl_trgt_turn_lingering = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_pitch_lingering: float
            let mssl_trgt_pitch_lingering = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_mouse_lingering: float
            let mssl_trgt_mouse_lingering = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_end_opacity: float
            let mssl_trgt_end_opacity = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_arc_speed: float
            let mssl_trgt_arc_speed = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_arc_repeat: float
            let mssl_trgt_arc_repeat = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_arc_width: float
            let mssl_trgt_arc_width = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_impact_radius: float[2]
            let mssl_trgt_impact_radius = crate::util::read_array_f32::<2>(chunk)?;

            // mssl_trgt_arc_texture: string_ref
            let mssl_trgt_arc_texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // mssl_trgt_impact_texture: string_ref
            let mssl_trgt_impact_texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // mssl_trgt_impact_model: string_ref[2]
            let mssl_trgt_impact_model = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // camera_yaw_offset: float
            let camera_yaw_offset = crate::util::read_f32_le(chunk)?;

            // ui_locomotion_type: int32
            let ui_locomotion_type = crate::util::read_i32_le(chunk)?;

            // mssl_trgt_impact_tex_radius: float
            let mssl_trgt_impact_tex_radius = crate::util::read_f32_le(chunk)?;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            let vehicle_u_i_indicator_id = VehicleUIIndicatorKey::new(crate::util::read_i32_le(chunk)?.into());

            // power_display_id: int32[3]
            let power_display_id = crate::util::read_array_i32::<3>(chunk)?;


            rows.push(VehicleRow {
                id,
                flags,
                turn_speed,
                pitch_speed,
                pitch_min,
                pitch_max,
                seat_id,
                mouse_look_offset_pitch,
                camera_fade_dist_scalar_min,
                camera_fade_dist_scalar_max,
                camera_pitch_offset,
                facing_limit_right,
                facing_limit_left,
                mssl_trgt_turn_lingering,
                mssl_trgt_pitch_lingering,
                mssl_trgt_mouse_lingering,
                mssl_trgt_end_opacity,
                mssl_trgt_arc_speed,
                mssl_trgt_arc_repeat,
                mssl_trgt_arc_width,
                mssl_trgt_impact_radius,
                mssl_trgt_arc_texture,
                mssl_trgt_impact_texture,
                mssl_trgt_impact_model,
                camera_yaw_offset,
                ui_locomotion_type,
                mssl_trgt_impact_tex_radius,
                vehicle_u_i_indicator_id,
                power_display_id,
            });
        }

        Ok(Vehicle { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Vehicle) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // turn_speed: float
            b.write_all(&row.turn_speed.to_le_bytes())?;

            // pitch_speed: float
            b.write_all(&row.pitch_speed.to_le_bytes())?;

            // pitch_min: float
            b.write_all(&row.pitch_min.to_le_bytes())?;

            // pitch_max: float
            b.write_all(&row.pitch_max.to_le_bytes())?;

            // seat_id: int32[8]
            for i in row.seat_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // mouse_look_offset_pitch: float
            b.write_all(&row.mouse_look_offset_pitch.to_le_bytes())?;

            // camera_fade_dist_scalar_min: float
            b.write_all(&row.camera_fade_dist_scalar_min.to_le_bytes())?;

            // camera_fade_dist_scalar_max: float
            b.write_all(&row.camera_fade_dist_scalar_max.to_le_bytes())?;

            // camera_pitch_offset: float
            b.write_all(&row.camera_pitch_offset.to_le_bytes())?;

            // facing_limit_right: float
            b.write_all(&row.facing_limit_right.to_le_bytes())?;

            // facing_limit_left: float
            b.write_all(&row.facing_limit_left.to_le_bytes())?;

            // mssl_trgt_turn_lingering: float
            b.write_all(&row.mssl_trgt_turn_lingering.to_le_bytes())?;

            // mssl_trgt_pitch_lingering: float
            b.write_all(&row.mssl_trgt_pitch_lingering.to_le_bytes())?;

            // mssl_trgt_mouse_lingering: float
            b.write_all(&row.mssl_trgt_mouse_lingering.to_le_bytes())?;

            // mssl_trgt_end_opacity: float
            b.write_all(&row.mssl_trgt_end_opacity.to_le_bytes())?;

            // mssl_trgt_arc_speed: float
            b.write_all(&row.mssl_trgt_arc_speed.to_le_bytes())?;

            // mssl_trgt_arc_repeat: float
            b.write_all(&row.mssl_trgt_arc_repeat.to_le_bytes())?;

            // mssl_trgt_arc_width: float
            b.write_all(&row.mssl_trgt_arc_width.to_le_bytes())?;

            // mssl_trgt_impact_radius: float[2]
            for i in row.mssl_trgt_impact_radius {
                b.write_all(&i.to_le_bytes())?;
            }


            // mssl_trgt_arc_texture: string_ref
            b.write_all(&string_cache.add_string(&row.mssl_trgt_arc_texture).to_le_bytes())?;

            // mssl_trgt_impact_texture: string_ref
            b.write_all(&string_cache.add_string(&row.mssl_trgt_impact_texture).to_le_bytes())?;

            // mssl_trgt_impact_model: string_ref[2]
            for i in &row.mssl_trgt_impact_model {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // camera_yaw_offset: float
            b.write_all(&row.camera_yaw_offset.to_le_bytes())?;

            // ui_locomotion_type: int32
            b.write_all(&row.ui_locomotion_type.to_le_bytes())?;

            // mssl_trgt_impact_tex_radius: float
            b.write_all(&row.mssl_trgt_impact_tex_radius.to_le_bytes())?;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            b.write_all(&(row.vehicle_u_i_indicator_id.id as i32).to_le_bytes())?;

            // power_display_id: int32[3]
            for i in row.power_display_id {
                b.write_all(&i.to_le_bytes())?;
            }


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
impl Indexable<i32> for Vehicle {
    type Table = Self;

    fn get(&self, key: &VehicleKey) -> Option<&VehicleRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &VehicleKey) -> Option<&mut VehicleRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VehicleRow {
    pub id: VehicleKey,
    pub flags: i32,
    pub turn_speed: f32,
    pub pitch_speed: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub seat_id: [i32; 8],
    pub mouse_look_offset_pitch: f32,
    pub camera_fade_dist_scalar_min: f32,
    pub camera_fade_dist_scalar_max: f32,
    pub camera_pitch_offset: f32,
    pub facing_limit_right: f32,
    pub facing_limit_left: f32,
    pub mssl_trgt_turn_lingering: f32,
    pub mssl_trgt_pitch_lingering: f32,
    pub mssl_trgt_mouse_lingering: f32,
    pub mssl_trgt_end_opacity: f32,
    pub mssl_trgt_arc_speed: f32,
    pub mssl_trgt_arc_repeat: f32,
    pub mssl_trgt_arc_width: f32,
    pub mssl_trgt_impact_radius: [f32; 2],
    pub mssl_trgt_arc_texture: String,
    pub mssl_trgt_impact_texture: String,
    pub mssl_trgt_impact_model: [String; 2],
    pub camera_yaw_offset: f32,
    pub ui_locomotion_type: i32,
    pub mssl_trgt_impact_tex_radius: f32,
    pub vehicle_u_i_indicator_id: VehicleUIIndicatorKey,
    pub power_display_id: [i32; 3],
}

impl DbcRow for VehicleRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn vehicle() {
        let mut file = File::open("../wrath-dbc/Vehicle.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Vehicle::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Vehicle::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
