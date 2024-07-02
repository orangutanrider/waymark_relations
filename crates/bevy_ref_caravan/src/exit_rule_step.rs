use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::common::collect_until_punct::*;
use crate::syntax_in::{ABBREVIATED_RETURN, ERR_MATCH_NOTATION, EXIT_RULE_DELIMITER, LINE_BREAK};
use crate::syntax_out::exit_rule_default;

pub(crate) struct ExitRule {
    pub(crate) statement: TokenStream,
    pub(crate) structure: ExitStructure
}
impl Default for ExitRule {
    fn default() -> Self {
        return Self { statement: exit_rule_default(), structure: ExitStructure::IsLetElse }
    }
}
impl ExitRule {
    pub(crate) fn wipe(&mut self) {
        self.statement = TokenStream::new();
        self.structure = ExitStructure::IsLetElse;
    }
}

pub(crate) fn exit_rule_step(
    mut caravan: TokenIter, 
    exit_rule: &mut ExitRule,
    bang_spacing: Spacing,
) -> Result<TokenIter, ()> {
    let entrance = match exit_rule_pre_processing_step(&mut caravan, bang_spacing) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let (caravan, collected_exit_rule, structure) = match entrance {
        ExitRuleEntrance::GroupCollected(group, structure) => {
            match expect_empty_or_line_break(&mut caravan) {
                Ok(_) => { },
                Err(err) => return Err(err),
            }

            (caravan, group, structure)
        },
        ExitRuleEntrance::InLineCollected(token, structure) => {
            let mut output = Vec::new();
            output.push(token);

            // Ill-suited function, the information of whether or not the punct was ever found is not needed.
            let (_, caravan, mut collected_exit_rule) = collect_until_matching_punct(LINE_BREAK, caravan, output); 
            exit_rule_collection_post_processing_step(&mut collected_exit_rule);

            let collected_exit_rule = TokenStream::from_iter(collected_exit_rule.into_iter());
            (caravan, collected_exit_rule, structure)
        },
    };

    exit_rule.structure = structure;
    exit_rule.statement.extend(collected_exit_rule);

    return Ok(caravan)
}

fn expect_empty_or_line_break(caravan: &mut TokenIter) -> Result<(), ()> {
    let Some(token) = caravan.next() else {
        return Ok(())
    };

    let TokenTree::Punct(token) = token else {
        return Err(())
    };

    if token != LINE_BREAK {
        return Err(())
    }

    Ok(())
}

pub(crate) fn exit_rule_collection_post_processing_step(
    collected_exit_rule: &mut Vec<TokenTree>,
) {
    match collected_exit_rule.get(0) { // If token 0 is just an "r" then it will be re-created as a "return"
        Some(exit_rule_0) => {
            if exit_rule_0.to_string() == ABBREVIATED_RETURN {
                collected_exit_rule[0] = TokenTree::Ident(Ident::new("return", exit_rule_0.span()));
            }
        },
        None => {/* Do nothing */},
    };
}

pub(crate) enum ExitRuleEntrance {
    GroupCollected(TokenStream, ExitStructure),
    InLineCollected(TokenTree, ExitStructure),
    Nothing(ExitStructure), // Given an empty decleration, it'll use default implementations for either structure.
}

pub(crate) enum ExitStructure{
    IsErrMatch,
    IsLetElse
}

/// Expectedly followed by a collection step, unless it returned a group.
/// The in_line_current token should be added to the collection as the first token (not relevant if this returns a group).
pub(crate) fn exit_rule_pre_processing_step(
    caravan: &mut TokenIter, 
    bang_spacing: Spacing,
) -> Result<ExitRuleEntrance, ()> {
    match bang_spacing {
        Spacing::Joint => return exit_rule_pre_processing(caravan, true, ExitStructure::IsLetElse),
        Spacing::Alone => return exit_rule_pre_processing(caravan, false, ExitStructure::IsLetElse),
    }
}

fn exit_rule_pre_processing(
    caravan: &mut TokenIter, 
    valid_first: bool,
    structure: ExitStructure,
) -> Result<ExitRuleEntrance, ()> {
    let Some(token) = caravan.next() else {
        return Err(())
    };

    match token {
        TokenTree::Group(group) => {
            // Validate delimiter
            if group.delimiter() != EXIT_RULE_DELIMITER {
                return Err(())
            }

            // Return
            return Ok(ExitRuleEntrance::GroupCollected(group.stream(), structure))
        },
        TokenTree::Punct(_) => {
            if token.to_string() != ERR_MATCH_NOTATION.to_string() {
                return Ok(ExitRuleEntrance::InLineCollected(token, structure))
            }

            // If it does equal the ERR_MATCH_NOTATION, and it has been validated that this is a combo'd first token input; 
            // Then '??' has been confirmed, and the function signals to create a match statement. Otherwise it exits, giving the token as an in_line current to be a part of an in_line collection.
            match valid_first {
                true =>  return exit_rule_pre_processing(caravan, false, ExitStructure::IsErrMatch),
                false => return Ok(ExitRuleEntrance::InLineCollected(token, structure)),
            }
        },
        _ => {
            return Ok(ExitRuleEntrance::InLineCollected(token, structure))
        },
    }
}