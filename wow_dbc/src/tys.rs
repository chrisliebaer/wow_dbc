use crate::util::StringCache;

/// DBCs from the English version of the game will only have English version strings, while other localizations will have other languages.
///
/// You are most likely interested in, [`LocalizedString::en_gb`], the English version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub struct ExtendedLocalizedString {
    /// English, Great Britain
    pub en_gb: String,
    /// Korean, Korea
    pub ko_kr: String,
    /// French, France
    pub fr_fr: String,
    /// German, Germany
    pub de_de: String,
    /// English, China
    pub en_cn: String,
    /// English, Taiwan
    pub en_tw: String,
    /// Spanish, Spain
    pub es_es: String,
    /// Spanish, Mexico
    pub es_mx: String,
    /// Russian, Russia
    pub ru_ru: String,
    /// Japanese, Japan
    pub ja_jp: String,
    /// Portuguese, Portugal
    ///
    /// Also works as Portuguese, Brazil
    pub pt_pt: String,
    /// Italian, Italy
    pub it_it: String,
    /// Possibly unused.
    pub unknown_12: String,
    /// Possibly unused.
    pub unknown_13: String,
    /// Possibly unused.
    pub unknown_14: String,
    /// Possibly unused.
    pub unknown_15: String,

    /// Unknown flags.
    pub flags: u32,
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
impl ExtendedLocalizedString {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        en_gb: String,
        ko_kr: String,
        fr_fr: String,
        de_de: String,
        en_cn: String,
        en_tw: String,
        es_es: String,
        es_mx: String,
        ru_ru: String,
        ja_jp: String,
        pt_pt: String,
        it_it: String,
        unknown_12: String,
        unknown_13: String,
        unknown_14: String,
        unknown_15: String,
        flags: u32,
    ) -> Self {
        Self {
            en_gb,
            ko_kr,
            fr_fr,
            de_de,
            en_cn,
            en_tw,
            es_es,
            es_mx,
            ru_ru,
            ja_jp,
            pt_pt,
            it_it,
            unknown_12,
            unknown_13,
            unknown_14,
            unknown_15,
            flags,
        }
    }
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
impl WritableString for ExtendedLocalizedString {
    fn strings(&self) -> Box<[&str]> {
        Box::new([
            &self.en_gb,
            &self.ko_kr,
            &self.fr_fr,
            &self.de_de,
            &self.en_cn,
            &self.en_tw,
            &self.es_es,
            &self.es_mx,
            &self.ru_ru,
            &self.ja_jp,
            &self.pt_pt,
            &self.it_it,
            &self.unknown_12,
            &self.unknown_13,
            &self.unknown_14,
            &self.unknown_15,
        ])
    }

    fn flags(&self) -> u32 {
        self.flags
    }
}

/// DBCs from the English version of the game will only have English version strings, while other localizations will have other languages.
///
/// You are most likely interested in, [`LocalizedString::en_gb`], the English version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(feature = "vanilla")]
pub struct LocalizedString {
    /// English, Great Britain
    pub en_gb: String,
    /// Korean, Korea
    pub ko_kr: String,
    /// French, France
    pub fr_fr: String,
    /// German, Germany
    pub de_de: String,
    /// English, China
    pub en_cn: String,
    /// English, Taiwan
    pub en_tw: String,
    /// Spanish, Spain
    pub es_es: String,
    /// Spanish, Mexico
    pub es_mx: String,
    /// Unknown flags.
    pub flags: u32,
}

#[cfg(feature = "vanilla")]
impl LocalizedString {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        en_gb: String,
        ko_kr: String,
        fr_fr: String,
        de_de: String,
        en_cn: String,
        en_tw: String,
        es_es: String,
        es_mx: String,
        flags: u32,
    ) -> Self {
        Self {
            en_gb,
            ko_kr,
            fr_fr,
            de_de,
            en_cn,
            en_tw,
            es_es,
            es_mx,
            flags,
        }
    }
}

#[cfg(feature = "vanilla")]
impl WritableString for LocalizedString {
    fn strings(&self) -> Box<[&str]> {
        Box::new([
            &self.en_gb,
            &self.ko_kr,
            &self.fr_fr,
            &self.de_de,
            &self.en_cn,
            &self.en_tw,
            &self.es_es,
            &self.es_mx,
        ])
    }

    fn flags(&self) -> u32 {
        self.flags
    }
}

pub(crate) trait WritableString {
    /// Returns a slice of strings, represented by the implementing type.
    fn strings(&self) -> Box<[&str]>;

    /// Returns the flags for the strings.
    fn flags(&self) -> u32;

    fn string_indices_as_array(&self, string_cache: &mut StringCache) -> Box<[u8]> {
        // S u32s + 1 u32 for flags
        let mut arr = Vec::new();

        for s in self.strings().iter() {
            // some strings can be empty, which is handled gracefully by the string cache by providing the same offset for all empty strings
            arr.extend_from_slice(&string_cache.add_string(s).to_le_bytes());
        }
        arr.extend_from_slice(&self.flags().to_le_bytes());
        arr.into_boxed_slice()
    }
}


/// Primary key for a `Table` with a primary key of type `Integer`.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrimaryKey<Integer, Table>
where
    Table: crate::Indexable<Integer> + crate::DbcTable,
{
    /// The primary key value.
    pub id: Integer,

    /// Phantom data to ensure that the primary key is only used with the correct table type.
    _marker: std::marker::PhantomData<Table>,
}

// ***********************
// Start of manual derives
// ***********************
// This section is required since basically all derives are not implemented for `PhantomData`.

// impl `PartialEq`
impl<I, T> PartialEq for PrimaryKey<I, T>
where
    I: PartialEq,
    T: crate::Indexable<I> + crate::DbcTable,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// impl `Eq`
impl<I, T> Eq for PrimaryKey<I, T>
where
    I: Eq,
    T: crate::Indexable<I> + crate::DbcTable,
{}

// impl `PartialOrd`
impl<I, T> PartialOrd for PrimaryKey<I, T>
where
    I: PartialOrd,
    T: crate::Indexable<I> + crate::DbcTable,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

// impl `Ord`
impl<I, T> Ord for PrimaryKey<I, T>
where
    I: Ord,
    T: crate::Indexable<I> + crate::DbcTable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

// impl `Hash`
impl<I, T> std::hash::Hash for PrimaryKey<I, T>
where
    I: std::hash::Hash,
    T: crate::Indexable<I> + crate::DbcTable,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// impl `Clone`
impl<I, T> Clone for PrimaryKey<I, T>
where
    I: Clone,
    T: crate::Indexable<I> + crate::DbcTable,
{
    fn clone(&self) -> Self {
        Self::new(self.id.clone())
    }
}

// impl `Copy`
impl<I, T> Copy for PrimaryKey<I, T>
where
    I: Copy,
    T: crate::Indexable<I> + crate::DbcTable,
{}

// *********************
// End of manual derives
// *********************


impl<I, T> PrimaryKey<I, T>
where
    T: crate::Indexable<I> + crate::DbcTable,
{
    /// Creates a new primary key from the given value.
    pub const fn new(id: I) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    /// Casts the primary key to a key for a different table type.
    pub fn cast_table_key<NewTable>(self) -> PrimaryKey<I, NewTable>
    where
        NewTable: crate::Indexable<I> + crate::DbcTable,
    {
        PrimaryKey::new(self.id)
    }
}


/// Macro for generating `From` implementations for `PrimaryKey` types.
macro_rules! impl_from_for_primary_key {
    ($($int_ty:ty),+) => {
        $(
            impl<T, V> From<V> for PrimaryKey<$int_ty, T>
            where
                V: Into<$int_ty>,
                T: crate::Indexable<$int_ty> + crate::DbcTable,
            {
                fn from(v: V) -> Self {
                    Self::new(v.into())
                }
            }
        )*
    };
}

// Implement `From` for all integer types that can be used as primary keys.
// We can't use a generic impl for all Into<T> because this would conflict with the
// generic `impl From<T> for T;` in rust core. Solutions welcome.
impl_from_for_primary_key!(i32);
impl_from_for_primary_key!(u32);
