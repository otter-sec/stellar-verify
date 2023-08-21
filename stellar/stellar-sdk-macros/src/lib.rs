use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn contract(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    input
}
