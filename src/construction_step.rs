use crate::*;

use crate::{
    common::*,
    syntax_out::*, 
    exit_rule_step::*, 
    wildcard_step::EntityWildcard
};

use std::str::FromStr;

pub(crate) fn construction_step(
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,

    entity_clause: (EntityWildcard, Vec<TokenTree>),
    query_clause: Vec<TokenTree>,
    bindings_clause: Vec<TokenTree>,
    contains_mut: bool,
) -> Result<TokenStream, ()> {
    match exit_rule.structure {
        ExitStructure::IsErrMatch => return err_match_construction(package, &exit_rule.statement, pre_process, entity_clause, query_clause, bindings_clause, contains_mut),
        ExitStructure::IsLetElse => return let_else_construction(package, &exit_rule.statement, pre_process, entity_clause, query_clause, bindings_clause, contains_mut),
    }
}

fn err_match_construction(
    mut package: TokenStream,
    exit_rule: &TokenStream,
    pre_process: &Option<EntityPreProcess>,

    entity_clause: (EntityWildcard, Vec<TokenTree>),
    query_clause: Vec<TokenTree>,
    bindings_clause: Vec<TokenTree>,
    contains_mut: bool,
) -> Result<TokenStream, ()> {
    let mut assembly = TokenStream::new();

    // Unwrap entity clause
    let (wildcard, entity_clause) = entity_clause;

    // To streams
    let entity_clause = TokenStream::from_iter(entity_clause.into_iter());
    let query_clause = TokenStream::from_iter(query_clause.into_iter());
    let bindings_clause = TokenStream::from_iter(bindings_clause.into_iter());

    // Handle pre-processing statement
    let (wildcard, entity_clause) = match pre_process { 
        Some(pre_process) => handle_pre_processing(pre_process, entity_clause, &mut assembly, wildcard),
        None => (wildcard, entity_clause),
    };

    // Create tokens
    let Ok(let_token) = TokenStream::from_str("let") else {
        return Err(())
    };

    let Ok(eq_token) = TokenStream::from_str("=") else {
        return Err(())
    };

    let Ok(match_token) = TokenStream::from_str("match") else {
        return Err(())
    };

    let get_token = match contains_mut {
        true => ".get_mut",
        false => ".get",
    };
    let Ok(get_token) = TokenStream::from_str(get_token) else {
        return Err(())
    }; 

    let Ok(ok_token) = TokenStream::from_str("Ok(ok) => ok,") else {
        return Err(())
    };
    let Ok(err_token) = TokenStream::from_str("Err(err) => ") else {
        return Err(())
    };
    let exit_rule = Group::new(Delimiter::Brace, exit_rule.clone());
    let Ok(exit_rule) = TokenStream::from_str(&exit_rule.to_string()) else {
        return Err(())
    };
    let Ok(exit_rule_end_token) = TokenStream::from_str(",") else {
        return Err(())
    };

    let Ok(end_token) = TokenStream::from_str(";") else {
        return Err(())
    };

    // Handle wildcard
    let (entity_binding, entity_clause) = handle_wildcard(entity_clause, wildcard)?;

    // Wrap in delimiters for query.
    let entity_clause = Group::new(Delimiter::Parenthesis, entity_clause);
    let Ok(entity_clause) = TokenStream::from_str(&entity_clause.to_string()) else {
        return Err(())
    };

    // Assemble and wrap match block in delimiters.
    let mut match_block = TokenStream::new();
    match_block.extend(ok_token);
    match_block.extend(err_token);
    match_block.extend(exit_rule);
    match_block.extend(exit_rule_end_token);
    let match_block = Group::new(Delimiter::Brace, match_block);
    let Ok(match_block) = TokenStream::from_str(&match_block.to_string()) else {
        return Err(())
    };

    // Create assembly
    match entity_binding {
        Some(entity_binding) => assembly.extend(entity_binding),
        None => {/* Do Nothing */},
    };

    // Assemble tokens
    assembly.extend(let_token);
    assembly.extend(bindings_clause);
    assembly.extend(eq_token);
    assembly.extend(match_token);
    assembly.extend(query_clause);
    assembly.extend(get_token);
    assembly.extend(entity_clause);
    assembly.extend(match_block);
    assembly.extend(end_token);

    // Add to package
    package.extend(assembly);

    return Ok(package);
}

fn handle_pre_processing(
    pre_process: &EntityPreProcess, 
    entity_clause: TokenStream, 
    assembly: &mut TokenStream, 
    wildcard: EntityWildcard
) -> (EntityWildcard, TokenStream) {
    match wildcard {
        EntityWildcard::DefaultedDirect => {/* Proceed */},
        _ => return (wildcard, entity_clause), // A declared wildcard overrides the pre-processing code
    }

    // Create and add pre-processing statement to assembly
    let pre_processing_code = pre_process.create_pre_processing_code(entity_clause.clone());
    assembly.extend(pre_processing_code);

    // Overide entity_clause with suffixed one from pre processing statement
    let entity_clause = pre_process.transform_entity_clause(entity_clause);

    return (EntityWildcard::Literal, entity_clause)
}

fn let_else_construction(
    mut package: TokenStream,
    exit_rule: &TokenStream,
    pre_process: &Option<EntityPreProcess>,

    entity_clause: (EntityWildcard, Vec<TokenTree>),
    query_clause: Vec<TokenTree>,
    bindings_clause: Vec<TokenTree>,
    contains_mut: bool,
) -> Result<TokenStream, ()> {
    let mut assembly = TokenStream::new();

    // Unwrap entity clause
    let (wildcard, entity_clause) = entity_clause;

    // To streams
    let entity_clause = TokenStream::from_iter(entity_clause.into_iter());
    let query_clause = TokenStream::from_iter(query_clause.into_iter());
    let bindings_clause = TokenStream::from_iter(bindings_clause.into_iter());

    // Handle pre-processing statement
    let (wildcard, entity_clause) = match pre_process { 
        Some(pre_process) => handle_pre_processing(pre_process, entity_clause, &mut assembly, wildcard),
        None => (wildcard, entity_clause),
    };

    // Create tokens
    let Ok(let_token) = TokenStream::from_str("let Ok") else {
        return Err(())
    };

    let bindings_clause = Group::new(Delimiter::Parenthesis, bindings_clause);
    let Ok(bindings_clause) = TokenStream::from_str(&bindings_clause.to_string()) else {
        return Err(())
    };

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

    let Ok(else_token) = TokenStream::from_str("else") else {
        return Err(())
    };

    let exit_rule = Group::new(Delimiter::Brace, exit_rule.clone());
    let Ok(exit_rule) = TokenStream::from_str(&exit_rule.to_string()) else {
        return Err(())
    };

    let Ok(end_token) = TokenStream::from_str(";") else {
        return Err(())
    };

    // Handle wildcard
    let (entity_binding, entity_clause) = handle_wildcard(entity_clause, wildcard)?;
    
    // Wrap in delimiters for query.
    let entity_clause = Group::new(Delimiter::Parenthesis, entity_clause);
    let Ok(entity_clause) = TokenStream::from_str(&entity_clause.to_string()) else {
        return Err(())
    };

    // Create assembly
    match entity_binding {
        Some(entity_binding) => assembly.extend(entity_binding),
        None => {/* Do Nothing */},
    };

    // Assemble tokens
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

fn handle_wildcard(
    mut entity_clause: TokenStream,
    wildcard: EntityWildcard,
) -> Result<(Option<TokenStream>, TokenStream), ()> {
    match wildcard {
        EntityWildcard::DefaultedDirect => {
            let Ok(entity_go) = TokenStream::from_str(TO_ENTITY_FN) else { 
                return Err(())
            };
            entity_clause.extend(entity_go);

            return Ok((None, entity_clause))
        }
        EntityWildcard::Direct => {
            let Ok(entity_go) = TokenStream::from_str(TO_ENTITY_FN) else { 
                return Err(())
            };
            entity_clause.extend(entity_go);

            return Ok((None, entity_clause))
        },
        EntityWildcard::Literal => {
            return Ok((None, entity_clause))
        },
        EntityWildcard::DeRefLiteral => {
            let entity_binding = create_de_ref_literal_binding(entity_clause.clone());

            return Ok((Some(entity_binding), entity_clause))
        },
        EntityWildcard::Overlap => {
            let Ok(entity_binding) = create_overlap_entity_binding(entity_clause.clone()) else {
                return Err(())
            };

            return Ok((Some(entity_binding), entity_clause))
        },
        EntityWildcard::Lifted => {
            // Created lifted clause
            let lifted_clause = match create_lifted_entity_clause(entity_clause.clone()) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Ok(entity_binding) = create_lifted_entity_binding(lifted_clause.clone(), entity_clause) else {
                return Err(())
            };

            return Ok((Some(entity_binding), lifted_clause))
        },
    };
}

fn create_de_ref_literal_binding(
    entity_clause: TokenStream,
) -> TokenStream {
    // Create tokens
    let Ok(let_token) = TokenStream::from_str("let") else {
        panic!("Unexpected lex error in create_de_ref_literal_binding.")
    };

    let Ok(eq_token) = TokenStream::from_str("=") else {
        panic!("Unexpected lex error in create_de_ref_literal_binding.")
    };

    let Ok(de_ref_token) = TokenStream::from_str("*") else {
        panic!("Unexpected lex error in create_de_ref_literal_binding.")
    };

    let entity_data = entity_clause.clone();

    let Ok(end_token) = TokenStream::from_str(";") else {
        panic!("Unexpected lex error in create_de_ref_literal_binding.")
    };

    // Assemble tokens
    let mut assembly = TokenStream::new();
    assembly.extend(let_token);
    assembly.extend(entity_clause);
    assembly.extend(eq_token);
    assembly.extend(de_ref_token);
    assembly.extend(entity_data);
    assembly.extend(end_token);

    return assembly
}

fn create_overlap_entity_binding(
    entity_clause: TokenStream,
) -> Result<TokenStream, ()> {
    // Create tokens
    let Ok(let_token) = TokenStream::from_str("let") else {
        return Err(())
    };

    let Ok(entity_go) = TokenStream::from_str(TO_ENTITY_FN) else { 
        return Err(())
    };
    let mut entity_data = entity_clause.clone();
    entity_data.extend(entity_go);

    let Ok(eq_token) = TokenStream::from_str("=") else {
        return Err(())
    };

    let Ok(end_token) = TokenStream::from_str(";") else {
        return Err(())
    };

    // Assemble tokens
    let mut assembly = TokenStream::new();
    assembly.extend(let_token);
    assembly.extend(entity_clause);
    assembly.extend(eq_token);
    assembly.extend(entity_data);
    assembly.extend(end_token);
    
    return Ok(assembly);
}

fn create_lifted_entity_binding(
    lifted_clause: TokenStream,
    entity_clause: TokenStream,
) -> Result<TokenStream, ()> {
    // Create tokens
    let Ok(let_token) = TokenStream::from_str("let") else {
        return Err(())
    };

    let Ok(entity_go) = TokenStream::from_str(TO_ENTITY_FN) else { 
        return Err(())
    };
    let mut entity_data = entity_clause.clone();
    entity_data.extend(entity_go);

    let Ok(eq_token) = TokenStream::from_str("=") else {
        return Err(())
    };

    let Ok(end_token) = TokenStream::from_str(";") else {
        return Err(())
    };

    // Assemble tokens
    let mut assembly = TokenStream::new();
    assembly.extend(let_token);
    assembly.extend(lifted_clause);
    assembly.extend(eq_token);
    assembly.extend(entity_data);
    assembly.extend(end_token);
    
    return Ok(assembly);
}

fn create_lifted_entity_clause(
    entity_clause: TokenStream,
) -> Result<TokenStream, ()> {
    // Iterate until the first ident is found, apply lift edits to that part of the token stream, reconstruct token stream and return.

    let entity_clause = entity_clause.into_iter();
    let (iter, processed, ident) = until_ident(entity_clause); // Get ident.

    let Some(ident) = ident else { // No ident was found; an invalid entity clause.
        return Err(())
    };

    // Apply lift edits
    let ident = ident.to_string();
    let lifted = ident.as_str();
    let lifted = { 
        match ident.strip_prefix(LIFT_PREFIX_REMOVE) {
            Some(prefix_removed) => prefix_removed, // Prefix removed.
            None => &(lifted.to_owned() + LIFT_SUFFIX_ADD) // If no prefix, add suffix,
        }
    };

    // Lifted to token stream.
    let Ok(lifted) = TokenStream::from_str(lifted) else {
        return Err(())
    };

    // Reconstruct complete entity_clause token stream.
    let mut entity_clause = TokenStream::from_iter(processed.into_iter()); // Processed as token stream.
    entity_clause.extend(lifted); // Add lifted ident.
    entity_clause.extend(iter); // Add un-processed tokens.

    return Ok(entity_clause)
}