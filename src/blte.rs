use binrw::{BinRead, BinResult};
use flate2::bufread::ZlibDecoder;
use std::io::Read;

#[binrw::parser(reader, endian)]
fn chunk_data_parser(chunk_info_entries: &[ChunkInfoEntry]) -> BinResult<Vec<DataChunk>> {
    let mut data_chunks: Vec<DataChunk> = Vec::with_capacity(chunk_info_entries.len());
    for chunk_info_entry in chunk_info_entries {
        data_chunks.push(DataChunk::read_options(
            reader,
            endian,
            (chunk_info_entry.compressed_size,),
        )?)
    }
    Ok(data_chunks)
}

#[derive(Debug, BinRead)]
#[br(big, magic = b"BLTE")]
pub struct BlockTable {
    pub header_size: u32,
    pub chunk_info: ChunkInfo,

    #[br(count = usize::from(chunk_info.chunk_count))]
    pub chunk_info_entries: Vec<ChunkInfoEntry>,
    #[br(parse_with = chunk_data_parser, args (&chunk_info_entries,))]
    pub chunk_data: Vec<DataChunk>,
}

impl BlockTable {
    pub fn decompress(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut full_data = Vec::new();

        for data_chunk in &self.chunk_data {
            match data_chunk.encoding_mode {
                EncodingMode::PlainData => {
                    full_data.extend_from_slice(&data_chunk.data);
                }
                EncodingMode::Zlib => {
                    let cursor = std::io::Cursor::new(&data_chunk.data);
                    let mut decoder = ZlibDecoder::new(cursor);
                    let mut decoded_bytes = Vec::new();
                    decoder.read_to_end(&mut decoded_bytes)?;
                    full_data.extend_from_slice(&decoded_bytes);
                }
                EncodingMode::Recursive => todo!(),
                EncodingMode::Encrypted => todo!(),
            }
        }
        Ok(full_data)
    }
}

#[derive(Debug, BinRead)]
pub struct ChunkInfo {
    pub flags: u8,
    pub flag_ext: u8,
    pub chunk_count: u16,
}

#[derive(Debug, BinRead)]
pub struct ChunkInfoEntry {
    pub compressed_size: u32,
    pub decompressed_size: u32,
    pub checksum: [u8; 16],
}

#[derive(PartialEq, Eq, Debug, BinRead)]
pub enum EncodingMode {
    #[br(magic = b'N')]
    PlainData,
    #[br(magic = b'Z')]
    Zlib,
    #[br(magic = b'F')]
    Recursive,
    #[br(magic = b'E')]
    Encrypted,
}

#[derive(Debug, BinRead)]
#[br(import(compressed_size: u32,))]
pub struct DataChunk {
    pub encoding_mode: EncodingMode,

    #[br(count = compressed_size - 1)]
    pub data: Vec<u8>,
}
