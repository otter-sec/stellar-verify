use std::io::Cursor;

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limited, Limits, ReadXdr, ScSpecEntry};
use wasmparser::{BinaryReaderError, Parser, Payload};

pub fn parse_raw(spec: &[u8]) -> Result<Vec<ScSpecEntry>, stellar_xdr::Error> {
    let cursor = Cursor::new(spec);
    let entries = ScSpecEntry::read_xdr_iter(&mut Limited::new(
        cursor,
        Limits {
            depth: 500,
            len: 0x1000000,
        },
    ))
    .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

#[derive(thiserror::Error, Debug)]
pub enum FromWasmError {
    #[error("reading wasm")]
    Read(BinaryReaderError),
    #[error("parsing contract spec")]
    Parse(stellar_xdr::Error),
    #[error("contract spec not found")]
    NotFound,
}

pub fn raw_from_wasm(wasm: &[u8]) -> Result<Vec<u8>, FromWasmError> {
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.map_err(FromWasmError::Read)?;
        if let Payload::CustomSection(section) = payload {
            if section.name() == "contractspecv0" {
                return Ok(section.data().to_vec());
            }
        };
    }
    Err(FromWasmError::NotFound)
}

pub fn from_wasm(wasm: &[u8]) -> Result<Vec<ScSpecEntry>, FromWasmError> {
    let spec = raw_from_wasm(wasm)?;
    parse_raw(&spec).map_err(FromWasmError::Parse)
}
