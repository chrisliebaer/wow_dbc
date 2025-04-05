use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type DestructibleModelDataKey = crate::PrimaryKey<i32, DestructibleModelData>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestructibleModelData {
    pub rows: Vec<DestructibleModelDataRow>,
}

impl DestructibleModelData {
    pub const FILENAME: &'static str = "DestructibleModelData.dbc";
    pub const FIELD_COUNT: usize = 19;
    pub const ROW_SIZE: usize = 76;

}

impl Into<WrathTable> for DestructibleModelData {
    fn into(self) -> WrathTable {
        WrathTable::DestructibleModelData(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for DestructibleModelData {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[DestructibleModelDataRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [DestructibleModelDataRow] { &mut self.rows }

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

            // id: primary_key (DestructibleModelData) int32
            let id = DestructibleModelDataKey::new(crate::util::read_i32_le(chunk)?);

            // state0_impact_effect_doodad_set: int32
            let state0_impact_effect_doodad_set = crate::util::read_i32_le(chunk)?;

            // state0_ambient_doodad_set: int32
            let state0_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // state1_w_m_o: int32
            let state1_w_m_o = crate::util::read_i32_le(chunk)?;

            // state1_destruction_doodad_set: int32
            let state1_destruction_doodad_set = crate::util::read_i32_le(chunk)?;

            // state1_impact_effect_doodad_set: int32
            let state1_impact_effect_doodad_set = crate::util::read_i32_le(chunk)?;

            // state1_ambient_doodad_set: int32
            let state1_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // state2_w_m_o: int32
            let state2_w_m_o = crate::util::read_i32_le(chunk)?;

            // state2_destruction_doodad_set: int32
            let state2_destruction_doodad_set = crate::util::read_i32_le(chunk)?;

            // state2_impact_effect_doodad_set: int32
            let state2_impact_effect_doodad_set = crate::util::read_i32_le(chunk)?;

            // state2_ambient_doodad_set: int32
            let state2_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // state3_w_m_o: int32
            let state3_w_m_o = crate::util::read_i32_le(chunk)?;

            // state3_init_doodad_set: int32
            let state3_init_doodad_set = crate::util::read_i32_le(chunk)?;

            // state3_ambient_doodad_set: int32
            let state3_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // eject_direction: int32
            let eject_direction = crate::util::read_i32_le(chunk)?;

            // repair_ground_fx: int32
            let repair_ground_fx = crate::util::read_i32_le(chunk)?;

            // do_not_highlight: int32
            let do_not_highlight = crate::util::read_i32_le(chunk)?;

            // heal_effect: int32
            let heal_effect = crate::util::read_i32_le(chunk)?;

            // heal_effect_speed: int32
            let heal_effect_speed = crate::util::read_i32_le(chunk)?;


            rows.push(DestructibleModelDataRow {
                id,
                state0_impact_effect_doodad_set,
                state0_ambient_doodad_set,
                state1_w_m_o,
                state1_destruction_doodad_set,
                state1_impact_effect_doodad_set,
                state1_ambient_doodad_set,
                state2_w_m_o,
                state2_destruction_doodad_set,
                state2_impact_effect_doodad_set,
                state2_ambient_doodad_set,
                state3_w_m_o,
                state3_init_doodad_set,
                state3_ambient_doodad_set,
                eject_direction,
                repair_ground_fx,
                do_not_highlight,
                heal_effect,
                heal_effect_speed,
            });
        }

        Ok(DestructibleModelData { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (DestructibleModelData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // state0_impact_effect_doodad_set: int32
            b.write_all(&row.state0_impact_effect_doodad_set.to_le_bytes())?;

            // state0_ambient_doodad_set: int32
            b.write_all(&row.state0_ambient_doodad_set.to_le_bytes())?;

            // state1_w_m_o: int32
            b.write_all(&row.state1_w_m_o.to_le_bytes())?;

            // state1_destruction_doodad_set: int32
            b.write_all(&row.state1_destruction_doodad_set.to_le_bytes())?;

            // state1_impact_effect_doodad_set: int32
            b.write_all(&row.state1_impact_effect_doodad_set.to_le_bytes())?;

            // state1_ambient_doodad_set: int32
            b.write_all(&row.state1_ambient_doodad_set.to_le_bytes())?;

            // state2_w_m_o: int32
            b.write_all(&row.state2_w_m_o.to_le_bytes())?;

            // state2_destruction_doodad_set: int32
            b.write_all(&row.state2_destruction_doodad_set.to_le_bytes())?;

            // state2_impact_effect_doodad_set: int32
            b.write_all(&row.state2_impact_effect_doodad_set.to_le_bytes())?;

            // state2_ambient_doodad_set: int32
            b.write_all(&row.state2_ambient_doodad_set.to_le_bytes())?;

            // state3_w_m_o: int32
            b.write_all(&row.state3_w_m_o.to_le_bytes())?;

            // state3_init_doodad_set: int32
            b.write_all(&row.state3_init_doodad_set.to_le_bytes())?;

            // state3_ambient_doodad_set: int32
            b.write_all(&row.state3_ambient_doodad_set.to_le_bytes())?;

            // eject_direction: int32
            b.write_all(&row.eject_direction.to_le_bytes())?;

            // repair_ground_fx: int32
            b.write_all(&row.repair_ground_fx.to_le_bytes())?;

            // do_not_highlight: int32
            b.write_all(&row.do_not_highlight.to_le_bytes())?;

            // heal_effect: int32
            b.write_all(&row.heal_effect.to_le_bytes())?;

            // heal_effect_speed: int32
            b.write_all(&row.heal_effect_speed.to_le_bytes())?;

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
impl Indexable<i32> for DestructibleModelData {
    type Table = Self;

    fn get(&self, key: &DestructibleModelDataKey) -> Option<&DestructibleModelDataRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &DestructibleModelDataKey) -> Option<&mut DestructibleModelDataRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DestructibleModelDataRow {
    pub id: DestructibleModelDataKey,
    pub state0_impact_effect_doodad_set: i32,
    pub state0_ambient_doodad_set: i32,
    pub state1_w_m_o: i32,
    pub state1_destruction_doodad_set: i32,
    pub state1_impact_effect_doodad_set: i32,
    pub state1_ambient_doodad_set: i32,
    pub state2_w_m_o: i32,
    pub state2_destruction_doodad_set: i32,
    pub state2_impact_effect_doodad_set: i32,
    pub state2_ambient_doodad_set: i32,
    pub state3_w_m_o: i32,
    pub state3_init_doodad_set: i32,
    pub state3_ambient_doodad_set: i32,
    pub eject_direction: i32,
    pub repair_ground_fx: i32,
    pub do_not_highlight: i32,
    pub heal_effect: i32,
    pub heal_effect_speed: i32,
}

impl DbcRow for DestructibleModelDataRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn destructible_model_data() {
        let mut file = File::open("../wrath-dbc/DestructibleModelData.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DestructibleModelData::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DestructibleModelData::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
