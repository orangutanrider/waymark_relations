use proc_macro::*;
use crate::syntax_in::{DIRECT, LITERAL, DE_REF_LITERAL, OVERLAP, LIFT};

pub(crate) enum EntityWildcard {
    DefaultedDirect,
    Direct,
    Literal,
    DeRefLiteral,
    Overlap,
    Lifted,
}

pub(crate) fn wildcard_step(
    current: Punct, 
) -> Result<EntityWildcard, ()> {
    match punct_to_wildcard(&current) {
        Some(kind) => return Ok(kind),
        None => return Err(()),
    }
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