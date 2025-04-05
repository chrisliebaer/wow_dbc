use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::item_group_sounds::{
    ItemGroupSounds, ItemGroupSoundsKey,
};
use crate::vanilla_tables::item_visuals::{
    ItemVisuals, ItemVisualsKey,
};
use crate::vanilla_tables::spell_visual::{
    SpellVisual, SpellVisualKey,
};
use std::io::Write;
use super::VanillaTable;

pub type ItemDisplayInfoKey = crate::PrimaryKey<u32, ItemDisplayInfo>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDisplayInfo {
    pub rows: Vec<ItemDisplayInfoRow>,
}

impl ItemDisplayInfo {
    pub const FILENAME: &'static str = "ItemDisplayInfo.dbc";
    pub const FIELD_COUNT: usize = 23;
    pub const ROW_SIZE: usize = 92;

    pub fn verify(&self, item_group_sounds: &ItemGroupSounds, item_visuals: &ItemVisuals, spell_visual: &SpellVisual) -> Result<(), crate::InvalidForeignKeyError<&ItemDisplayInfoRow>> {
        for row in &self.rows {
            if row.spell_visual.id != 0 && spell_visual.get(&row.spell_visual).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ItemDisplayInfo>(),
                    row,
                    id,
                    row.spell_visual.id.into()
                ));
            }

            if row.group_sound_index.id != 0 && item_group_sounds.get(&row.group_sound_index).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ItemDisplayInfo>(),
                    row,
                    id,
                    row.group_sound_index.id.into()
                ));
            }

            if row.item_visual.id != 0 && item_visuals.get(&row.item_visual).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<ItemDisplayInfo>(),
                    row,
                    id,
                    row.item_visual.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for ItemDisplayInfo {
    fn into(self) -> VanillaTable {
        VanillaTable::ItemDisplayInfo(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for ItemDisplayInfo {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[ItemDisplayInfoRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [ItemDisplayInfoRow] { &mut self.rows }

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

            // id: primary_key (ItemDisplayInfo) uint32
            let id = ItemDisplayInfoKey::new(crate::util::read_u32_le(chunk)?);

            // model_name: string_ref[2]
            let model_name = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // model_texture: string_ref[2]
            let model_texture = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // inventory_icon: string_ref[2]
            let inventory_icon = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // geoset_group: int32[3]
            let geoset_group = crate::util::read_array_i32::<3>(chunk)?;

            // spell_visual: foreign_key (SpellVisual) uint32
            let spell_visual = SpellVisualKey::new(crate::util::read_u32_le(chunk)?.into());

            // group_sound_index: foreign_key (ItemGroupSounds) uint32
            let group_sound_index = ItemGroupSoundsKey::new(crate::util::read_u32_le(chunk)?.into());

            // helmet_geoset_vis: uint32[2]
            let helmet_geoset_vis = crate::util::read_array_u32::<2>(chunk)?;

            // textures: string_ref[8]
            let textures = {
                let mut arr = Vec::with_capacity(8);
                for _ in 0..8 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // item_visual: foreign_key (ItemVisuals) uint32
            let item_visual = ItemVisualsKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(ItemDisplayInfoRow {
                id,
                model_name,
                model_texture,
                inventory_icon,
                geoset_group,
                spell_visual,
                group_sound_index,
                helmet_geoset_vis,
                textures,
                item_visual,
            });
        }

        Ok(ItemDisplayInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ItemDisplayInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_name: string_ref[2]
            for i in &row.model_name {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // model_texture: string_ref[2]
            for i in &row.model_texture {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // inventory_icon: string_ref[2]
            for i in &row.inventory_icon {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // geoset_group: int32[3]
            for i in row.geoset_group {
                b.write_all(&i.to_le_bytes())?;
            }


            // spell_visual: foreign_key (SpellVisual) uint32
            b.write_all(&(row.spell_visual.id as u32).to_le_bytes())?;

            // group_sound_index: foreign_key (ItemGroupSounds) uint32
            b.write_all(&(row.group_sound_index.id as u32).to_le_bytes())?;

            // helmet_geoset_vis: uint32[2]
            for i in row.helmet_geoset_vis {
                b.write_all(&i.to_le_bytes())?;
            }


            // textures: string_ref[8]
            for i in &row.textures {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // item_visual: foreign_key (ItemVisuals) uint32
            b.write_all(&(row.item_visual.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for ItemDisplayInfo {
    type Table = Self;

    fn get(&self, key: &ItemDisplayInfoKey) -> Option<&ItemDisplayInfoRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &ItemDisplayInfoKey) -> Option<&mut ItemDisplayInfoRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDisplayInfoRow {
    pub id: ItemDisplayInfoKey,
    pub model_name: [String; 2],
    pub model_texture: [String; 2],
    pub inventory_icon: [String; 2],
    pub geoset_group: [i32; 3],
    pub spell_visual: SpellVisualKey,
    pub group_sound_index: ItemGroupSoundsKey,
    pub helmet_geoset_vis: [u32; 2],
    pub textures: [String; 8],
    pub item_visual: ItemVisualsKey,
}

impl DbcRow for ItemDisplayInfoRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_display_info() {
        let mut file = File::open("../vanilla-dbc/ItemDisplayInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemDisplayInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemDisplayInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
