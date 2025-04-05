use std::error::Error;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use wow_world_base::EnumError;

/// Main error enum. Returned from [`crate::DbcTable::read`].
#[derive(Debug)]
pub enum DbcError {
    /// IO errors.
    Io(std::io::Error),
    /// Errors from invalid enum values.
    InvalidEnum(EnumError),
    /// Errors from converting bytes to strings.
    String(FromUtf8Error),
    /// Errors related to headers.
    InvalidHeader(InvalidHeaderError),
    /// Errors from dynamic table selection.
    InvalidTableName(String),
}


impl Display for DbcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbcError::Io(i) => i.fmt(f),
            DbcError::InvalidEnum(i) => i.fmt(f),
            DbcError::String(i) => i.fmt(f),
            DbcError::InvalidHeader(i) => i.fmt(f),
            DbcError::InvalidTableName(i) => {
                write!(f, "invalid table name: '{}'", i)
            }
        }
    }
}

impl Error for DbcError {}

impl From<std::io::Error> for DbcError {
    fn from(i: std::io::Error) -> Self {
        Self::Io(i)
    }
}

impl From<FromUtf8Error> for DbcError {
    fn from(e: FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<InvalidHeaderError> for DbcError {
    fn from(e: InvalidHeaderError) -> Self {
        Self::InvalidHeader(e)
    }
}

/// Errors from reading the header of the DBC file.
#[derive(Debug)]
pub enum InvalidHeaderError {
    /// The magic value was not `0x43424457`, but was instead [`InvalidHeaderError::MagicValue::actual`].
    MagicValue {
        /// Value gotten instead of magic header.
        actual: u32,
    },
    /// The reported `record_size` did not match the precomputed.
    RecordSize {
        /// Expected value.
        expected: u32,
        /// Actual value read.
        actual: u32,
    },
    /// The reported amount of fields did not match the precomputed.
    FieldCount {
        /// Expected value.
        expected: u32,
        /// Actual value read.
        actual: u32,
    },
}

impl Display for InvalidHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidHeaderError::RecordSize { expected, actual } => {
                write!(
                    f,
                    "invalid record size. Expected '{}', got '{}'",
                    expected, actual
                )
            }
            InvalidHeaderError::FieldCount { expected, actual } => write!(
                f,
                "invalid field count. Expected '{}', got '{}'",
                expected, actual
            ),
            InvalidHeaderError::MagicValue { actual } => {
                write!(f, "invalid header magic: '{}'", actual)
            }
        }
    }
}

impl Error for InvalidHeaderError {}

impl From<EnumError> for DbcError {
    fn from(i: EnumError) -> Self {
        Self::InvalidEnum(i)
    }
}


/// Error for invalid foreign key references
#[derive(Debug)]
pub struct InvalidForeignKeyError<Row: std::fmt::Debug> {
    /// The name of the table that was invalid.
    pub table: &'static str,

    /// The row that was invalid.
    pub row: Row,

    /// The id of the row that was invalid (if table has a primary key).
    pub row_id: Option<PrimaryKeyNumber>,

    /// The id of the foreign key that was invalid.
    pub foreign_key: PrimaryKeyNumber,
}

impl<Row: std::fmt::Debug> InvalidForeignKeyError<Row> {
    /// Creates a new InvalidForeignKeyError
    pub fn new(
        table: &'static str,
        row: Row,
        row_id: Option<PrimaryKeyNumber>,
        foreign_key: PrimaryKeyNumber,
    ) -> Self {
        Self {
            table,
            row,
            row_id,
            foreign_key,
        }
    }
}

impl<Row: std::fmt::Debug> Display for InvalidForeignKeyError<Row> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid foreign key reference in table '{}'. Row: '{:?}', row_id: '{:?}', foreign_key: '{}'",
            self.table,
            self.row,
            self.row_id.map(|id| id.to_string()),
            self.foreign_key.to_string(),
        )
    }
}

impl<Row: std::fmt::Debug> Error for InvalidForeignKeyError<Row> {}

/// Enum for storing the primary key of a table in instances of `DbcError`.
///
/// This is used to store the primary key of a table in the `InvalidForeignKey` error.
/// Since some tables use `u32` and some use `i32`, we need to store both types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimaryKeyNumber {
    /// The primary key is a `u32`.
    U32(u32),

    /// The primary key is a `i32`.
    I32(i32),
}

impl From<u32> for PrimaryKeyNumber {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<i32> for PrimaryKeyNumber {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<PrimaryKeyNumber> for i64 {
    fn from(value: PrimaryKeyNumber) -> i64 {
        match value {
            PrimaryKeyNumber::U32(v) => v as i64,
            PrimaryKeyNumber::I32(v) => v as i64,
        }
    }
}

impl PrimaryKeyNumber {
    /// Creates a new PrimaryKeyNumber from a u32 value
    pub fn new(value: u32) -> Self {
        Self::U32(value)
    }

    /// Gets the value as i64 regardless of the internal type
    pub fn as_i64(&self) -> i64 {
        match self {
            PrimaryKeyNumber::U32(v) => *v as i64,
            PrimaryKeyNumber::I32(v) => *v as i64,
        }
    }

    /// Gets the value as a displayable string
    pub fn to_string(&self) -> String {
        match self {
            PrimaryKeyNumber::U32(v) => v.to_string(),
            PrimaryKeyNumber::I32(v) => v.to_string(),
        }
    }
}

impl Display for PrimaryKeyNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimaryKeyNumber::U32(v) => write!(f, "{}", v),
            PrimaryKeyNumber::I32(v) => write!(f, "{}", v),
        }
    }
}
