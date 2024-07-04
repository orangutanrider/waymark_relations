use core::panic;
use std::str::FromStr;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::syntax_in::{ENTITY_PRE_PROCESS_NOTATION, ENTITY_PRE_PROCESS_VAR, LINE_BREAK};

/// The step that reads entity pre-processing statements, storing them.
pub(crate) fn entity_pre_process_decleration_step(
    mut caravan: TokenIter, 
) -> Result<(TokenIter, Option<EntityPreProcess>), ()> {
    // If nothing wipe the pre-process statement.
    // Expect suffix or brace'd group
    // Expect brace'd group
    // Linebreak doesn't need to be validated or expected.

    let Some(token) = caravan.next() else {
        return Ok((caravan, None))
    };

    let suffix = match token {
        TokenTree::Group(group) => { // No suffix, pre_processing will be shadowed by following statement.
            if group.delimiter() != Delimiter::Brace {
                return Err(());
            }

            let pre_process = EntityPreProcess::new(group.stream(), None);
            return Ok((caravan, Some(pre_process)));
        },
        TokenTree::Ident(ident) => { // Suffix declared
            ident.to_string()
        },
        TokenTree::Punct(punct) => {
            if punct != LINE_BREAK {
                return Err(())
            }

            return Ok((caravan, None))
        }
        _ => return Err(()), // Unexpected
    };

    let Some(token) = caravan.next() else { // Expect group
        return Err(())
    };

    match token {
        TokenTree::Group(group) => {
            if group.delimiter() != Delimiter::Brace {
                return Err(());
            }

            let pre_process = EntityPreProcess::new(group.stream(), Some(suffix.to_string()));
            return Ok((caravan, Some(pre_process)));
        },
        _ => return Err(()), // Unexpected
    }
}

pub(crate) struct EntityPreProcess {
    pub(crate) farm: TokenStream, // The origin of constructed pre-processing statements; When one is created the farm is read to do so.
    pub(crate) suffix: Option<String>,
} 
impl EntityPreProcess {
    pub(crate) fn transform_entity_clause(&self, entity_clause: TokenStream) -> TokenStream { // The entity clause itself needs to be changed to the one created during pre-processing.
        let suffixed_entity_clause = match &self.suffix {
            Some(suffix) => entity_clause.to_string() + suffix,
            None => entity_clause.to_string(),
        };
        let Ok(suffixed_entity_clause) = TokenStream::from_str(&suffixed_entity_clause) else {
            panic!("Unexpected lex error, while creating a token stream, from a suffixed entity clause, when creating a transformed entity clause.")
        };

        return suffixed_entity_clause
    }

    pub(crate) fn create_pre_processing_code(&self, entity_clause: TokenStream) -> TokenStream {
        // PSEUDOCODE
        // Iterate across farm and construct a seperate and new one.
        // When notation is found, create suffix-ed entity clause.
        // When var notation is found, insert entity clause directly.
        // Return this altered copy of farm.

        let iter = self.farm.clone().into_iter();
        let suffixed_entity_clause = match &self.suffix {
            Some(suffix) => entity_clause.to_string() + suffix,
            None => entity_clause.to_string(),
        };
        let Ok(suffixed_entity_clause) = TokenStream::from_str(&suffixed_entity_clause) else {
            panic!("Unexpected lex error, while creating a token stream, from a suffixed entity clause, during the construction of entity pre-processing code.")
        };

        let mut output = Vec::new(); 
        build_pre_processing_code(iter, &entity_clause, &suffixed_entity_clause, &mut output);
        let output = TokenStream::from_iter(output.into_iter());

        return output;
    }

    pub(crate) fn new(
        statement: TokenStream,
        suffix: Option<String>,
    ) -> Self {
        return Self {
            farm: statement,
            suffix,
        }
    }
}

fn build_pre_processing_code(
    mut iter: TokenIter,
    entity_clause: &TokenStream,
    suffixed_entity_clause: &TokenStream,
    output: &mut Vec<TokenTree>,
) {
    let Some(token) = iter.next() else {
        return;
    };
    
    match token {
        TokenTree::Group(group) => {
            let nested_iter = group.stream().into_iter();
            let mut nested = Vec::new();
            build_pre_processing_code(nested_iter, entity_clause, suffixed_entity_clause, &mut nested); // Go into nested group
            let nested = TokenStream::from_iter(nested.into_iter());
            let nested = Group::new(group.delimiter(), nested); // Create group output as singular token

            output.push(TokenTree::Group(nested)); // Add group to upper level output
            return build_pre_processing_code(iter, entity_clause, suffixed_entity_clause, output); // Continue
        },
        TokenTree::Punct(punct) => { // Check for relevant symbols and insert relevant data into output when found
            if punct == ENTITY_PRE_PROCESS_NOTATION { // Add suffixed entity clause
                let token_push = Group::new(Delimiter::None, suffixed_entity_clause.clone());
                output.push(TokenTree::Group(token_push));
                return build_pre_processing_code(iter, entity_clause, suffixed_entity_clause, output);
            }

            if punct == ENTITY_PRE_PROCESS_VAR { // Add entity clause
                let token_push = Group::new(Delimiter::None, entity_clause.clone());
                output.push(TokenTree::Group(token_push));
                return build_pre_processing_code(iter, entity_clause, suffixed_entity_clause, output);
            }

            output.push(TokenTree::Punct(punct)); // If token is misc, continue.
            return build_pre_processing_code(iter, entity_clause, suffixed_entity_clause, output);
        },
        _ => { // Continue
            output.push(token);
            return build_pre_processing_code(iter, entity_clause, suffixed_entity_clause, output);
        }
    }
}