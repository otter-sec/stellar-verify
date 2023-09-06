use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatIdent, Stmt};

const KANI_UNWIND: usize = 10;

#[proc_macro_attribute]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn contract(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn verify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Generate the output code
    let expanded = quote! {

        #input_fn

        fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient, TokenAdminClient) {
        let contract_address = e.register_stellar_asset_contract(admin.clone());
            (
                TokenClient::new(e, &contract_address),
                TokenAdminClient::new(e, &contract_address),
            )
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as an ItemFn
    let input_fn = parse_macro_input!(item as ItemFn);
    // Extract the function name
    let fn_name = &input_fn.sig.ident;

    // Parse the custom assignments provided in the attribute
    let custom_assignments = parse_macro_input!(attr as InitParams);

    // Create a Vec to store the input argument names
    let mut arg_names = Vec::new();

    // Iterate over the function's arguments and initialize them
    for input_arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat) = input_arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat.pat {
                let arg_name = ident.clone();
                // let arg_ty = &pat.ty;
                arg_names.push(arg_name);
            }
        }
    }

    let expanded = quote! {

        #input_fn


        #[kani::proof]
        #[kani::unwind(#KANI_UNWIND)]
        pub fn verify() {

            #custom_assignments

            let _ = env.register_contract(None);


            Self::#fn_name(#(#arg_names),*);
        }
    };

    expanded.into()
}

struct InitParams {
    pub assignments: Vec<Stmt>,
}

impl syn::parse::Parse for InitParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::braced!(content in input);

        let assignments = content.call(syn::Block::parse_within)?;

        Ok(Self { assignments })
    }
}

// Implement ToTokens
impl quote::ToTokens for InitParams {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for assignment in &self.assignments {
            assignment.to_tokens(tokens);
        }
    }
}
