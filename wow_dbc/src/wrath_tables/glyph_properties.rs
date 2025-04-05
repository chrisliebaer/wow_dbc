use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::spell::{
    Spell, SpellKey,
};
use crate::wrath_tables::spell_icon::{
    SpellIcon, SpellIconKey,
};
use std::io::Write;
use super::WrathTable;

pub type GlyphPropertiesKey = crate::PrimaryKey<i32, GlyphProperties>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlyphProperties {
    pub rows: Vec<GlyphPropertiesRow>,
}

impl GlyphProperties {
    pub const FILENAME: &'static str = "GlyphProperties.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, spell: &Spell, spell_icon: &SpellIcon) -> Result<(), crate::InvalidForeignKeyError<&GlyphPropertiesRow>> {
        for row in &self.rows {
            if row.spell_id.id != 0 && spell.get(&row.spell_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<GlyphProperties>(),
                    row,
                    id,
                    row.spell_id.id.into()
                ));
            }

            if row.spell_icon_id.id != 0 && spell_icon.get(&row.spell_icon_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<GlyphProperties>(),
                    row,
                    id,
                    row.spell_icon_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for GlyphProperties {
    fn into(self) -> WrathTable {
        WrathTable::GlyphProperties(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GlyphProperties {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GlyphPropertiesRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GlyphPropertiesRow] { &mut self.rows }

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

            // id: primary_key (GlyphProperties) int32
            let id = GlyphPropertiesKey::new(crate::util::read_i32_le(chunk)?);

            // spell_id: foreign_key (Spell) int32
            let spell_id = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // glyph_slot_flags: int32
            let glyph_slot_flags = crate::util::read_i32_le(chunk)?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(GlyphPropertiesRow {
                id,
                spell_id,
                glyph_slot_flags,
                spell_icon_id,
            });
        }

        Ok(GlyphProperties { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GlyphProperties) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // spell_id: foreign_key (Spell) int32
            b.write_all(&(row.spell_id.id as i32).to_le_bytes())?;

            // glyph_slot_flags: int32
            b.write_all(&row.glyph_slot_flags.to_le_bytes())?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.spell_icon_id.id as i32).to_le_bytes())?;

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
impl Indexable<i32> for GlyphProperties {
    type Table = Self;

    fn get(&self, key: &GlyphPropertiesKey) -> Option<&GlyphPropertiesRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GlyphPropertiesKey) -> Option<&mut GlyphPropertiesRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlyphPropertiesRow {
    pub id: GlyphPropertiesKey,
    pub spell_id: SpellKey,
    pub glyph_slot_flags: i32,
    pub spell_icon_id: SpellIconKey,
}

impl DbcRow for GlyphPropertiesRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn glyph_properties() {
        let mut file = File::open("../wrath-dbc/GlyphProperties.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GlyphProperties::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GlyphProperties::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
