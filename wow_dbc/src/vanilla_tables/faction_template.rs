use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::faction::{
    Faction, FactionKey,
};
use crate::vanilla_tables::faction_group::{
    FactionGroup, FactionGroupKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::PvpFlags;

pub type FactionTemplateKey = crate::PrimaryKey<u32, FactionTemplate>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactionTemplate {
    pub rows: Vec<FactionTemplateRow>,
}

impl FactionTemplate {
    pub const FILENAME: &'static str = "FactionTemplate.dbc";
    pub const FIELD_COUNT: usize = 14;
    pub const ROW_SIZE: usize = 56;

    pub fn verify(&self, faction: &Faction, faction_group: &FactionGroup) -> Result<(), crate::InvalidForeignKeyError<&FactionTemplateRow>> {
        for row in &self.rows {
            if row.faction.id != 0 && faction.get(&row.faction).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<FactionTemplate>(),
                    row,
                    id,
                    row.faction.id.into()
                ));
            }

            if row.faction_group.id != 0 && faction_group.get(&row.faction_group).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<FactionTemplate>(),
                    row,
                    id,
                    row.faction_group.id.into()
                ));
            }

            if row.friend_group.id != 0 && faction_group.get(&row.friend_group).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<FactionTemplate>(),
                    row,
                    id,
                    row.friend_group.id.into()
                ));
            }

            if row.enemy_group.id != 0 && faction_group.get(&row.enemy_group).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<FactionTemplate>(),
                    row,
                    id,
                    row.enemy_group.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for FactionTemplate {
    fn into(self) -> VanillaTable {
        VanillaTable::FactionTemplate(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for FactionTemplate {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[FactionTemplateRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [FactionTemplateRow] { &mut self.rows }

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

            // id: primary_key (FactionTemplate) uint32
            let id = FactionTemplateKey::new(crate::util::read_u32_le(chunk)?);

            // faction: foreign_key (Faction) uint32
            let faction = FactionKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: PvpFlags
            let flags = PvpFlags::new(crate::util::read_u32_le(chunk)? as _);

            // faction_group: foreign_key (FactionGroup) uint32
            let faction_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // friend_group: foreign_key (FactionGroup) uint32
            let friend_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // enemy_group: foreign_key (FactionGroup) uint32
            let enemy_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // enemies: uint32[4]
            let enemies = crate::util::read_array_u32::<4>(chunk)?;

            // friends: uint32[4]
            let friends = crate::util::read_array_u32::<4>(chunk)?;


            rows.push(FactionTemplateRow {
                id,
                faction,
                flags,
                faction_group,
                friend_group,
                enemy_group,
                enemies,
                friends,
            });
        }

        Ok(FactionTemplate { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (FactionTemplate) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction: foreign_key (Faction) uint32
            b.write_all(&(row.faction.id as u32).to_le_bytes())?;

            // flags: PvpFlags
            b.write_all(&(row.flags.as_int() as u32).to_le_bytes())?;

            // faction_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.faction_group.id as u32).to_le_bytes())?;

            // friend_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.friend_group.id as u32).to_le_bytes())?;

            // enemy_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.enemy_group.id as u32).to_le_bytes())?;

            // enemies: uint32[4]
            for i in row.enemies {
                b.write_all(&i.to_le_bytes())?;
            }


            // friends: uint32[4]
            for i in row.friends {
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
impl Indexable<u32> for FactionTemplate {
    type Table = Self;

    fn get(&self, key: &FactionTemplateKey) -> Option<&FactionTemplateRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &FactionTemplateKey) -> Option<&mut FactionTemplateRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactionTemplateRow {
    pub id: FactionTemplateKey,
    pub faction: FactionKey,
    pub flags: PvpFlags,
    pub faction_group: FactionGroupKey,
    pub friend_group: FactionGroupKey,
    pub enemy_group: FactionGroupKey,
    pub enemies: [u32; 4],
    pub friends: [u32; 4],
}

impl DbcRow for FactionTemplateRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn faction_template() {
        let mut file = File::open("../vanilla-dbc/FactionTemplate.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = FactionTemplate::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = FactionTemplate::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
