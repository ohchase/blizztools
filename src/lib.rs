#![allow(dead_code)]

use std::fmt::Write;

use binrw::BinResult;
use binrw::{BinRead, BinReaderExt};
use thiserror::Error;

pub mod blte;
pub mod cdn;
pub(crate) mod parse;
pub mod tact;

#[derive(Debug, Error)]
#[error("md5 decoding error")]
pub struct Md5Error;

impl std::fmt::Debug for Md5Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(BinRead, PartialEq, Eq)]
pub struct Md5Hash(pub [u8; 16]);

impl Md5Hash {
    /// Whether the MD5 Hash is all zeroes
    pub fn is_null(&self) -> bool {
        self.0 == [0u8; 16]
    }

    /// 32 character hexadecimal representation of the MD5 Hash
    pub fn as_str(&self) -> String {
        let mut s = String::with_capacity(32);
        for &b in &self.0 {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }
}

impl std::str::FromStr for Md5Hash {
    type Err = Md5Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut decoded = [0; 16];
        hex::decode_to_slice(s, &mut decoded).map_err(|_| Md5Error)?;
        Ok(Self(decoded))
    }
}

#[derive(Debug, BinRead)]
#[br(big, magic = b"IN")]
pub struct InstallManifest {
    pub version: u8,
    pub encoding_hash_size: u8,
    pub num_tags: u16,
    pub num_entries: u32,
    #[br(count = num_tags, args { inner: (num_entries / 8,) })]
    pub tags: Vec<ManifestTag>,
    #[br(count = num_entries)]
    pub entries: Vec<InstallManifestEntry>,
}

#[derive(Debug, BinRead)]
pub struct InstallManifestEntry {
    pub name: binrw::NullString,
    pub hash: Md5Hash,
    pub size: u32,
}

#[derive(Debug, BinRead)]
#[br(big, magic = b"DL")]
pub struct DownloadManifest {
    pub version: u8,
    pub encoding_key_size: u8,
    pub include_checksum: u8,
    pub num_entries: u32,
    pub num_tags: u16,
    #[br(count = num_entries)]
    pub entries: Vec<DownloadManifestEntry>,
    #[br(count = num_tags, args { inner: (num_entries / 8,) })]
    pub tags: Vec<ManifestTag>,
}

#[derive(Debug, BinRead)]
pub struct DownloadManifestEntry {
    pub hash: Md5Hash,
    pub file_size: [u8; 5],
    pub priority: u8,
}

#[derive(Debug, BinRead)]
#[br(import(mask_len: u32))]
pub struct ManifestTag {
    pub name: binrw::NullString,
    pub tag_type: u16,
    #[br(count = mask_len)]
    pub mask: Vec<u8>,
}

#[binrw::parser(reader)]
fn key_table_data_parser(page_size_kb: u16, num_pages: u32) -> BinResult<Vec<CeKeyPageEntry>> {
    let mut results = Vec::new();
    let mut page_count = 0;
    while page_count != num_pages as usize {
        let mut page = vec![0u8; page_size_kb as usize * 1024];
        let read_n = reader.read(&mut page)?;
        if read_n != page_size_kb as usize * 1024 {
            return Err(binrw::Error::AssertFail {
                pos: reader.stream_position()?,
                message: format!(
                    "Unable to read a full page, Page: {}, Expected Pages: {}",
                    page_count, num_pages
                ),
            });
        }

        let mut cursor = binrw::io::Cursor::new(page);
        loop {
            match cursor.read_be::<CeKeyPageEntry>() {
                Ok(entry) => {
                    results.push(entry);
                }
                Err(_e) => {
                    break;
                }
            }
        }

        page_count += 1;
    }

    Ok(results)
}

#[derive(Debug, BinRead)]
#[br(big, magic = b"EN")]
pub struct EncodingManifest {
    pub version: u8,

    pub ckey_hash_size: u8,
    pub ekey_hash_size: u8,

    pub ce_page_size_kb: u16,
    pub e_page_size_kb: u16,

    pub ce_key_table_page_count: u32,
    pub e_key_table_count: u32,

    _unknown: u8,
    pub espec_block_size: u32,
    #[br(count = espec_block_size)]
    pub espec_block: Vec<u8>,
    #[br(count = ce_key_table_page_count)]
    pub ce_key_table_index: Vec<CeKeyTableIndex>,
    #[br(parse_with = key_table_data_parser, args (ce_page_size_kb, ce_key_table_page_count, ) )]
    pub ce_key_table_entries: Vec<CeKeyPageEntry>,
}

#[derive(Debug, BinRead)]
pub struct CeKeyTableIndex {
    pub first_key: Md5Hash,
    pub md5: Md5Hash,
}

#[derive(Debug, BinRead)]
pub struct CeKeyPageEntry {
    pub key_count: u8,
    pub file_size: [u8; 5],
    pub c_key: Md5Hash,
    #[br(count = key_count)]
    pub e_keys: Vec<Md5Hash>,
}

#[derive(Debug, BinRead)]
pub struct IndexEntry {
    pub e_key: Md5Hash,
    pub size: u32,
    pub offset: u32,
}

#[derive(Debug, BinRead)]
pub struct IndexFile {
    #[br(parse_with = index_table_data_parser)]
    pub index_entries: Vec<IndexEntry>,
}

#[binrw::parser(reader)]
fn index_table_data_parser() -> BinResult<Vec<IndexEntry>> {
    let mut results = Vec::new();

    loop {
        let mut page = vec![0u8; 4096];
        let read_n = reader.read(&mut page)?;
        if read_n != 4096 {
            return Ok(results);
        }

        let mut cursor = binrw::io::Cursor::new(page);
        loop {
            match cursor.read_be::<IndexEntry>() {
                Ok(entry) => {
                    if entry.e_key.is_null() {
                        tracing::info!(
                            "ending parsing index file, because we encountered a null hash"
                        );
                        return Ok(results);
                    }

                    results.push(entry);
                }
                Err(e) => {
                    tracing::warn!("ending with an err... {e:?}");
                    return Ok(results);
                }
            }
        }
    }
}
