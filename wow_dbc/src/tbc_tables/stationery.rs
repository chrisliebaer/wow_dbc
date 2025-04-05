use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::item::{
    Item, ItemKey,
};
use crate::util::StringCache;
use std::io::Write;
use super::TbcTable;

pub type StationeryKey = crate::PrimaryKey<i32, Stationery>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stationery {
    pub rows: Vec<StationeryRow>,
}

impl Stationery {
    pub const FILENAME: &'static str = "Stationery.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

    pub fn verify(&self, item: &Item) -> Result<(), crate::InvalidForeignKeyError<&StationeryRow>> {
        for row in &self.rows {
            if row.item_id.id != 0 && item.get(&row.item_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<Stationery>(),
                    row,
                    id,
                    row.item_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<TbcTable> for Stationery {
    fn into(self) -> TbcTable {
        TbcTable::Stationery(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Stationery {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[StationeryRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [StationeryRow] { &mut self.rows }

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

            // id: primary_key (Stationery) int32
            let id = StationeryKey::new(crate::util::read_i32_le(chunk)?);

            // item_id: foreign_key (Item) int32
            let item_id = ItemKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture: string_ref
            let texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(StationeryRow {
                id,
                item_id,
                texture,
                flags,
            });
        }

        Ok(Stationery { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Stationery) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_id: foreign_key (Item) int32
            b.write_all(&(row.item_id.id as i32).to_le_bytes())?;

            // texture: string_ref
            b.write_all(&string_cache.add_string(&row.texture).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

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
impl Indexable<i32> for Stationery {
    type Table = Self;

    fn get(&self, key: &StationeryKey) -> Option<&StationeryRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &StationeryKey) -> Option<&mut StationeryRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StationeryRow {
    pub id: StationeryKey,
    pub item_id: ItemKey,
    pub texture: String,
    pub flags: i32,
}

impl DbcRow for StationeryRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn stationery() {
        let mut file = File::open("../tbc-dbc/Stationery.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Stationery::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Stationery::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
