use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::area_table::{
    AreaTable, AreaTableKey,
};
use crate::wrath_tables::faction_template::{
    FactionTemplate, FactionTemplateKey,
};
use crate::wrath_tables::map::{
    Map, MapKey,
};
use crate::wrath_tables::world_state_ui::{
    WorldStateUI, WorldStateUIKey,
};
use std::io::Write;
use super::WrathTable;

pub type AreaPOIKey = crate::PrimaryKey<i32, AreaPOI>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaPOI {
    pub rows: Vec<AreaPOIRow>,
}

impl AreaPOI {
    pub const FILENAME: &'static str = "AreaPOI.dbc";
    pub const FIELD_COUNT: usize = 54;
    pub const ROW_SIZE: usize = 216;

    pub fn verify(&self, area_table: &AreaTable, faction_template: &FactionTemplate, map: &Map, world_state_ui: &WorldStateUI) -> Result<(), crate::InvalidForeignKeyError<&AreaPOIRow>> {
        for row in &self.rows {
            if row.faction_id.id != 0 && faction_template.get(&row.faction_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.faction_id.id.into()
                ));
            }

            if row.continent_id.id != 0 && map.get(&row.continent_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.continent_id.id.into()
                ));
            }

            if row.area_id.id != 0 && area_table.get(&row.area_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.area_id.id.into()
                ));
            }

            if row.world_state_id.id != 0 && world_state_ui.get(&row.world_state_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<AreaPOI>(),
                    row,
                    id,
                    row.world_state_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for AreaPOI {
    fn into(self) -> WrathTable {
        WrathTable::AreaPOI(self)
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

            // id: primary_key (AreaPOI) int32
            let id = AreaPOIKey::new(crate::util::read_i32_le(chunk)?);

            // importance: int32
            let importance = crate::util::read_i32_le(chunk)?;

            // icon: int32[9]
            let icon = crate::util::read_array_i32::<9>(chunk)?;

            // faction_id: foreign_key (FactionTemplate) int32
            let faction_id = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?.into());

            // pos: float[3]
            let pos = crate::util::read_array_f32::<3>(chunk)?;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // world_state_id: foreign_key (WorldStateUI) int32
            let world_state_id = WorldStateUIKey::new(crate::util::read_i32_le(chunk)?.into());

            // world_map_link: int32
            let world_map_link = crate::util::read_i32_le(chunk)?;


            rows.push(AreaPOIRow {
                id,
                importance,
                icon,
                faction_id,
                pos,
                continent_id,
                flags,
                area_id,
                name_lang,
                description_lang,
                world_state_id,
                world_map_link,
            });
        }

        Ok(AreaPOI { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AreaPOI) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // importance: int32
            b.write_all(&row.importance.to_le_bytes())?;

            // icon: int32[9]
            for i in row.icon {
                b.write_all(&i.to_le_bytes())?;
            }


            // faction_id: foreign_key (FactionTemplate) int32
            b.write_all(&(row.faction_id.id as i32).to_le_bytes())?;

            // pos: float[3]
            for i in row.pos {
                b.write_all(&i.to_le_bytes())?;
            }


            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

            // world_state_id: foreign_key (WorldStateUI) int32
            b.write_all(&(row.world_state_id.id as i32).to_le_bytes())?;

            // world_map_link: int32
            b.write_all(&row.world_map_link.to_le_bytes())?;

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
impl Indexable<i32> for AreaPOI {
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
    pub icon: [i32; 9],
    pub faction_id: FactionTemplateKey,
    pub pos: [f32; 3],
    pub continent_id: MapKey,
    pub flags: i32,
    pub area_id: AreaTableKey,
    pub name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub world_state_id: WorldStateUIKey,
    pub world_map_link: i32,
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
        let mut file = File::open("../wrath-dbc/AreaPOI.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaPOI::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaPOI::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
