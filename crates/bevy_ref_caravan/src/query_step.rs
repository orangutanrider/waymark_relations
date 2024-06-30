use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    bindings_step::bindings_step, common::collect_until_punct::*, exit_rule_step::ExitRule, syntax_in::*, wildcard_step::EntityWildcard
};

pub(crate) enum QueryMutation {
    GetMut,
    Get,
}

pub(crate) fn query_step(
    current: TokenTree,

    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    is_nested: bool,

    entity_clause: (EntityWildcard, Vec<TokenTree>), 
) -> Result<(TokenIter, TokenStream), ()> {
    let (caravan, query_clause) = match collect_query_clause(caravan, current) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return bindings_step(caravan, package, exit_rule, is_nested, entity_clause, query_clause) 
}

fn collect_query_clause(
    caravan: TokenIter, 
    current: TokenTree,
) -> Result<(TokenIter, (Vec<TokenTree>, QueryMutation)), ()> {
    let mut output = Vec::new();
    let mut mutation = QueryMutation::Get;

    if current.to_string() == "mut" { // You can declare mutability within the query step.
        mutation = QueryMutation::GetMut;
    }
    else {
        output.push(current);
    }

    
    let (result, iter, output) = collect_until_matching_punct(QUERY_TO_BINDINGS_PUNCT, caravan, output);

    match result {
        PunctMatch::Matching => return Ok((iter, (output, mutation))),
        PunctMatch::NotMatching => return Err(()),
        // PunctMatch::ConnectedMatch => return Err(()),
    }
}