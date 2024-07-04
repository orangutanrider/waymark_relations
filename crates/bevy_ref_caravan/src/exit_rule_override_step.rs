use crate::*;
use crate::{
    common::collect_until_punct::*, 
    syntax_in::*, 
    construction_step::construction_step, 
    entity_step::entity_step_entrance, 
    exit_rule_step::*, 
    into_next::*, 
    wildcard_step::EntityWildcard
};

enum OverrideNext {
    Next,
    IntoNext,
    Escape,
}

// Exits into construction step
pub(crate) fn exit_rule_override_step(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
    is_nested: bool,

    entity_clause: (EntityWildcard, Vec<TokenTree>),
    query_clause: Vec<TokenTree>,
    bindings_clause: Vec<TokenTree>,
    contains_mut: bool,

    bang_spacing: Spacing,
) -> Result<(TokenIter, TokenStream), ()> {
    let (mut caravan, proto_exit_rule, next) = match collect_until_override_end(caravan, Vec::new(), is_nested) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let override_rule = exit_rule_post_processing(proto_exit_rule, bang_spacing)?;

    match next {
        OverrideNext::Escape => {
            let package = match construction_step(package, &override_rule, entity_clause, query_clause, bindings_clause, contains_mut) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            return Ok((caravan, package))
        },
        OverrideNext::Next => {
            let package = match construction_step(package, &override_rule, entity_clause, query_clause, bindings_clause, contains_mut) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Some(current) = caravan.next() else {
                return Err(())
            };

            return entity_step_entrance(caravan, package, exit_rule, pre_process, is_nested, true, current);
        },
        OverrideNext::IntoNext => {
            let package = match construction_step(package, &override_rule, entity_clause, query_clause, bindings_clause.clone(), contains_mut) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Collect individual binding clauses as a post-processing step on the bindings clause.
            let indv_bindings = match collect_individual_bindings(bindings_clause) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Continue into query steps, feeding in individual bindings, until scope is exhausted.
            return into_next_step_entrance(caravan, package, exit_rule, pre_process, is_nested, indv_bindings.into_iter());
        },
    }
}

// Basically the same thing as collect until bindings end
fn collect_until_override_end(
    mut caravan: TokenIter, 
    mut output: Vec<TokenTree>,
    is_nested: bool,
) -> Result<(TokenIter, Vec<TokenTree>, OverrideNext), ()> {
    let token = caravan.next();
    let Some(token) = token else { // Expect to be un-nested or else throw an error.
        return Ok((caravan, output, OverrideNext::Escape))
    };

    let TokenTree::Punct(token) = token else { // Is Punct?
        output.push(token);
        return collect_until_override_end(caravan, output, is_nested) // If not, continue and add token to output.
    };

    // Is valid singular token?
    match is_nested {
        true => {
            if token == SCOPED_BREAK { // For nested the NEXT symbol is valid.
                return Ok((caravan, output, OverrideNext::Escape))
            }
        },
        false => {
            if token == LINE_BREAK { // For un-nested the LINE_BREAK symbol is valid.
                return Ok((caravan, output, OverrideNext::Escape))
            }
        },
    }

    if token == NEXT_BANG { 
        // match_one_punct_combo ill-suited function, inefficient computation.
        let (results, caravan, output) = match_one_punct_combo(NEXT.iter(), caravan, token, output);
        match results {
            PunctMatch::Matching => return Ok((caravan, output, OverrideNext::Next)),
            _ => {
                return collect_until_override_end(caravan, output, is_nested) // If not, continue. (token is already added to output because of match_one_punct_combo).
            },
        }
    }
    else if token == INTO_BANG { 
        // match_one_punct_combo ill-suited function, inefficient computation.
        let (results, caravan, output) = match_one_punct_combo(INTO_NEXT.iter(), caravan, token, output);
        match results {
            PunctMatch::Matching => return Ok((caravan, output, OverrideNext::IntoNext)),
            _ => {
                return collect_until_override_end(caravan, output, is_nested) // If not, continue. (token is already added to output because of match_one_punct_combo).
            },
        }
    }
    else {
        output.push(TokenTree::Punct(token));
        return collect_until_override_end(caravan, output, is_nested)
    }
}