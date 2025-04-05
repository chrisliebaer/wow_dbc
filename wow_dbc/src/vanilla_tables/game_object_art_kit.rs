use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;
use super::VanillaTable;

pub type GameObjectArtKitKey = crate::PrimaryKey<u32, GameObjectArtKit>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameObjectArtKit {
    pub rows: Vec<GameObjectArtKitRow>,
}

impl GameObjectArtKit {
    pub const FILENAME: &'static str = "GameObjectArtKit.dbc";
    pub const FIELD_COUNT: usize = 8;
    pub const ROW_SIZE: usize = 32;

}

impl Into<VanillaTable> for GameObjectArtKit {
    fn into(self) -> VanillaTable {
        VanillaTable::GameObjectArtKit(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GameObjectArtKit {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GameObjectArtKitRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GameObjectArtKitRow] { &mut self.rows }

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

            // id: primary_key (GameObjectArtKit) uint32
            let id = GameObjectArtKitKey::new(crate::util::read_u32_le(chunk)?);

            // texture_variation: string_ref[3]
            let texture_variation = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // attach_model: string_ref[4]
            let attach_model = {
                let mut arr = Vec::with_capacity(4);
                for _ in 0..4 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };


            rows.push(GameObjectArtKitRow {
                id,
                texture_variation,
                attach_model,
            });
        }

        Ok(GameObjectArtKit { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GameObjectArtKit) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // texture_variation: string_ref[3]
            for i in &row.texture_variation {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // attach_model: string_ref[4]
            for i in &row.attach_model {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
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
impl Indexable<u32> for GameObjectArtKit {
    type Table = Self;

    fn get(&self, key: &GameObjectArtKitKey) -> Option<&GameObjectArtKitRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GameObjectArtKitKey) -> Option<&mut GameObjectArtKitRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameObjectArtKitRow {
    pub id: GameObjectArtKitKey,
    pub texture_variation: [String; 3],
    pub attach_model: [String; 4],
}

impl DbcRow for GameObjectArtKitRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn game_object_art_kit() {
        let mut file = File::open("../vanilla-dbc/GameObjectArtKit.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GameObjectArtKit::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GameObjectArtKit::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
