use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;

pub type PetPersonalityKey = crate::PrimaryKey<u32, PetPersonality>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PetPersonality {
    pub rows: Vec<PetPersonalityRow>,
}

impl PetPersonality {
    pub const FILENAME: &'static str = "PetPersonality.dbc";
    pub const FIELD_COUNT: usize = 19;
    pub const ROW_SIZE: usize = 76;

}

impl Into<VanillaTable> for PetPersonality {
    fn into(self) -> VanillaTable {
        VanillaTable::PetPersonality(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for PetPersonality {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[PetPersonalityRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [PetPersonalityRow] { &mut self.rows }

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

            // id: primary_key (PetPersonality) uint32
            let id = PetPersonalityKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // threshold_unhappy: int32
            let threshold_unhappy = crate::util::read_i32_le(chunk)?;

            // threshold_content: int32
            let threshold_content = crate::util::read_i32_le(chunk)?;

            // threshold_happy: int32
            let threshold_happy = crate::util::read_i32_le(chunk)?;

            // damage_unhappy: float
            let damage_unhappy = crate::util::read_f32_le(chunk)?;

            // damage_content: float
            let damage_content = crate::util::read_f32_le(chunk)?;

            // damage_happy: float
            let damage_happy = crate::util::read_f32_le(chunk)?;

            // modifier_unhappy: float
            let modifier_unhappy = crate::util::read_f32_le(chunk)?;

            // modifier_content: float
            let modifier_content = crate::util::read_f32_le(chunk)?;

            // modifier_happy: float
            let modifier_happy = crate::util::read_f32_le(chunk)?;


            rows.push(PetPersonalityRow {
                id,
                name,
                threshold_unhappy,
                threshold_content,
                threshold_happy,
                damage_unhappy,
                damage_content,
                damage_happy,
                modifier_unhappy,
                modifier_content,
                modifier_happy,
            });
        }

        Ok(PetPersonality { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (PetPersonality) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // threshold_unhappy: int32
            b.write_all(&row.threshold_unhappy.to_le_bytes())?;

            // threshold_content: int32
            b.write_all(&row.threshold_content.to_le_bytes())?;

            // threshold_happy: int32
            b.write_all(&row.threshold_happy.to_le_bytes())?;

            // damage_unhappy: float
            b.write_all(&row.damage_unhappy.to_le_bytes())?;

            // damage_content: float
            b.write_all(&row.damage_content.to_le_bytes())?;

            // damage_happy: float
            b.write_all(&row.damage_happy.to_le_bytes())?;

            // modifier_unhappy: float
            b.write_all(&row.modifier_unhappy.to_le_bytes())?;

            // modifier_content: float
            b.write_all(&row.modifier_content.to_le_bytes())?;

            // modifier_happy: float
            b.write_all(&row.modifier_happy.to_le_bytes())?;

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
impl Indexable<u32> for PetPersonality {
    type Table = Self;

    fn get(&self, key: &PetPersonalityKey) -> Option<&PetPersonalityRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &PetPersonalityKey) -> Option<&mut PetPersonalityRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name: LocalizedString,
    pub threshold_unhappy: i32,
    pub threshold_content: i32,
    pub threshold_happy: i32,
    pub damage_unhappy: f32,
    pub damage_content: f32,
    pub damage_happy: f32,
    pub modifier_unhappy: f32,
    pub modifier_content: f32,
    pub modifier_happy: f32,
}

impl DbcRow for PetPersonalityRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn pet_personality() {
        let mut file = File::open("../vanilla-dbc/PetPersonality.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = PetPersonality::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = PetPersonality::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
