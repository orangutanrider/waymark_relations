mod caravan;

mod syntax_in;
mod syntax_out;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use caravan::*;

//#[proc_macro]
//pub fn ref_caravan(input: TokenStream) -> TokenStream {
//
//}