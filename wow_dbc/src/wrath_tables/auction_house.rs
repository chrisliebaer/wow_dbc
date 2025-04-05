use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::faction::{
    Faction, FactionKey,
};
use std::io::Write;
use super::WrathTable;

pub type AuctionHouseKey = crate::PrimaryKey<i32, AuctionHouse>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuctionHouse {
    pub rows: Vec<AuctionHouseRow>,
}

impl AuctionHouse {
    pub const FILENAME: &'static str = "AuctionHouse.dbc";
    pub const FIELD_COUNT: usize = 21;
    pub const ROW_SIZE: usize = 84;

    pub fn verify(&self, faction: &Faction) -> Result<(), crate::InvalidForeignKeyError<&AuctionHouseRow>> {
        for row in &self.rows {
            if row.faction_id.id != 0 && faction.get(&row.faction_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AuctionHouse>(),
                    row,
                    id,
                    row.faction_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for AuctionHouse {
    fn into(self) -> WrathTable {
        WrathTable::AuctionHouse(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AuctionHouse {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AuctionHouseRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AuctionHouseRow] { &mut self.rows }

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

            // id: primary_key (AuctionHouse) int32
            let id = AuctionHouseKey::new(crate::util::read_i32_le(chunk)?);

            // faction_id: foreign_key (Faction) int32
            let faction_id = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // deposit_rate: int32
            let deposit_rate = crate::util::read_i32_le(chunk)?;

            // consignment_rate: int32
            let consignment_rate = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(AuctionHouseRow {
                id,
                faction_id,
                deposit_rate,
                consignment_rate,
                name_lang,
            });
        }

        Ok(AuctionHouse { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AuctionHouse) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction_id: foreign_key (Faction) int32
            b.write_all(&(row.faction_id.id as i32).to_le_bytes())?;

            // deposit_rate: int32
            b.write_all(&row.deposit_rate.to_le_bytes())?;

            // consignment_rate: int32
            b.write_all(&row.consignment_rate.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<i32> for AuctionHouse {
    type Table = Self;

    fn get(&self, key: &AuctionHouseKey) -> Option<&AuctionHouseRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AuctionHouseKey) -> Option<&mut AuctionHouseRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuctionHouseRow {
    pub id: AuctionHouseKey,
    pub faction_id: FactionKey,
    pub deposit_rate: i32,
    pub consignment_rate: i32,
    pub name_lang: ExtendedLocalizedString,
}

impl DbcRow for AuctionHouseRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn auction_house() {
        let mut file = File::open("../wrath-dbc/AuctionHouse.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AuctionHouse::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AuctionHouse::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
