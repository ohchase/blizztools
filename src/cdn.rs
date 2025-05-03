use serde::Serialize;

use crate::{
    parse::{parse_named_attribute, parse_named_attribute_pair, ParserError},
    Md5Hash,
};

#[derive(Debug, Serialize)]
pub struct BuildConfig {
    pub root: Md5Hash,
    pub install: (Md5Hash, Md5Hash),
    pub install_size: (u32, u32),
    pub download: (Md5Hash, Md5Hash),
    pub download_size: (u32, u32),
    pub encoding: (Md5Hash, Md5Hash),
    pub encoding_size: (u32, u32),
}

pub fn parse_build_config(data: &str) -> Result<BuildConfig, ParserError> {
    let mut lines = data.lines();
    let _title = lines.next().ok_or(ParserError::Exhausted)?;
    let _empty = lines.next().ok_or(ParserError::Exhausted)?;

    //     # Build Configuration
    //
    // root = 74260639df2c36f256dec1dc99007dee
    // install = cb771e4587a2e7d3df2aa0a0802a1fc9 5707c55346b2bdffdc12587673ca6e78
    // install-size = 17491 16957
    // download = 742820d6e2a8e08c657b2f6402f5beb3 0ee936e6e1c5eda32dad6e133eb24b02
    // download-size = 9391314 8189832
    // size = 04b685919f85d762322f635a207d85d2 1a98c149a20d884fe4a6d6ec507b0dcd
    // size-size = 6043993 5280643
    // encoding = 81d6b3444dbb7113f69c7625361dbb91 9ea78760c2cfe3c9c3ccd42bf2057f95
    // encoding-size = 23840656 23805555
    let root = parse_named_attribute("root", &mut lines)?;
    let install = parse_named_attribute_pair("install", &mut lines)?;
    let install_size = parse_named_attribute_pair::<u32>("install-size", &mut lines)?;

    let download = parse_named_attribute_pair("download", &mut lines)?;
    let download_size = parse_named_attribute_pair("download-size", &mut lines)?;

    let _size = parse_named_attribute_pair::<Md5Hash>("size", &mut lines)?;
    let _size_size = parse_named_attribute_pair::<u32>("size-size", &mut lines)?;

    let encoding = parse_named_attribute_pair("encoding", &mut lines)?;
    let encoding_size = parse_named_attribute_pair("encoding-size", &mut lines)?;

    Ok(BuildConfig {
        root,
        install,
        install_size,
        download,
        download_size,
        encoding,
        encoding_size,
    })
}
