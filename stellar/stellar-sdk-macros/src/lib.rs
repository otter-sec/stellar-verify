use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Block, Expr, FnArg, ItemFn, Pat, PatIdent};

const KANI_UNWIND: usize = 10;

#[proc_macro_attribute]
pub fn contractimpl(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

#[proc_macro_attribute]
pub fn contract(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
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
       let _ = Env::new();
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

    let input: proc_macro::TokenStream = precondition.into();
    // Parse the input as a Block
    let block: Block = parse_macro_input!(input);

    // Extract the content of the block which inlclude's the variable declarations
    let extracted_content = &block.stmts;

    let proof_name = format_ident!("verify_{}", function_name, span = function_name.span());

    // Create a Vec to store the input argument names
    let mut arg_names = Vec::new();

    // Iterate over the function's arguments and add their names to the Vec
    for input_arg in &item_fn.sig.inputs {
        if let FnArg::Typed(pat) = input_arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat.pat {
                let arg_name = ident.clone();
                // let arg_ty = &pat.ty;
                arg_names.push(arg_name);
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

        fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient, TokenAdminClient) {
        let contract_address = e.register_stellar_asset_contract(admin.clone());
            (
                TokenClient::new(e, &contract_address),
                TokenAdminClient::new(e, &contract_address),
            )
        }

        #item_fn

        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND)]
        pub fn #proof_name() {

            // First: Initialize the environment and declare the variables
            #(#extracted_content)*

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
