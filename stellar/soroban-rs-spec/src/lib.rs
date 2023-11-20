pub mod functions;
mod read;
pub mod types;

use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::ScSpecEntry;

use read::{from_wasm, FromWasmError};

use types::{generate_enum, generate_error_enum, generate_struct, generate_union};

#[derive(thiserror::Error, Debug)]
pub enum GenerateFromFileError {
    #[error("reading file: {0}")]
    Io(io::Error),
    #[error("sha256 does not match, expected: {expected}")]
    VerifySha256 { expected: String },
    #[error("parsing contract spec: {0}")]
    Parse(stellar_xdr::Error),
    #[error("getting contract spec: {0}")]
    GetSpec(FromWasmError),
}

pub fn generate_from_file(file: &str) -> Result<TokenStream, GenerateFromFileError> {
    let wasm = fs::read(file).map_err(GenerateFromFileError::Io)?;

    let spec = from_wasm(&wasm).map_err(GenerateFromFileError::GetSpec)?;
    let code = generate(&spec);
    Ok(code)
}

pub fn generate(specs: &[ScSpecEntry]) -> TokenStream {
    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    let mut spec_enums = Vec::new();
    let mut spec_error_enums = Vec::new();
    for s in specs {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
            ScSpecEntry::UdtEnumV0(e) => spec_enums.push(e),
            ScSpecEntry::UdtErrorEnumV0(e) => spec_error_enums.push(e),
        }
    }

    let trait_name = "Contract";

    let functions_impl = functions::generate_fns(trait_name, &spec_fns);
    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    let enums = spec_enums.iter().map(|s| generate_enum(s));
    let error_enums = spec_error_enums.iter().map(|s| generate_error_enum(s));

    quote! {

        #functions_impl

        #(#structs)*
        #(#unions)*
        #(#enums)*
        #(#error_enums)*
    }
}
