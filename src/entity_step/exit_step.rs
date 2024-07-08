use crate::*;

use crate::{
    common::collect_until_punct::*, 
    query_step::query_step, syntax_in::* 
};

use super::EntityWildcard;

pub(super) fn raw_entity_step_exit(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
    is_nested: bool,

    raw: Group, 
    wildcard: EntityWildcard, 
) -> Result<(TokenIter, TokenStream), ()> {
    let result = collect_entity_clause(caravan, TokenTree::Group(raw.clone()));
    let (mut caravan, entity_clause) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let Some(current) = caravan.next() else { // Caravan next for query
        return Err(())
    };

    let token1 = entity_clause.get(1);
    match token1 {
        Some(_) => { // Raw has been invalidated
            return query_step(current, caravan, package, exit_rule, pre_process, is_nested, (wildcard, entity_clause));
        },
        None => { 
            let entity_clause = raw.stream().into_iter().collect();
            return query_step(current, caravan, package, exit_rule, pre_process, is_nested, (wildcard, entity_clause));
        },
    }
}

pub(super) fn entity_step_exit(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
    is_nested: bool,

    current: TokenTree, 
    wildcard: EntityWildcard, 
) -> Result<(TokenIter, TokenStream), ()> {
    let result = collect_entity_clause(caravan, current);
    let (mut caravan, entity_clause) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let Some(current) = caravan.next() else {
        return Err(())
    };

    return query_step(current, caravan, package, exit_rule, pre_process, is_nested, (wildcard, entity_clause));
}

fn collect_entity_clause(
    iter: TokenIter, 
    current: TokenTree
) -> Result<(TokenIter, Vec<TokenTree>), ()> {
    let mut entity_clause = Vec::new();
    entity_clause.push(current);
    return collect_until_clause_end(iter, entity_clause)
}

fn collect_until_clause_end(
    iter: TokenIter, 
    output: Vec<TokenTree>
) -> Result<(TokenIter, Vec<TokenTree>), ()> {
    let (result, iter, output) = until_exact_combo(ENTITY_TO_QUERY_PUNCT.to_vec(), iter, output);
    match result {
        ExactComboFound::WasFound => return Ok((iter, output)),
        ExactComboFound::WasNeverFound => return Err(()),
    }
}