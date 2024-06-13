mod common;

mod syntax_in;
mod syntax_out;

// Caravan.
mod root_step;
mod entity_step;
mod query_step;
mod bindings_step;
mod construction_step;

use std::str::FromStr;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;
use root_step::root_step;
use syntax_out::exit_rule_default;

#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    let caravan = input.into_iter();
    let package = TokenStream::new();
    let exit_rule = exit_rule_default();
    let (_, package, _) = match root_step(caravan, package, exit_rule) {
        Ok(ok) => ok,
        Err(err) => {
            //return Err(err)
            return TokenStream::new()
        },
    };

    return package
}