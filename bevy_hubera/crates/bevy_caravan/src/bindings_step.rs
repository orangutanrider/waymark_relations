use crate::*;

use crate::{
    common::{collect_until_punct::*, *}, 
    syntax_in::*, 
    construction_step::construction_step, 
    entity_step::*, 
    query_step::QueryMutation, 
    wildcard_step::EntityWildcard,
    exit_rule_override_step::exit_rule_override_step, 
    into_next::*, 
};

enum BindingsNext {
    ExitRuleOverride(Spacing),
    Next,
    IntoNext,
    Escape,
}

pub(crate) fn bindings_step(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
    is_nested: bool,

    entity_clause: (EntityWildcard, Vec<TokenTree>), 
    query_clause: (Vec<TokenTree>, QueryMutation),
) -> Result<(TokenIter, TokenStream), ()> {
    // Collect
    let (mut caravan, bindings_clause, next) = match collect_until_bindings_end(caravan, Vec::new(), is_nested) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    
    // Check for raw input
    let Some(token0) = bindings_clause.get(0) else {
        return Err(())
    };
    let token1 = bindings_clause.get(1);

    let raw_bindings = match token0 {
        TokenTree::Group(group) => { match group.delimiter() {
            RAW_INPUT_DELIMITER => {
                match token1 {
                    Some(_) => None,
                    None => Some(group.stream()),
                }
            },
            _ => None,
        }},
        _ => None,
    };
    
    // Unwrap clauses and check for mutation for non-raw inputs
    let (query_clause, contains_mut, bindings_clause) = match raw_bindings {
        Some(raw_bindings) => {
            let contains_mut = match query_clause.1 {
                QueryMutation::GetMut => true,
                QueryMutation::Get => false,
            };
            (query_clause.0, contains_mut, raw_bindings.into_iter().collect())
        },
        None => {
            match query_clause.1 {
                QueryMutation::GetMut => (query_clause.0, true, bindings_clause),
                QueryMutation::Get => {
                    let mut_iter = bindings_clause.iter();
                    let contains_mut = contains_mut_recursive(mut_iter);
                    (query_clause.0, contains_mut, bindings_clause)
                },
            }
        },
    };

    // Exit
    match next {
        BindingsNext::ExitRuleOverride(spacing) => return exit_rule_override_step(caravan, package, exit_rule, pre_process, is_nested, entity_clause, query_clause, bindings_clause, contains_mut, spacing),
        BindingsNext::Escape => {
            let package = match construction_step(package, exit_rule, pre_process, entity_clause, query_clause, bindings_clause, contains_mut) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            return Ok((caravan, package));
        },
        BindingsNext::Next => {
            let package = match construction_step(package, exit_rule, pre_process, entity_clause, query_clause, bindings_clause, contains_mut) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Some(current) = caravan.next() else {
                return Err(())
            };

            return entity_step_entrance(caravan, package, exit_rule, pre_process, is_nested, true, current);
        },
        BindingsNext::IntoNext => {
            let package = match construction_step(package, exit_rule, pre_process, entity_clause, query_clause, bindings_clause.clone(), contains_mut) {
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

fn collect_until_bindings_end(
    mut caravan: TokenIter,
    mut output: Vec<TokenTree>,
    is_nested: bool,
) -> Result<(TokenIter, Vec<TokenTree>, BindingsNext), ()> {
    let token = caravan.next();
    let Some(token) = token else { // Expect to be un-nested or else throw an error.
        return Ok((caravan, output, BindingsNext::Escape))
    };

    let TokenTree::Punct(token) = token else { // Is Punct?
        output.push(token);
        return collect_until_bindings_end(caravan, output, is_nested) // If not, continue and add token to output.
    };

    if token == EXIT_RULE_NOTATION {
        // Into override
        return Ok((caravan, output, BindingsNext::ExitRuleOverride(token.spacing())))
    }

    // Is valid singular token?
    match is_nested {
        true => {
            if token == SCOPED_BREAK { // For nested the NEXT symbol is valid.
                return Ok((caravan, output, BindingsNext::Escape))
            }
        },
        false => {
            if token == LINE_BREAK { // For un-nested the LINE_BREAK symbol is valid.
                return Ok((caravan, output, BindingsNext::Escape))
            }
        },
    }

    if token == NEXT_BANG { 
        // match_one_punct_combo ill-suited function, inefficient computation.
        let (results, caravan, output) = match_one_punct_combo(NEXT.iter(), caravan, token, output);
        match results {
            PunctMatch::Matching => return Ok((caravan, output, BindingsNext::Next)),
            _ => {
                return collect_until_bindings_end(caravan, output, is_nested) // If not, continue. (token is already added to output because of match_one_punct_combo).
            },
        }
    }
    else if token == INTO_BANG { 
        // match_one_punct_combo ill-suited function, inefficient computation.
        let (results, caravan, output) = match_one_punct_combo(INTO_NEXT.iter(), caravan, token, output);
        match results {
            PunctMatch::Matching => return Ok((caravan, output, BindingsNext::IntoNext)),
            _ => {
                return collect_until_bindings_end(caravan, output, is_nested) // If not, continue. (token is already added to output because of match_one_punct_combo).
            },
        }
    }
    else {
        output.push(TokenTree::Punct(token));
        return collect_until_bindings_end(caravan, output, is_nested)
    }
}