use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::syntax_in::{ENTITY_PRE_PROCESS_NOTATION, ENTITY_PRE_PROCESS_VAR};

pub(crate) struct EntityPreProcess {
    pub(crate) farm: TokenStream,
    pub(crate) suffix: Ident,
} 
impl EntityPreProcess {
    pub(crate) fn create_pre_processing_code(&self, entity_clause: &Vec<TokenTree>) -> TokenStream {
        // PSEUDOCODE
        // Iterate across farm and construct a seperate and new one.
        // When notation is found, create suffix-ed entity clause.
        // When var notation is found, insert entity clause directly.
        // Return this altered copy of farm.

        let iter = self.farm.clone().into_iter();

        todo!()
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
            if punct == ENTITY_PRE_PROCESS_NOTATION {
                let token_push = Group::new(Delimiter::None, suffixed_entity_clause.clone());
                output.push(TokenTree::Group(token_push));
                return build_pre_processing_code(iter, entity_clause, suffixed_entity_clause, output);
            }

            if punct == ENTITY_PRE_PROCESS_VAR {
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