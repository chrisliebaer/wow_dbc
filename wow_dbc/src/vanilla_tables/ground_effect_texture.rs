use crate::{
    DbcRow, DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::terrain_type::{
    TerrainType, TerrainTypeKey,
};
use std::io::Write;
use super::VanillaTable;

pub type GroundEffectTextureKey = crate::PrimaryKey<u32, GroundEffectTexture>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroundEffectTexture {
    pub rows: Vec<GroundEffectTextureRow>,
}

impl GroundEffectTexture {
    pub const FILENAME: &'static str = "GroundEffectTexture.dbc";
    pub const FIELD_COUNT: usize = 7;
    pub const ROW_SIZE: usize = 28;

    pub fn verify(&self, terrain_type: &TerrainType) -> Result<(), crate::InvalidForeignKeyError<&GroundEffectTextureRow>> {
        for row in &self.rows {
            if row.terrain_type.id != 0 && terrain_type.get(&row.terrain_type).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<GroundEffectTexture>(),
                    row,
                    id,
                    row.terrain_type.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<VanillaTable> for GroundEffectTexture {
    fn into(self) -> VanillaTable {
        VanillaTable::GroundEffectTexture(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GroundEffectTexture {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GroundEffectTextureRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GroundEffectTextureRow] { &mut self.rows }

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

            // id: primary_key (GroundEffectTexture) uint32
            let id = GroundEffectTextureKey::new(crate::util::read_u32_le(chunk)?);

            // doodad: uint32[4]
            let doodad = crate::util::read_array_u32::<4>(chunk)?;

            // density: int32
            let density = crate::util::read_i32_le(chunk)?;

            // terrain_type: foreign_key (TerrainType) uint32
            let terrain_type = TerrainTypeKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(GroundEffectTextureRow {
                id,
                doodad,
                density,
                terrain_type,
            });
        }

        Ok(GroundEffectTexture { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GroundEffectTexture) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // doodad: uint32[4]
            for i in row.doodad {
                b.write_all(&i.to_le_bytes())?;
            }


            // density: int32
            b.write_all(&row.density.to_le_bytes())?;

            // terrain_type: foreign_key (TerrainType) uint32
            b.write_all(&(row.terrain_type.id as u32).to_le_bytes())?;

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
impl Indexable<u32> for GroundEffectTexture {
    type Table = Self;

    fn get(&self, key: &GroundEffectTextureKey) -> Option<&GroundEffectTextureRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GroundEffectTextureKey) -> Option<&mut GroundEffectTextureRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroundEffectTextureRow {
    pub id: GroundEffectTextureKey,
    pub doodad: [u32; 4],
    pub density: i32,
    pub terrain_type: TerrainTypeKey,
}

impl DbcRow for GroundEffectTextureRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn ground_effect_texture() {
        let mut file = File::open("../vanilla-dbc/GroundEffectTexture.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GroundEffectTexture::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GroundEffectTexture::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
