mod common; use common::compile_error_stream;

mod syntax_in; 
mod syntax_out; use syntax_out::exit_rule_default;

// Caravan.
mod root_step; use root_step::root_step;
mod entity_step;
mod query_step;
mod bindings_step;
mod construction_step;

use proc_macro::*;
// use proc_macro::token_stream::IntoIter as TokenIter;

#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    let caravan = input.into_iter();
    let package = TokenStream::new();
    let exit_rule = exit_rule_default();
    let (_, package, _) = match root_step(caravan, package, exit_rule) {
        Ok(ok) => ok,
        Err(err) => {
            return compile_error_stream("Undefined")
        },
    };

    return package
}