use std::str::FromStr;

use crate::{parse::ParserError, Md5Hash};

fn next_or_exhaust<'a>(split: &mut impl Iterator<Item = &'a str>) -> Result<String, ParserError> {
    let next = split.next().ok_or(ParserError::Exhausted)?;
    Ok(next.to_owned())
}
/// Product configurations available
#[derive(Debug)]
pub struct VersionDefinition {
    pub region: String,
    pub build_config: Md5Hash,
    pub cdn_config: Md5Hash,
    pub key_ring: Option<Md5Hash>,
    pub build_id: String,
    pub version_name: String,
    pub product_config: Md5Hash,
}

/// Defines cdn servers available
#[derive(Debug)]
pub struct CdnDefinition {
    pub name: String,
    pub path: String,
    pub hosts: Vec<String>,
    pub servers: Vec<String>,
    pub config_path: String,
}

/// Parses a String representation of the Version Table
pub fn parse_version_table(data: &str) -> Result<Vec<VersionDefinition>, ParserError> {
    let mut resp_lines = data.split('\n');
    let _format_line = next_or_exhaust(&mut resp_lines)?;
    let _sequence_line = next_or_exhaust(&mut resp_lines)?;
    resp_lines
        .filter(|l| !l.is_empty())
        .map(parse_version_table_entry)
        .collect::<Result<Vec<VersionDefinition>, _>>()
}

/// Parses a String representation of the CDN table
pub fn parse_cdn_table(data: &str) -> Result<Vec<CdnDefinition>, ParserError> {
    let mut resp_lines = data.split('\n');
    let _format_line = next_or_exhaust(&mut resp_lines)?;
    let _sequence_line = next_or_exhaust(&mut resp_lines)?;
    resp_lines
        .filter(|l| !l.is_empty())
        .map(parse_cdn_table_entry)
        .collect::<Result<Vec<CdnDefinition>, _>>()
}

fn parse_cdn_table_entry(line: &str) -> Result<CdnDefinition, ParserError> {
    let mut splits = line.split('|');

    let name = next_or_exhaust(&mut splits)?;
    let path = next_or_exhaust(&mut splits)?;
    let servers = next_or_exhaust(&mut splits)?;
    let servers: Vec<String> = servers.split_whitespace().map(|s| s.to_owned()).collect();

    let hosts = next_or_exhaust(&mut splits)?;
    let hosts: Vec<String> = hosts.split_whitespace().map(|s| s.to_owned()).collect();
    let config_path = next_or_exhaust(&mut splits)?;

    Ok(CdnDefinition {
        name,
        path,
        hosts,
        servers,
        config_path,
    })
}

/// Parses version table entries
pub fn parse_version_table_entry(line: &str) -> Result<VersionDefinition, ParserError> {
    let mut splits = line.split('|');

    let region = next_or_exhaust(&mut splits)?;
    let build_config = next_or_exhaust(&mut splits).map(|b| Md5Hash::from_str(&b))??;

    let cdn_config = next_or_exhaust(&mut splits).map(|b| Md5Hash::from_str(&b))??;
    let key_ring: Option<Md5Hash> = next_or_exhaust(&mut splits)
        .map(|b| Md5Hash::from_str(&b))?
        .ok();
    let build_id = next_or_exhaust(&mut splits)?;
    let version_name = next_or_exhaust(&mut splits)?;
    let product_config = next_or_exhaust(&mut splits).map(|b| Md5Hash::from_str(&b))??;

    Ok(VersionDefinition {
        region,
        build_config,
        cdn_config,
        key_ring,
        build_id,
        version_name,
        product_config,
    })
}
