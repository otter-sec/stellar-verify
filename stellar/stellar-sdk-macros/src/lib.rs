use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use soroban_rs_spec::generate_from_file;
use syn::{
    parse_macro_input, Block, Data, DataEnum, DeriveInput, Error, Expr, Fields, FieldsNamed,
    FieldsUnnamed, FnArg, ItemFn, ItemTrait, Pat, PatIdent,
};

const KANI_UNWIND: usize = 20;

#[proc_macro_attribute]
pub fn contractimpl(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as syn::ItemImpl);
    let struct_name = input.self_ty.as_ref();

    let name = if let syn::Type::Path(syn::TypePath { path, .. }) = struct_name {
        path.segments
            .last()
            .map(|seg: &syn::PathSegment| seg.ident.clone())
            .unwrap()
    } else {
        return syn::Error::new_spanned(input, "Expected an impl for a struct")
            .to_compile_error()
            .into();
    };

    let client = syn::Ident::new(&format!("{}Client", name), name.span());

    let methods = input.items.clone().into_iter().filter_map(|item| {
        if let syn::ImplItem::Fn(method) = item {
            let output = &method.sig.output;
            let method_name = &method.sig.ident;
            // let inputs = &method.sig.inputs;

            let mut inputs = Vec::new();
            inputs.push(syn::parse_quote! { &self });
            for arg in method.sig.inputs.iter().skip(1) {
                let transformed_arg = if let FnArg::Typed(pat_type) = arg {
                    let syn::PatType { pat, ty, attrs, .. } = pat_type;
                    let new_ty = quote! { &#ty };
                    syn::parse_quote! { #(#attrs)* #pat: #new_ty }
                } else {
                    arg.clone()
                };

                inputs.push(transformed_arg);
            }

            let ret = match output {
                syn::ReturnType::Default => quote! {
                    pub fn #method_name(#(#inputs),*) #output {}
                },
                syn::ReturnType::Type(_, _) => {
                    quote! {
                            pub fn #method_name(#(#inputs),*) #output {
                                kani::any()
                            }

                    }
                }
            };

            Some(ret)
        } else {
            None
        }
    });

    quote! {
        #input

        #[cfg(any(kani, feature = "kani"))]
        impl<'a> #client<'a> {
            #( #methods )*

        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn contract(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as syn::ItemStruct);
    let name = &item.ident;

    let client = format_ident!("{}Client", name, span = name.span());

    quote! {
        use soroban_sdk::{
            token::AdminClient as TokenAdminClient_, token::Client as TokenClient_, verify, EnvTrait
        };
        #[cfg(any(kani, feature = "kani"))]
        use soroban_sdk::kani;

        #item

        pub struct #client<'a> {
            pub env: soroban_sdk::Env,
            pub address: soroban_sdk::Address,
            _phantom: core::marker::PhantomData<&'a ()>,
        }

        impl<'a> #client<'a> {
            pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
                Self {
                    env: env.clone(),
                    address: address.clone(),
                    _phantom: core::marker::PhantomData,
                }
            }
        }

        impl #name {
            fn create_token_contract<'a>(e: &soroban_sdk::Env, admin: &soroban_sdk::Address) -> (TokenClient_, TokenAdminClient_) {
                let contract_address = e.register_stellar_asset_contract(admin.clone());
                (
                    TokenClient_::new(e, &contract_address),
                    TokenAdminClient_::new(e, &contract_address),
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
    let visiblity = item_fn.vis.clone();

    let mut precondition: TokenStream = quote! {
       {}
    };
    let mut succeeds_if: Option<TokenStream> = None;
    let mut postcondition: Option<TokenStream> = None;
    for attr in std::mem::take(&mut item_fn.attrs).into_iter() {
        if attr.path().is_ident("init") {
            precondition = attr.parse_args::<Expr>().unwrap().to_token_stream();
        } else if attr.path().is_ident("succeeds_if") {
            succeeds_if = Some(attr.parse_args::<Expr>().unwrap().to_token_stream());
        } else if attr.path().is_ident("post_condition") {
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

    // Set the default name of the environment variable
    let mut env_name = Ident::new("env", proc_macro2::Span::call_site());

    let mut env_clone_register_contract = Vec::new();

    // Iterate over the function's arguments and add their names to the Vec
    for input_arg in &item_fn.sig.inputs {
        if let FnArg::Typed(pat) = input_arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat.pat {
                let arg_name = ident.clone();
                let arg_ty = &pat.ty;

                if let syn::Type::Path(path) = arg_ty.as_ref() {
                    if let Some(segment) = path.path.segments.first() {
                        if segment.ident == "Env" {
                            // Update the name of the environment variable
                            env_name = arg_name.clone();
                            // The argument type is Env
                            let cloned_env =
                                format_ident!("{}_clone", arg_name, span = arg_name.span());
                            arg_names.push(cloned_env.clone());

                            env_clone_register_contract.push(quote! {
                                // Clone the environment
                                let #cloned_env = #arg_name.clone();
                            });
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
        #visiblity fn #proof_name() {
            // Register the contract
            let #env_name = kani::any::<Env>();
            let _ = #env_name.register_contract(None, 0);
            // First: Initialize the environment and declare the variables
            #(#arg_initializations)*
            #(#extracted_content)*

            #(#env_clone_register_contract)*

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

#[proc_macro_attribute]
pub fn verifiable(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let Ok(item_trait) = syn::parse2::<ItemTrait>(input) else {
        panic!("use #[verifiable] on a trait")
    };

    // Extract the trait name
    let trait_name = &item_trait.ident;

    // Generate verify_ functions for each trait method
    let verify_functions = item_trait
        .items
        .iter()
        .filter_map(|item| {
            if let syn::TraitItem::Fn(method) = item {
                let method_name = &method.sig.ident;
                let verify_method_name =
                    format_ident!("verify_{}", method_name, span = method_name.span());
                Some(quote! {
                    #item
                    fn #verify_method_name() {
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Combine the generated test functions into a single TokenStream
    let expanded = quote! {
        trait #trait_name {
            #( #verify_functions )*
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn contractmeta(_metadata: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
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
            pub struct Client<'a> {
                pub env: soroban_sdk::Env,
                pub address: soroban_sdk::Address,
                _phantom: core::marker::PhantomData<&'a ()>,
            }

            impl<'a> Client<'a> {
                pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
                    Self {
                        env : env.clone(),
                        address: address.clone(),
                        _phantom: core::marker::PhantomData,
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

    let mut derive_arbitrary = quote! {
        #[cfg_attr(any(kani, feature="kani"), derive(kani::Arbitrary))]
    };

    let derived = match &input.data {
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

                // Generate to_le_bytes and from_le_bytes
                let to_from_bytes = generate_from_to_le_bytes(struct_name.clone());
                // Combine serialization and deserialization code
                let result = quote! {
                    #derive_arbitrary
                    #input
                    impl #struct_name {
                        #serialize_code
                        #deserialize_code
                        #to_from_bytes
                    }
                    #traits_code
                };

                return result.into();
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                // Get the name of the struct
                let struct_name = &input.ident;

                // Generate the serialization code
                let serialize_code = generate_serialize_code_unnamed(unnamed);

                // Generate the deserialization code
                let deserialize_code = generate_deserialize_code_unnamed(unnamed);

                // Generate the code for the FromValEnum and ToValEnum traits
                let traits_code = generate_traits_for_structs(struct_name.clone());

                // Generate to_le_bytes and from_le_bytes
                let to_from_bytes = generate_from_to_le_bytes(struct_name.clone());

                // Combine serialization and deserialization code
                let result = quote! {
                    #derive_arbitrary
                    #input
                    impl #struct_name {
                        #serialize_code
                        #deserialize_code
                        #to_from_bytes
                    }
                    #traits_code
                };

                return result.into();
            }
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

            // Only derive kani if there are more than one variants
            if enum_data.variants.len() <= 1 {
                derive_arbitrary = quote! {};
            }

            expanded
        }
        Data::Union(_u) => {
            Error::new(ident.span(), "unions are unsupported as contract types").to_compile_error()
        }
    };

    quote! {
        #derive_arbitrary
        #struct_in

        #derived
    }
    .into()
}

#[proc_macro_attribute]
pub fn contracterror(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::ItemEnum);

    let enum_name = &input.ident;

    // The rest remains the same
    let expanded = quote! {
        #input

        #[cfg(kani)]
        impl kani::Arbitrary for #enum_name {
            fn any() -> Self {
                kani::any()
            }
        }

        impl core::fmt::Display for #enum_name {
             fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                 write!(f, "{:?}", self)
             }
         }
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
        pub fn serialize(&self) -> alloc::vec::Vec<u8> {
            let mut buf = alloc::vec::Vec::new();
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

        let size_expr = match field_ty {
            syn::Type::Path(type_path) if type_path.path.segments.len() == 1 => {
                let segment = &type_path.path.segments[0];
                if segment.ident == "BytesN" {
                    match &segment.arguments {
                        syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                            let arg = &args.args[0];
                            quote! { #arg }
                        }
                        _ => quote! { core::mem::size_of::<#field_ty>() },
                    }
                } else {
                    quote! { core::mem::size_of::<#field_ty>() }
                }
            }
            _ => quote! { core::mem::size_of::<#field_ty>() },
        };

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
            let mut #field_name_bytes = [0u8; #size_expr];
            #field_name_bytes.copy_from_slice(&buf[offset..offset + #size_expr]);
            let #field_name = <#field_ty>::from_le_bytes(#field_name_bytes);
            offset += #size_expr;
        }
    });

    quote! {
        pub fn deserialize(buf: &[u8]) -> Self {
            let mut offset = 0;
            #( #field_deserialization_statements )*
            Self {
                #( #field_names ),*
            }
        }
    }
}

fn generate_serialize_code_unnamed(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let field_serialization_statements = fields.iter().enumerate().map(|(i, field)| {
        let field_ty = &field.ty;
        quote! {
            let mut field_bytes = [0u8; core::mem::size_of::<#field_ty>()];
            field_bytes.copy_from_slice(&self.#i.to_le_bytes());
            result.extend_from_slice(&field_bytes);
        }
    });

    quote! {
        pub fn serialize(&self) -> alloc::vec::Vec<u8> {
            let mut result = alloc::vec::Vec::new();
            #( #field_serialization_statements )*
            result
        }
    }
}

fn generate_deserialize_code_unnamed(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let field_deserialization_statements = fields.iter().enumerate().map(|(i, field)| {
        let field_ty = &field.ty;
        let field_index = syn::Index::from(i);
        let field_name = Ident::new(&format!("field_{}", i), field_index.span);

        let field_name_bytes = format_ident!("{}_bytes", field_name);
        quote! {
            let mut #field_name_bytes = [0u8; core::mem::size_of::<#field_ty>()];
            #field_name_bytes.copy_from_slice(&buf[offset..offset + core::mem::size_of::<#field_ty>()]);
            let #field_name = <#field_ty>::from_le_bytes(#field_name_bytes);
            offset += core::mem::size_of::<#field_ty>();
        }
    });

    let field_statements = fields.iter().enumerate().map(|(i, _)| {
        let field_index = syn::Index::from(i);
        let field_name = Ident::new(&format!("field_{}", i), field_index.span);
        quote! {
            #field_name
        }
    });

    quote! {
        pub fn deserialize(buf: &[u8]) -> Self {
            let mut offset = 0;
            #( #field_deserialization_statements )*
            Self(
                #( #field_statements ),*
            )
        }
    }
}

fn generate_traits_for_structs(name: Ident) -> proc_macro2::TokenStream {
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

fn generate_from_to_le_bytes(name: Ident) -> proc_macro2::TokenStream {
    quote! {
        fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
            self.serialize()
        }

        fn from_le_bytes(bytes: alloc::vec::Vec<u8>) -> #name {
            #name::deserialize(&bytes)
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
            }
            Fields::Named(_) => {
                quote! {
                    Error::new(
                        ident.span(),
                        "named structs are not supported as contract types"
                    )
                    .to_compile_error()
                }
            }
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

    quote! {
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
            }
            Fields::Named(_) => {
                quote! {
                    Error::new(
                        ident.span(),
                        "named structs are not supported as contract types"
                    )
                    .to_compile_error()
                }
            }
            Fields::Unnamed(unnamed_feilds) => {
                let ty = &unnamed_feilds.unnamed.last().unwrap().ty;
                quote! {
                    #variant_name => Some(#enum_name::#variant_ident(
                        <#ty>::from_le_bytes(
                            enumval.value.try_into().unwrap()
                        )
                    )),
                }
            }
        };

        arms.extend(arm);
    }
    arms.extend(quote! {
        _ => None,
    });

    quote! {
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

#[proc_macro_attribute]
pub fn contractclient(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}
