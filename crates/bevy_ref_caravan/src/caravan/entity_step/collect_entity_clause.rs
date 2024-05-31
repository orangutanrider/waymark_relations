use proc_macro::*;
use crate::*;
use crate::syntax_in::*;

/// (caravan, entity clause)
fn collect_entity_clause(caravan: Caravan, current: TokenTree) -> Result<(Caravan, TokenStream), ()>{
    let mut entity_clause = Vec::new();
    entity_clause.push(current);
    return collect_until_clause_end(caravan, entity_clause)
}

fn collect_until_clause_end(mut caravan: Caravan, mut collection: Vec<TokenTree>) -> Result<(Caravan, TokenStream), ()> {
    let token: Option<TokenTree> = caravan.next();
    let Some(token) = token else {
        return Err(());
    };

    match token {
        TokenTree::Group(_) => {
            collection.push(token);
            return collect_until_clause_end(caravan, collection);
        },
        TokenTree::Punct(current) => {
            return end_if_clause_end(caravan, collection, current)
        },
        TokenTree::Ident(_) => {
            collection.push(token);
            return collect_until_clause_end(caravan, collection);
        },
        TokenTree::Literal(_) => {
            collection.push(token);
            return collect_until_clause_end(caravan, collection);
        },
    }
}

fn end_if_clause_end(mut caravan: Caravan, mut collection: Vec<TokenTree>, current: Punct) -> Result<(Caravan, TokenStream), ()> {
    // Is punct 1?
    if current != ENTITY_TO_QUERY_PUNCT_1 {
        collection.push(TokenTree::Punct(current));
        return collect_until_clause_end(caravan, collection) // Invalid, add to string and continue.
    }

    // Check for correct spacing (e.g. joined symbols)
    match current.spacing() {
        Spacing::Joint => (/* proceed */),
        Spacing::Alone => {
            collection.push(TokenTree::Punct(current));
            return collect_until_clause_end(caravan, collection) // Invalid, add to string and continue.
        }, 
    }

    // Go to next symbol
    let Some(current) = caravan.next() else {
        return Err(()) // Expected clause end; Not entire statement end.
    };

    // Is punct? 
    let TokenTree::Punct(current) = current else {
        collection.push(current);
        return collect_until_clause_end(caravan, collection) // Invalid, add to string and continue.
    };

    // Is punct 2?
    if current != ENTITY_TO_QUERY_PUNCT_2 {
        collection.push(TokenTree::Punct(current));
        return collect_until_clause_end(caravan, collection) // Invalid, add to string and continue.
    }

    // Clause end validated/found; End.
    let output = collection.into_iter();
    let output = TokenStream::from_iter(output);
    return Ok((caravan, output))
}