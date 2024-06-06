use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    syntax_in::*,
    common::collect_until_punct::*,
};

pub(crate) fn bindings_step_entrance(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,

    entity_clause: TokenStream, 
    query_clause: TokenStream,
) {

}

fn collect_until_bindings_end(
    mut caravan: TokenIter, 
    mut output: Vec<TokenTree>
) -> Result<(TokenIter, Vec<TokenTree>), ()> {
    let token = caravan.next();
    let Some(token) = token else { // Expect to be un-nested or else throw an error.
        return Ok((caravan, output))
    };

    let TokenTree::Punct(token) = token else { // Is Punct?
        output.push(token);
        return collect_until_bindings_end(caravan, output) // If not, continue and add token to output.
    };

    if token == LINE_BREAK || token == NEXT { // Is valid singular token?
        return Ok((caravan, output))
    }

    match token.spacing() { // Is a token combo?
        Spacing::Joint => {/* Proceed */},
        Spacing::Alone => {
            output.push(TokenTree::Punct(token));
            return collect_until_bindings_end(caravan, output) // If not, continue and add token to output.
        },
    }

    // Is INTO_NEXT punct combo?
    let (results, caravan, mut output) = match_one_punct_combo(INTO_NEXT.iter(), caravan, token, output);
    match results {
        PunctMatch::Matching => return Ok((caravan, output)),
        _ => {
            return collect_until_bindings_end(caravan, output) // If not, continue. (token is already added to output because of match_one_punct_combo).
        },
    }
}