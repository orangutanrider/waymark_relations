use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    bindings_step::bindings_step_entrance, common::collect_until_punct::*, syntax_in::*
};

pub(super) fn query_step_exit(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,

    entity_clause: Vec<TokenTree>, 
    current: TokenTree, 
) -> Result<(TokenIter, TokenStream), ()> {
    let (caravan, query_clause) = match collect_query_clause(caravan, current) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return bindings_step_entrance(caravan, package, exit_rule, entity_clause, query_clause) 
}

fn collect_query_clause(
    caravan: TokenIter, 
    current: TokenTree,
) -> Result<(TokenIter, Vec<TokenTree>), ()> {
    let mut output = Vec::new();
    output.push(current);
    let (result, iter, output) = collect_until_matching_punct(QUERY_TO_BINDINGS_PUNCT, caravan, output);

    match result {
        PunctMatch::Matching => return Ok((iter, output)),
        PunctMatch::NotMatching => return Err(()),
        // PunctMatch::ConnectedMatch => return Err(()),
    }
}