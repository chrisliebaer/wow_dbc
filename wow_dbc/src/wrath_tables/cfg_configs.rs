use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::WrathTable;

pub type Cfg_ConfigsKey = crate::PrimaryKey<i32, Cfg_Configs>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cfg_Configs {
    pub rows: Vec<Cfg_ConfigsRow>,
}

impl Cfg_Configs {
    pub const FILENAME: &'static str = "Cfg_Configs.dbc";
    pub const FIELD_COUNT: usize = 4;
    pub const ROW_SIZE: usize = 16;

}

impl Into<WrathTable> for Cfg_Configs {
    fn into(self) -> WrathTable {
        WrathTable::Cfg_Configs(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for Cfg_Configs {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[Cfg_ConfigsRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Cfg_ConfigsRow] { &mut self.rows }

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

            // id: primary_key (Cfg_Configs) int32
            let id = Cfg_ConfigsKey::new(crate::util::read_i32_le(chunk)?);

            // realm_type: int32
            let realm_type = crate::util::read_i32_le(chunk)?;

            // player_killing_allowed: int32
            let player_killing_allowed = crate::util::read_i32_le(chunk)?;

            // roleplaying: int32
            let roleplaying = crate::util::read_i32_le(chunk)?;


            rows.push(Cfg_ConfigsRow {
                id,
                realm_type,
                player_killing_allowed,
                roleplaying,
            });
        }

        Ok(Cfg_Configs { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Cfg_Configs) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // realm_type: int32
            b.write_all(&row.realm_type.to_le_bytes())?;

            // player_killing_allowed: int32
            b.write_all(&row.player_killing_allowed.to_le_bytes())?;

            // roleplaying: int32
            b.write_all(&row.roleplaying.to_le_bytes())?;

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
impl Indexable<i32> for Cfg_Configs {
    type Table = Self;

    fn get(&self, key: &Cfg_ConfigsKey) -> Option<&Cfg_ConfigsRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &Cfg_ConfigsKey) -> Option<&mut Cfg_ConfigsRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cfg_ConfigsRow {
    pub id: Cfg_ConfigsKey,
    pub realm_type: i32,
    pub player_killing_allowed: i32,
    pub roleplaying: i32,
}

impl DbcRow for Cfg_ConfigsRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn cfg_configs() {
        let mut file = File::open("../wrath-dbc/Cfg_Configs.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Cfg_Configs::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Cfg_Configs::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
