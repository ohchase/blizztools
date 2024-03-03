use std::str::{FromStr, Lines};

use thiserror::Error;

use crate::Md5Error;

/// parses a named attribute such as "build_hash: {hash1}"
/// name should  be the expected attribute name
pub fn parse_named_attribute<T: FromStr>(
    name: &'static str,
    lines: &mut Lines,
) -> Result<T, ParserError> {
    let line = lines.next().ok_or(ParserError::Exhausted)?;
    let (key, value) = line.split_once(" = ").ok_or(ParserError::Exhausted)?;
    if name != key {
        return Err(ParserError::AttributeName(name, key.to_owned()));
    }
    let value = T::from_str(value).map_err(|_e| ParserError::FromStr)?;
    Ok(value)
}

/// parses a named attribute pair such as "build_hashes: {hash1} {hash2}"
/// name should be the expected attribute name
pub fn parse_named_attribute_pair<T: FromStr>(
    name: &'static str,
    lines: &mut Lines,
) -> Result<(T, T), ParserError> {
    let line = lines.next().ok_or(ParserError::Exhausted)?;
    let (key, value) = line.split_once(" = ").ok_or(ParserError::Exhausted)?;
    if name != key {
        return Err(ParserError::AttributeName(name, key.to_owned()));
    }

    let (value_0, value_1) = value.split_once(' ').ok_or(ParserError::Exhausted)?;
    let value_0 = T::from_str(value_0).map_err(|_e| ParserError::FromStr)?;
    let value_1 = T::from_str(value_1).map_err(|_e| ParserError::FromStr)?;
    Ok((value_0, value_1))
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("exhausted available lines")]
    Exhausted,

    #[error("invalid attribute name, expected {0} got {1}")]
    AttributeName(&'static str, String),

    #[error("error reading md5hash")]
    Md5Parse(#[from] Md5Error),

    #[error("from str error")]
    FromStr,
}
