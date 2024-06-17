use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    entity_step::EntityWildcard,
    bindings_step::bindings_step, 
    common::collect_until_punct::*, 
    syntax_in::*
};

pub(crate) fn query_step(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    is_nested: bool,

    entity_clause: (EntityWildcard, Vec<TokenTree>), 
) -> Result<(TokenIter, TokenStream), ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(())
    };

    let (caravan, query_clause) = match collect_query_clause(caravan, token) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return bindings_step(caravan, package, exit_rule, is_nested, entity_clause, query_clause) 
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