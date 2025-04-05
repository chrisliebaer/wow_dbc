use crate::{
    DbcRow, DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::area_table::{
    AreaTable, AreaTableKey,
};
use crate::vanilla_tables::map::{
    Map, MapKey,
};
use std::io::Write;
use super::VanillaTable;

pub type WorldStateUIKey = crate::PrimaryKey<u32, WorldStateUI>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUI {
    pub rows: Vec<WorldStateUIRow>,
}

impl WorldStateUI {
    pub const FILENAME: &'static str = "WorldStateUI.dbc";
    pub const FIELD_COUNT: usize = 39;
    pub const ROW_SIZE: usize = 156;

    pub fn verify(&self, area_table: &AreaTable, map: &Map) -> Result<(), crate::InvalidForeignKeyError<&WorldStateUIRow>> {
        for row in &self.rows {
            if row.map.id != 0 && map.get(&row.map).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldStateUI>(),
                    row,
                    id,
                    row.map.id.into()
                ));
            }

            if row.area_table.id != 0 && area_table.get(&row.area_table).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<WorldStateUI>(),
                    row,
                    id,
                    row.area_table.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for WorldStateUI {
    fn into(self) -> VanillaTable {
        VanillaTable::WorldStateUI(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for WorldStateUI {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[WorldStateUIRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [WorldStateUIRow] { &mut self.rows }

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

            // id: primary_key (WorldStateUI) uint32
            let id = WorldStateUIKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // icon: string_ref
            let icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // state_variable: string_ref_loc
            let state_variable = crate::util::read_localized_string(chunk, &string_block)?;

            // tooltip: string_ref_loc
            let tooltip = crate::util::read_localized_string(chunk, &string_block)?;

            // state: int32
            let state = crate::util::read_i32_le(chunk)?;

            // world_state: uint32
            let world_state = crate::util::read_u32_le(chunk)?;

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // dynamic_icon: string_ref
            let dynamic_icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // dynamic_tooltip: string_ref_loc
            let dynamic_tooltip = crate::util::read_localized_string(chunk, &string_block)?;

            // extended_ui: string_ref
            let extended_ui = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // unknown: uint32[3]
            let unknown = crate::util::read_array_u32::<3>(chunk)?;


            rows.push(WorldStateUIRow {
                id,
                map,
                area_table,
                icon,
                state_variable,
                tooltip,
                state,
                world_state,
                ty,
                dynamic_icon,
                dynamic_tooltip,
                extended_ui,
                unknown,
            });
        }

        Ok(WorldStateUI { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldStateUI) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // icon: string_ref
            b.write_all(&string_cache.add_string(&row.icon).to_le_bytes())?;

            // state_variable: string_ref_loc
            b.write_all(&row.state_variable.string_indices_as_array(&mut string_cache))?;

            // tooltip: string_ref_loc
            b.write_all(&row.tooltip.string_indices_as_array(&mut string_cache))?;

            // state: int32
            b.write_all(&row.state.to_le_bytes())?;

            // world_state: uint32
            b.write_all(&row.world_state.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // dynamic_icon: string_ref
            b.write_all(&string_cache.add_string(&row.dynamic_icon).to_le_bytes())?;

            // dynamic_tooltip: string_ref_loc
            b.write_all(&row.dynamic_tooltip.string_indices_as_array(&mut string_cache))?;

            // extended_ui: string_ref
            b.write_all(&string_cache.add_string(&row.extended_ui).to_le_bytes())?;

            // unknown: uint32[3]
            for i in row.unknown {
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
impl Indexable<u32> for WorldStateUI {
    type Table = Self;

    fn get(&self, key: &WorldStateUIKey) -> Option<&WorldStateUIRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &WorldStateUIKey) -> Option<&mut WorldStateUIRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUIRow {
    pub id: WorldStateUIKey,
    pub map: MapKey,
    pub area_table: AreaTableKey,
    pub icon: String,
    pub state_variable: LocalizedString,
    pub tooltip: LocalizedString,
    pub state: i32,
    pub world_state: u32,
    pub ty: i32,
    pub dynamic_icon: String,
    pub dynamic_tooltip: LocalizedString,
    pub extended_ui: String,
    pub unknown: [u32; 3],
}

impl DbcRow for WorldStateUIRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_state_ui() {
        let mut file = File::open("../vanilla-dbc/WorldStateUI.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldStateUI::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldStateUI::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
