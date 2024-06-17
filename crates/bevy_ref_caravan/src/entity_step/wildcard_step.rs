use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::syntax_in::{DIRECT, LITERAL, DE_REF_LITERAL, OVERLAP, LIFT};

enum EntityWildcard {
    Direct,
    Literal,
    DeRefLiteral,
    Overlap,
    Lifted,
}

pub(super) fn wildcard_step(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    is_nested: bool,

    current: Punct, 
) -> Result<(TokenIter, TokenStream), ()> {
    let Some(kind) = punct_to_wildcard(&current) else {
        return Err(())
    };

    

    todo!()
}

fn punct_to_wildcard(
    punct: &Punct, 
) -> Option<EntityWildcard> {
    match punct.spacing() {
        Spacing::Joint => return None,
        Spacing::Alone => {/* Proceed */},
    }

    match punct.as_char() {
        DIRECT => return Some(EntityWildcard::Direct),
        LITERAL => return Some(EntityWildcard::Literal),
        DE_REF_LITERAL => return Some(EntityWildcard::DeRefLiteral),
        OVERLAP => return Some(EntityWildcard::Overlap),
        LIFT => return Some(EntityWildcard::Lifted),
        _ => return None,
    }
}