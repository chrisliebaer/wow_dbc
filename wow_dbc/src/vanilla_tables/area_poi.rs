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
use crate::vanilla_tables::faction::{
    Faction, FactionKey,
};
use crate::vanilla_tables::map::{
    Map, MapKey,
};
use crate::vanilla_tables::world_state_ui::{
    WorldStateUI, WorldStateUIKey,
};
use std::io::Write;
use super::VanillaTable;

pub type AreaPOIKey = crate::PrimaryKey<u32, AreaPOI>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaPOI {
    pub rows: Vec<AreaPOIRow>,
}

impl AreaPOI {
    pub const FILENAME: &'static str = "AreaPOI.dbc";
    pub const FIELD_COUNT: usize = 29;
    pub const ROW_SIZE: usize = 116;

    pub fn verify(&self, area_table: &AreaTable, faction: &Faction, map: &Map, world_state_ui: &WorldStateUI) -> Result<(), crate::InvalidForeignKeyError<&AreaPOIRow>> {
        for row in &self.rows {
            if row.faction.id != 0 && faction.get(&row.faction).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.faction.id.into()
                ));
            }

            if row.map.id != 0 && map.get(&row.map).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.map.id.into()
                ));
            }

            if row.area_table.id != 0 && area_table.get(&row.area_table).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.area_table.id.into()
                ));
            }

            if row.world_state.id != 0 && world_state_ui.get(&row.world_state).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.world_state.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for AreaPOI {
    fn into(self) -> VanillaTable {
        VanillaTable::AreaPOI(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for AreaPOI {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[AreaPOIRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [AreaPOIRow] { &mut self.rows }

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

            // id: primary_key (AreaPOI) uint32
            let id = AreaPOIKey::new(crate::util::read_u32_le(chunk)?);

            // importance: int32
            let importance = crate::util::read_i32_le(chunk)?;

            // icon: int32
            let icon = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (Faction) uint32
            let faction = FactionKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;

            // world_state: foreign_key (WorldStateUI) uint32
            let world_state = WorldStateUIKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(AreaPOIRow {
                id,
                importance,
                icon,
                faction,
                location_x,
                location_y,
                location_z,
                map,
                flags,
                area_table,
                name,
                description,
                world_state,
            });
        }

        Ok(AreaPOI { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AreaPOI) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // importance: int32
            b.write_all(&row.importance.to_le_bytes())?;

            // icon: int32
            b.write_all(&row.icon.to_le_bytes())?;

            // faction: foreign_key (Faction) uint32
            b.write_all(&(row.faction.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_cache))?;

            // world_state: foreign_key (WorldStateUI) uint32
            b.write_all(&(row.world_state.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for AreaPOI {
    type Table = Self;

    fn get(&self, key: &AreaPOIKey) -> Option<&AreaPOIRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &AreaPOIKey) -> Option<&mut AreaPOIRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaPOIRow {
    pub id: AreaPOIKey,
    pub importance: i32,
    pub icon: i32,
    pub faction: FactionKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub map: MapKey,
    pub flags: i32,
    pub area_table: AreaTableKey,
    pub name: LocalizedString,
    pub description: LocalizedString,
    pub world_state: WorldStateUIKey,
}

impl DbcRow for AreaPOIRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn area_poi() {
        let mut file = File::open("../vanilla-dbc/AreaPOI.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaPOI::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaPOI::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
