use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;

pub type SpellChainEffectsKey = crate::PrimaryKey<u32, SpellChainEffects>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellChainEffects {
    pub rows: Vec<SpellChainEffectsRow>,
}

impl SpellChainEffects {
    pub const FILENAME: &'static str = "SpellChainEffects.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

}

impl Into<VanillaTable> for SpellChainEffects {
    fn into(self) -> VanillaTable {
        VanillaTable::SpellChainEffects(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for SpellChainEffects {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[SpellChainEffectsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [SpellChainEffectsRow] { &mut self.rows }

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

            // id: primary_key (SpellChainEffects) uint32
            let id = SpellChainEffectsKey::new(crate::util::read_u32_le(chunk)?);

            // average_seg_len: float
            let average_seg_len = crate::util::read_f32_le(chunk)?;

            // width: float
            let width = crate::util::read_f32_le(chunk)?;

            // noise_scale: float
            let noise_scale = crate::util::read_f32_le(chunk)?;

            // tex_coord_scale: float
            let tex_coord_scale = crate::util::read_f32_le(chunk)?;

            // seg_duration: int32
            let seg_duration = crate::util::read_i32_le(chunk)?;

            // seg_delay: int32
            let seg_delay = crate::util::read_i32_le(chunk)?;

            // texture: string_ref
            let texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SpellChainEffectsRow {
                id,
                average_seg_len,
                width,
                noise_scale,
                tex_coord_scale,
                seg_duration,
                seg_delay,
                texture,
            });
        }

        Ok(SpellChainEffects { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SpellChainEffects) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // average_seg_len: float
            b.write_all(&row.average_seg_len.to_le_bytes())?;

            // width: float
            b.write_all(&row.width.to_le_bytes())?;

            // noise_scale: float
            b.write_all(&row.noise_scale.to_le_bytes())?;

            // tex_coord_scale: float
            b.write_all(&row.tex_coord_scale.to_le_bytes())?;

            // seg_duration: int32
            b.write_all(&row.seg_duration.to_le_bytes())?;

            // seg_delay: int32
            b.write_all(&row.seg_delay.to_le_bytes())?;

            // texture: string_ref
            b.write_all(&string_cache.add_string(&row.texture).to_le_bytes())?;

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
impl Indexable<u32> for SpellChainEffects {
    type Table = Self;

    fn get(&self, key: &SpellChainEffectsKey) -> Option<&SpellChainEffectsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &SpellChainEffectsKey) -> Option<&mut SpellChainEffectsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellChainEffectsRow {
    pub id: SpellChainEffectsKey,
    pub average_seg_len: f32,
    pub width: f32,
    pub noise_scale: f32,
    pub tex_coord_scale: f32,
    pub seg_duration: i32,
    pub seg_delay: i32,
    pub texture: String,
}

impl DbcRow for SpellChainEffectsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_chain_effects() {
        let mut file = File::open("../vanilla-dbc/SpellChainEffects.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellChainEffects::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellChainEffects::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
