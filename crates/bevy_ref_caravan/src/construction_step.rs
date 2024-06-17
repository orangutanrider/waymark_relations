use std::str::FromStr;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    syntax_out::TO_ENTITY_FN,
    entity_step::EntityWildcard,
};

pub(crate) fn construction_step(
    mut package: TokenStream,
    exit_rule: &TokenStream,

    entity_clause: (EntityWildcard, Vec<TokenTree>),
    query_clause: Vec<TokenTree>,
    bindings_clause: Vec<TokenTree>,
    contains_mut: bool,
) -> Result<TokenStream, ()> {
    // Unwrap entity clause
    let (wildcard, entity_clause) = entity_clause;

    // To streams
    let mut entity_clause = TokenStream::from_iter(entity_clause.into_iter());
    let query_clause = TokenStream::from_iter(query_clause.into_iter());
    let bindings_clause = TokenStream::from_iter(bindings_clause.into_iter());

    // Create tokens
    let Ok(let_token) = TokenStream::from_str("let Ok") else {
        return Err(())
    };

    let bindings_clause = Group::new(Delimiter::Parenthesis, bindings_clause);
    let bindings_clause = TokenStream::from_str(&bindings_clause.to_string());
    //let bindings_clause = bindings_clause.stream();

    let Ok(eq_token) = TokenStream::from_str("=") else {
        return Err(())
    };

    let get_token = match contains_mut {
        true => ".get_mut",
        false => ".get",
    };

    let Ok(get_token) = TokenStream::from_str(get_token) else {
        return Err(())
    }; 

    let Ok(entity_go) = TokenStream::from_str(TO_ENTITY_FN) else { // Direct  
        return Err(())
    };
    entity_clause.extend(entity_go);
    let entity_clause = Group::new(Delimiter::Parenthesis, entity_clause);
    let entity_clause = TokenStream::from_str(&entity_clause.to_string());
    //let entity_clause = entity_clause.stream();

    let Ok(else_token) = TokenStream::from_str("else") else {
        return Err(())
    };

    let exit_rule = Group::new(Delimiter::Brace, exit_rule.clone());
    let exit_rule = TokenStream::from_str(&exit_rule.to_string());
    //let exit_rule = exit_rule.stream();

    let Ok(end_token) = TokenStream::from_str(";") else {
        return Err(())
    };

    // Assemble tokens
    let mut assembly = TokenStream::new();
    assembly.extend(let_token);
    assembly.extend(bindings_clause);
    assembly.extend(eq_token);
    assembly.extend(query_clause);
    assembly.extend(get_token);
    assembly.extend(entity_clause);
    assembly.extend(else_token);
    assembly.extend(exit_rule);
    assembly.extend(end_token);

    // Add to package
    package.extend(assembly);

    return Ok(package);
}