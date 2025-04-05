use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::chr_races::{
    ChrRaces, ChrRacesKey,
};
use std::io::Write;
use super::VanillaTable;
use wow_world_base::vanilla::Gender;

pub type CreatureDisplayInfoExtraKey = crate::PrimaryKey<u32, CreatureDisplayInfoExtra>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfoExtra {
    pub rows: Vec<CreatureDisplayInfoExtraRow>,
}

impl CreatureDisplayInfoExtra {
    pub const FILENAME: &'static str = "CreatureDisplayInfoExtra.dbc";
    pub const FIELD_COUNT: usize = 19;
    pub const ROW_SIZE: usize = 76;

    pub fn verify(&self, chr_races: &ChrRaces) -> Result<(), crate::InvalidForeignKeyError<&CreatureDisplayInfoExtraRow>> {
        for row in &self.rows {
            if row.display_race.id != 0 && chr_races.get(&row.display_race).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<CreatureDisplayInfoExtra>(),
                    row,
                    id,
                    row.display_race.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for CreatureDisplayInfoExtra {
    fn into(self) -> VanillaTable {
        VanillaTable::CreatureDisplayInfoExtra(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for CreatureDisplayInfoExtra {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[CreatureDisplayInfoExtraRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [CreatureDisplayInfoExtraRow] { &mut self.rows }

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

            // id: primary_key (CreatureDisplayInfoExtra) uint32
            let id = CreatureDisplayInfoExtraKey::new(crate::util::read_u32_le(chunk)?);

            // display_race: foreign_key (ChrRaces) uint32
            let display_race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i32_le(chunk)?.try_into()?;

            // skin: int32
            let skin = crate::util::read_i32_le(chunk)?;

            // face: int32
            let face = crate::util::read_i32_le(chunk)?;

            // hair_style: int32
            let hair_style = crate::util::read_i32_le(chunk)?;

            // hair_colour: int32
            let hair_colour = crate::util::read_i32_le(chunk)?;

            // facial_hair: int32
            let facial_hair = crate::util::read_i32_le(chunk)?;

            // npc_item_display: uint32[9]
            let npc_item_display = crate::util::read_array_u32::<9>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // bake_name: string_ref
            let bake_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(CreatureDisplayInfoExtraRow {
                id,
                display_race,
                gender,
                skin,
                face,
                hair_style,
                hair_colour,
                facial_hair,
                npc_item_display,
                flags,
                bake_name,
            });
        }

        Ok(CreatureDisplayInfoExtra { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureDisplayInfoExtra) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // display_race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.display_race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // skin: int32
            b.write_all(&row.skin.to_le_bytes())?;

            // face: int32
            b.write_all(&row.face.to_le_bytes())?;

            // hair_style: int32
            b.write_all(&row.hair_style.to_le_bytes())?;

            // hair_colour: int32
            b.write_all(&row.hair_colour.to_le_bytes())?;

            // facial_hair: int32
            b.write_all(&row.facial_hair.to_le_bytes())?;

            // npc_item_display: uint32[9]
            for i in row.npc_item_display {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // bake_name: string_ref
            b.write_all(&string_cache.add_string(&row.bake_name).to_le_bytes())?;

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
impl Indexable<u32> for CreatureDisplayInfoExtra {
    type Table = Self;

    fn get(&self, key: &CreatureDisplayInfoExtraKey) -> Option<&CreatureDisplayInfoExtraRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &CreatureDisplayInfoExtraKey) -> Option<&mut CreatureDisplayInfoExtraRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfoExtraRow {
    pub id: CreatureDisplayInfoExtraKey,
    pub display_race: ChrRacesKey,
    pub gender: Gender,
    pub skin: i32,
    pub face: i32,
    pub hair_style: i32,
    pub hair_colour: i32,
    pub facial_hair: i32,
    pub npc_item_display: [u32; 9],
    pub flags: i32,
    pub bake_name: String,
}

impl DbcRow for CreatureDisplayInfoExtraRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_display_info_extra() {
        let mut file = File::open("../vanilla-dbc/CreatureDisplayInfoExtra.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureDisplayInfoExtra::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureDisplayInfoExtra::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
