mod convert_map;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ConvertMap, attributes(convert_map))]
pub fn convert_map_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let context = convert_map::ConvertMapDerive::from(input);
    let out: TokenStream = context.render().into();
    out
}
