use crate::{
    DbcRow, DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::gm_survey_questions::{
    GMSurveyQuestions, GMSurveyQuestionsKey,
};
use std::io::Write;
use super::WrathTable;

pub type GMSurveyAnswersKey = crate::PrimaryKey<i32, GMSurveyAnswers>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GMSurveyAnswers {
    pub rows: Vec<GMSurveyAnswersRow>,
}

impl GMSurveyAnswers {
    pub const FILENAME: &'static str = "GMSurveyAnswers.dbc";
    pub const FIELD_COUNT: usize = 20;
    pub const ROW_SIZE: usize = 80;

    pub fn verify(&self, gm_survey_questions: &GMSurveyQuestions) -> Result<(), crate::InvalidForeignKeyError<&GMSurveyAnswersRow>> {
        for row in &self.rows {
            if row.g_m_survey_question_id.id != 0 && gm_survey_questions.get(&row.g_m_survey_question_id).is_none() {
                let id = Some(row.id.id.into());
                return Err(crate::InvalidForeignKeyError::new(
                    std::any::type_name::<GMSurveyAnswers>(),
                    row,
                    id,
                    row.g_m_survey_question_id.id.into()
                ));
            }

        }

        Ok(())
    }

}

impl Into<WrathTable> for GMSurveyAnswers {
    fn into(self) -> WrathTable {
        WrathTable::GMSurveyAnswers(self)
    }
}

#[allow(refining_impl_trait)]
impl DbcTable for GMSurveyAnswers {
    fn filename(&self) -> &'static str { Self::FILENAME }
    fn field_count(&self) -> usize { Self::FIELD_COUNT }
    fn row_size(&self) -> usize { Self::ROW_SIZE }

    fn rows(&self) -> &[GMSurveyAnswersRow] { &self.rows }
    fn rows_mut(&mut self) -> &mut [GMSurveyAnswersRow] { &mut self.rows }

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

            // id: primary_key (GMSurveyAnswers) int32
            let id = GMSurveyAnswersKey::new(crate::util::read_i32_le(chunk)?);

            // sort_index: int32
            let sort_index = crate::util::read_i32_le(chunk)?;

            // g_m_survey_question_id: foreign_key (GMSurveyQuestions) int32
            let g_m_survey_question_id = GMSurveyQuestionsKey::new(crate::util::read_i32_le(chunk)?.into());

            // answer_lang: string_ref_loc (Extended)
            let answer_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(GMSurveyAnswersRow {
                id,
                sort_index,
                g_m_survey_question_id,
                answer_lang,
            });
        }

        Ok(GMSurveyAnswers { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (GMSurveyAnswers) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sort_index: int32
            b.write_all(&row.sort_index.to_le_bytes())?;

            // g_m_survey_question_id: foreign_key (GMSurveyQuestions) int32
            b.write_all(&(row.g_m_survey_question_id.id as i32).to_le_bytes())?;

            // answer_lang: string_ref_loc (Extended)
            b.write_all(&row.answer_lang.string_indices_as_array(&mut string_cache))?;

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
impl Indexable<i32> for GMSurveyAnswers {
    type Table = Self;

    fn get(&self, key: &GMSurveyAnswersKey) -> Option<&GMSurveyAnswersRow> {
        self.rows.iter().find(|a| &a.id == key)
    }

    fn get_mut(&mut self, key: &GMSurveyAnswersKey) -> Option<&mut GMSurveyAnswersRow> {
        self.rows.iter_mut().find(|a| &a.id == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GMSurveyAnswersRow {
    pub id: GMSurveyAnswersKey,
    pub sort_index: i32,
    pub g_m_survey_question_id: GMSurveyQuestionsKey,
    pub answer_lang: ExtendedLocalizedString,
}

impl DbcRow for GMSurveyAnswersRow {
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn gm_survey_answers() {
        let mut file = File::open("../wrath-dbc/GMSurveyAnswers.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = GMSurveyAnswers::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GMSurveyAnswers::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
