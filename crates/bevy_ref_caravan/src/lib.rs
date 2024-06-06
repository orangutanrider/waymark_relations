mod common;

mod syntax_in;
mod syntax_out;

// Caravan.
mod root_step;
mod entity_step;
mod query_step;
mod bindings_step;
mod next_step;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

//#[proc_macro]
//pub fn ref_caravan(input: TokenStream) -> TokenStream {
//
//}