use proc_macro::*;
use crate::*;
use crate::syntax_in::*;
use crate::common::*;
use super::*;

pub(super) fn single_entity_step(caravan: Caravan, current: TokenTree, wildcard: EntityBindingKind) -> Result<Caravan, ()> {
    let result = collect_entity_clause(caravan, current);
    let (mut caravan, mut entity_clause) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    // Into query step.
    match wildcard {
        EntityBindingKind::Direct => {
            todo!()
        },
        EntityBindingKind::Lifted => {
            todo!()
        },
        EntityBindingKind::Overlap => {
            todo!()
        },
        EntityBindingKind::Literal => {
            todo!()
        },
    }
}

/// (Caravan, Entity clause)
fn collect_entity_clause(
    caravan: Caravan, 
    current: TokenTree
) -> Result<(Caravan, Vec<TokenTree>), ()> {
    let mut entity_clause = Vec::new();
    entity_clause.push(current);
    return collect_until_clause_end(caravan, entity_clause)
}

fn collect_until_clause_end(
    caravan: Caravan, 
    mut collection: Vec<TokenTree>
) -> Result<(Caravan, Vec<TokenTree>), ()> {
    match collect_until_punct_combo(ENTITY_TO_QUERY_PUNCT.to_vec(), caravan.iter) {
        ComboFound::WasFound((iter, mut gathered)) => {
            let caravan = Caravan::new(iter, caravan.package, caravan.depth);
            collection.append(&mut gathered);
            return Ok((caravan, collection))
        },
        ComboFound::WasNeverFound(_) => return Err(()),
    }
}