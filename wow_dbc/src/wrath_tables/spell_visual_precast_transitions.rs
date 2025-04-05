use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type SpellVisualPrecastTransitionsKey = crate::PrimaryKey<i32, SpellVisualPrecastTransitions>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualPrecastTransitions {
    pub rows: Vec<SpellVisualPrecastTransitionsRow>,
}

impl SpellVisualPrecastTransitions {
    pub const FILENAME: &'static str = "SpellVisualPrecastTransitions.dbc";
    pub const FIELD_COUNT: usize = 3;
    pub const ROW_SIZE: usize = 12;

}

impl Into<WrathTable> for SpellVisualPrecastTransitions {
    fn into(self) -> WrathTable {
        WrathTable::SpellVisualPrecastTransitions(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellVisualPrecastTransitions {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellVisualPrecastTransitionsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellVisualPrecastTransitionsRow] { &mut self.rows }

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

            // id: primary_key (SpellVisualPrecastTransitions) int32
            let id = SpellVisualPrecastTransitionsKey::new(crate::util::read_i32_le(chunk)?);

            // precast_load_anim_name: string_ref
            let precast_load_anim_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // precast_hold_anim_name: string_ref
            let precast_hold_anim_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SpellVisualPrecastTransitionsRow {
                id,
                precast_load_anim_name,
                precast_hold_anim_name,
            });
        }

        Ok(SpellVisualPrecastTransitions { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellVisualPrecastTransitions) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // precast_load_anim_name: string_ref
            b.write_all(&string_cache.add_string(&row.precast_load_anim_name).to_le_bytes())?;

            // precast_hold_anim_name: string_ref
            b.write_all(&string_cache.add_string(&row.precast_hold_anim_name).to_le_bytes())?;

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
impl Indexable<i32> for SpellVisualPrecastTransitions {
    type Table = Self;

    fn get(&self, key: &SpellVisualPrecastTransitionsKey) -> Option<&SpellVisualPrecastTransitionsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellVisualPrecastTransitionsKey) -> Option<&mut SpellVisualPrecastTransitionsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualPrecastTransitionsRow {
    pub id: SpellVisualPrecastTransitionsKey,
    pub precast_load_anim_name: String,
    pub precast_hold_anim_name: String,
}

impl DbcRow for SpellVisualPrecastTransitionsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_visual_precast_transitions() {
        let mut file = File::open("../wrath-dbc/SpellVisualPrecastTransitions.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellVisualPrecastTransitions::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellVisualPrecastTransitions::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
