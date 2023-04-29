use proc_macro::TokenStream;

#[cfg(test)]
mod tests;

#[proc_macro_attribute]
pub fn axemc_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
