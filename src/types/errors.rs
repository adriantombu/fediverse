use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TypeError {
    #[error("Value out of bound (expected minimum of {min:?}, found {found:?})")]
    OutOfBoundsMin { min: String, found: String },

    #[error("Value out of bound (expected maximum of {max:?}, found {found:?})")]
    OutOfBoundsMax { max: String, found: String },

    #[error("Unable to parse string as url")]
    UrlParse(#[from] url::ParseError),
}
