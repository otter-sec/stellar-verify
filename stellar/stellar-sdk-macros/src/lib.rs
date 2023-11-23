use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Block, Data, DeriveInput, Error, Expr, Fields, FieldsNamed, FnArg, ItemFn, Pat, PatIdent, DataEnum,
};
use soroban_rs_spec::generate_from_file;

const KANI_UNWIND: usize = 20;

#[proc_macro_attribute]
pub fn contractimpl(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}


#[proc_macro_attribute]
pub fn contract(_metadata: proc_macro::TokenStream, input_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input_ as syn::ItemStruct);
    let name = &input.ident;

    quote! {
        use soroban_sdk::{
            token::AdminClient as TokenAdminClient, token::Client as TokenClient, verify, kani
        };

        #input

        impl #name {
            fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient, TokenAdminClient) {
                let contract_address = e.register_stellar_asset_contract(admin.clone());
                (
                    TokenClient::new(e, &contract_address),
                    TokenAdminClient::new(e, &contract_address),
                )
            }
        }
    }.into()
}

#[proc_macro_attribute]
pub fn verify(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let Ok(mut item_fn) = syn::parse2::<ItemFn>(input) else {
        panic!("use #[verify] on a function")
    };

    let function_name = item_fn.sig.ident.clone();

    let mut precondition: TokenStream = quote! {
       {}
    };
    let mut succeeds_if: Option<TokenStream> = None;
    let mut postcondition: Option<TokenStream> = None;
    for attr in std::mem::take(&mut item_fn.attrs).into_iter() {
        if attr.path.is_ident("init") {
            precondition = attr.parse_args::<Expr>().unwrap().to_token_stream();
        } else if attr.path.is_ident("succeeds_if") {
            succeeds_if = Some(attr.parse_args::<Expr>().unwrap().to_token_stream());
        } else if attr.path.is_ident("post_condition") {
            postcondition = Some(attr.parse_args::<Expr>().unwrap().to_token_stream());
        } else {
            item_fn.attrs.push(attr);
        }
    }

    if postcondition.is_none() {
        postcondition = Some(quote! { true });
    }

    if succeeds_if.is_none() {
        succeeds_if = Some(quote! { true });
    }

    let input: proc_macro::TokenStream = precondition.into();
    // Parse the input as a Block
    let block: Block = parse_macro_input!(input);
    
    let mut inited_vars = Vec::new();
    for stmt in block.clone().stmts {
        if let syn::Stmt::Local(local) = stmt {
            if let Pat::Ident(pat_ident) = local.pat {
                inited_vars.push(pat_ident.ident);
            }
        }
    }

    // Extract the content of the block which inlclude's the variable declarations
    let extracted_content = &block.stmts;

    let proof_name = format_ident!("verify_{}", function_name, span = function_name.span());

    // Create a Vec to store the input argument names
    let mut arg_names = Vec::new();
    let mut arg_initializations = Vec::new();

    // Iterate over the function's arguments and add their names to the Vec
    for input_arg in &item_fn.sig.inputs {
        if let FnArg::Typed(pat) = input_arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat.pat {
                let arg_name = ident.clone();
                let arg_ty = &pat.ty;
                if arg_name == "env" {
                    // Create new variable name for the cloned environment as env_clone
                    arg_names.push(Ident::new("env_clone", arg_name.span()));
                    arg_initializations.push(quote! {
                        let #arg_name = Env::default();
                    });
                    continue;
                } else {
                    arg_names.push(arg_name.clone());
                    if !inited_vars.contains(&arg_name.clone()) {
                        arg_initializations.push(quote! {
                            let #arg_name = kani::any::<#arg_ty>();
                        }); 
                    }
                }
            }
        }
    }

    let fn_call = if item_fn.sig.receiver().is_some() {
        quote! {
            let result = #function_name();
        }
    } else {
        quote! {
            let result = Self::#function_name(
                #(#arg_names),*
            );
        }
    };

    quote! {

        #item_fn

        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND)]
        #[kani::solver(kissat)]
        pub fn #proof_name() {

            // First: Initialize the environment and declare the variables
            #(#arg_initializations)*
            #(#extracted_content)*

            // Clone the environment
            let env_clone = env.clone();

            // Register the contract
            let _ = env.register_contract(None);

            // Assume the preconditions
            kani::assume(
                #succeeds_if
            );

            // Finally: Actually call the function we are trying to verify
            #fn_call

            // Assert the postconditions apply.
            assert!((#postcondition));

        }


    }
    .into()
}

#[proc_macro]
pub fn contractmeta(metadata: proc_macro::TokenStream) -> proc_macro::TokenStream {
    metadata
}

#[derive(Debug, FromMeta)]
#[allow(dead_code)]
struct ContractImportArgs {
    file: String,
    #[darling(default)]
    sha256: darling::util::SpannedValue<Option<String>>,
}

#[proc_macro]
pub fn contractimport(metadata: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let args = match NestedMeta::parse_meta_list(metadata.into()) {
            Ok(v) => v,
            Err(e) => {
                return proc_macro::TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        let args = match ContractImportArgs::from_list(&args) {
            Ok(v) => v,
            Err(e) => return e.write_errors().into(),
        };
    
        // Read WASM from file.
        let file_abs = abs_from_rel_to_manifest(args.file);
    
        // Generate.
        match generate_from_file(file_abs.to_str().unwrap()) {
            Ok(code) => quote! { 
                pub struct Client {
                    pub env: soroban_sdk::Env,
                    pub address: soroban_sdk::Address,
                }

                impl Client {
                    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
                        Self {
                            env : env.clone(),
                            address: address.clone()
                        }
                    }
                }
                #code 
            },
            Err(e) => Error::new(proc_macro2::Span::call_site(), e.to_string()).into_compile_error(),
        }
        .into()

    }


#[proc_macro_attribute]
pub fn contracttype(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let struct_in: TokenStream = input.clone().into();

    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let derived =
        match &input.data {
            Data::Struct(s) => match &s.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    // Get the name of the struct
                    let struct_name = &input.ident;

                    // Generate the serialization code
                    let serialize_code = generate_serialize_code(named);

                    // Generate the deserialization code
                    let deserialize_code = generate_deserialize_code(named);

                    // Generate the code for the FromValEnum and ToValEnum traits
                    let traits_code = generate_traits_for_structs(struct_name.clone());

                    // Combine serialization and deserialization code
                    let result = quote! {
                        #input
                        impl #struct_name {
                            #serialize_code
                            #deserialize_code
                        }
                        #traits_code
                    };

                    return result.into();
                },
                Fields::Unnamed(_) => Error::new(
                    ident.span(),
                    "tuple structs are not supported as contract types",
                )
                .to_compile_error(),
                Fields::Unit => Error::new(
                    ident.span(),
                    "unit structs are not supported as contract types",
                )
                .to_compile_error(),
            },
            Data::Enum(enum_data) => {
                let enum_name = &input.ident;
                let to_val_enum_impl = generate_to_val_enum(enum_data, enum_name);
                let from_val_enum_impl = generate_from_val_enum(enum_data, enum_name);

                let expanded = quote! {
                    #to_val_enum_impl

                    #from_val_enum_impl
                };
                


                expanded
            },
            Data::Union(_u) => Error::new(ident.span(), "unions are unsupported as contract types")
                .to_compile_error(),
        };
    quote! {

        #struct_in

        #derived
    }
    .into()
}



#[proc_macro_attribute]
pub fn contracterror(_attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::ItemEnum);

    // The rest remains the same
    let expanded = quote! {
        #input
    };

    expanded.into()
}


fn generate_serialize_code(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let serialization_statements = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            buf.extend_from_slice(&self.#field_name.to_le_bytes());
        }
    });

    quote! {
        pub fn serialize(&self) -> Vec<u8> {
            let mut buf = Vec::new();
            #( #serialization_statements )*
            buf
        }
    }
}

fn generate_deserialize_code(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name
        }
    });
    let field_deserialization_statements = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        let field_name_bytes = match field_name {
            Some(ident) => format_ident!("{}_bytes", ident.to_string(), span = ident.span()),
            None => {
                // Handle the case where the field identifier is not provided
                return quote! {
                    return None;
                };
            }
        };
        quote! {            
            let mut offset = 0;
            let mut #field_name_bytes = [0u8; core::mem::size_of::<#field_ty>()];
            #field_name_bytes.copy_from_slice(&buf[offset..offset + core::mem::size_of::<#field_ty>()]);
            let #field_name = <#field_ty>::from_le_bytes(#field_name_bytes);
            offset += core::mem::size_of::<#field_ty>();
        }
    });

    quote! {
        pub fn deserialize(buf: &[u8]) -> Self {
            #( #field_deserialization_statements )*
            Self {
                #( #field_names ),*
            }
        }
    }
}

fn generate_traits_for_structs(name:Ident) -> proc_macro2::TokenStream {
    quote! {
        impl soroban_sdk::FromValEnum for #name {
            fn from_val(val: soroban_sdk::Val) -> Option<Self> {
                match val {
                    soroban_sdk::Val::Struct(bytes) => Some(Self::deserialize(&bytes)),
                    _ => None,
                }
            }
        }

        impl soroban_sdk::ToValEnum for #name {
            fn to_val(&self) -> soroban_sdk::Val {
                soroban_sdk::Val::Struct(self.serialize())
            }
        }
    }
}

fn generate_to_val_enum(enum_data: &DataEnum, enum_name: &Ident) -> proc_macro2::TokenStream {
            let variants = &enum_data.variants;
            let mut arms = proc_macro2::TokenStream::new();

            for variant in variants {
                let variant_ident = &variant.ident;
                let variant_name = variant_ident.to_string();

                let arm = match &variant.fields {
                    Fields::Unit => {
                        quote! {
                            #enum_name::#variant_ident => soroban_sdk::Val::EnumVal(
                                soroban_sdk::EnumType {
                                    variant: soroban_sdk::symbol_short!(#variant_name),
                                    value: alloc::vec::Vec::new().into(),
                                }
                            ),
                        }
                    },
                    Fields::Named(_) => {
                       quote! {
                            Error::new(
                                ident.span(),
                                "named structs are not supported as contract types"
                            )
                            .to_compile_error()
                        }
                    },
                    Fields::Unnamed(_) => {
                        quote! {
                            #enum_name::#variant_ident(data) => soroban_sdk::Val::EnumVal(
                                soroban_sdk::EnumType {
                                    variant: soroban_sdk::symbol_short!(#variant_name),
                                    value: data.to_le_bytes().to_vec().into(),
                                }
                            ),
                        }
                    }
                };

                arms.extend(arm);
            }

            quote!{
                impl soroban_sdk::ToValEnum for #enum_name {
                    fn to_val(&self) -> soroban_sdk::Val {
                        match self {
                            #arms
                        }
                    }
                }
            }

}

fn generate_from_val_enum(data: &DataEnum, enum_name: &Ident) -> proc_macro2::TokenStream {
    let variants = &data.variants;  
            let mut arms = proc_macro2::TokenStream::new();

            for variant in variants {
                let variant_ident = &variant.ident;
                let variant_name = variant_ident.to_string();

                let arm = match &variant.fields {
                    Fields::Unit => {
                        quote! {
                            #variant_name => Some(#enum_name::#variant_ident),
                        }
                    },
                    Fields::Named(_) => {
                        quote! {
                            Error::new(
                                ident.span(),
                                "named structs are not supported as contract types"
                            )
                            .to_compile_error()
                        }

                    },
                    Fields::Unnamed(unnamed_feilds) => {
                        let ty = &unnamed_feilds.unnamed.last().unwrap().ty;
                        quote! {
                            #variant_name => Some(#enum_name::#variant_ident(
                                <#ty>::from_le_bytes(
                                    enumval.value[0..core::mem::size_of::<#ty>()].try_into().unwrap()
                                )
                            )),
                        }
                    },
                };

                arms.extend(arm);
            }
            arms.extend(
                quote! {
                    _ => None,
                }
            );

            quote!{
                impl soroban_sdk::FromValEnum for #enum_name {
                    fn from_val(val: soroban_sdk::Val) -> Option<Self> {
                        if let soroban_sdk::Val::EnumVal(enumval) = val {
                            match enumval.variant.as_str() {
                                #arms
                            }
                        }  else {
                            None
                        }
                    }
                }
            }
            
}




fn abs_from_rel_to_manifest(path: impl Into<std::path::PathBuf>) -> std::path::PathBuf {
    let path: std::path::PathBuf = path.into();
    if path.is_relative() {
        let root: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable is required to be set")
            .into();
        root.join(path)
    } else {
        path
    }
}