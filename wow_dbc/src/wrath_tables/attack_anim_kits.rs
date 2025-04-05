use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type AttackAnimKitsKey = crate::PrimaryKey<i32, AttackAnimKits>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttackAnimKits {
    pub rows: Vec<AttackAnimKitsRow>,
}

impl AttackAnimKits {
    pub const FILENAME: &'static str = "AttackAnimKits.dbc";
    pub const FIELD_COUNT: usize = 5;
    pub const ROW_SIZE: usize = 20;

}

impl Into<WrathTable> for AttackAnimKits {
    fn into(self) -> WrathTable {
        WrathTable::AttackAnimKits(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AttackAnimKits {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AttackAnimKitsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AttackAnimKitsRow] { &mut self.rows }

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

            // id: primary_key (AttackAnimKits) int32
            let id = AttackAnimKitsKey::new(crate::util::read_i32_le(chunk)?);

            // item_subclass_id: int32
            let item_subclass_id = crate::util::read_i32_le(chunk)?;

            // anim_type_id: int32
            let anim_type_id = crate::util::read_i32_le(chunk)?;

            // anim_frequency: int32
            let anim_frequency = crate::util::read_i32_le(chunk)?;

            // which_hand: int32
            let which_hand = crate::util::read_i32_le(chunk)?;


            rows.push(AttackAnimKitsRow {
                id,
                item_subclass_id,
                anim_type_id,
                anim_frequency,
                which_hand,
            });
        }

        Ok(AttackAnimKits { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AttackAnimKits) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_subclass_id: int32
            b.write_all(&row.item_subclass_id.to_le_bytes())?;

            // anim_type_id: int32
            b.write_all(&row.anim_type_id.to_le_bytes())?;

            // anim_frequency: int32
            b.write_all(&row.anim_frequency.to_le_bytes())?;

            // which_hand: int32
            b.write_all(&row.which_hand.to_le_bytes())?;

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
impl Indexable<i32> for AttackAnimKits {
    type Table = Self;

    fn get(&self, key: &AttackAnimKitsKey) -> Option<&AttackAnimKitsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AttackAnimKitsKey) -> Option<&mut AttackAnimKitsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttackAnimKitsRow {
    pub id: AttackAnimKitsKey,
    pub item_subclass_id: i32,
    pub anim_type_id: i32,
    pub anim_frequency: i32,
    pub which_hand: i32,
}

impl DbcRow for AttackAnimKitsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn attack_anim_kits() {
        let mut file = File::open("../wrath-dbc/AttackAnimKits.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AttackAnimKits::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AttackAnimKits::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
