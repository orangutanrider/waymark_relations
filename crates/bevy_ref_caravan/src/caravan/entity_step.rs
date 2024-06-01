mod nested_entity_step;
mod single_entity_step;
mod wildcard_step;

use nested_entity_step::*;
use single_entity_step::*;
use wildcard_step::*;

use proc_macro::*;
use super::*;

// (Caravan, Exit_rule)
pub(crate) fn entity_step_entrance(mut caravan: Caravan, exit_rule: Vec<TokenTree>) -> Result<(Caravan, Vec<TokenTree>), ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok((caravan, exit_rule)); // Exit.
    };
    
    match token {
        // Into nested entity step
        TokenTree::Group(group) => {
            let mut nested = match into_nested_entity_step(group, &mut caravan, &exit_rule) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and continue.
            caravan.repack(nested.unpack());
            return entity_step_entrance(caravan, exit_rule);
        },
        // Into single entity step
        TokenTree::Ident(_) => {
            match single_entity_step(caravan, token, EntityBindingKind::Direct) {
                Ok(caravan) => return Ok((caravan, exit_rule)),
                Err(err) => return Err(err),
            }
        },
        // Into wildcard step, entity step following.
        TokenTree::Punct(_) => {
            todo!()
        },
        // Unexpected, throw error.
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}

#[derive(Clone, Copy)]
/// Matched to entity wildcard symbols.
enum EntityBindingKind {
    /// DEFAULT
    /// A waymark binding.
    /// The component is used directly, feeding the entity data into the query; no entity binding is created.
    Direct,
    /// LIFT ^
    /// A waymark binding.
    /// The component is used to create an entity binding, without shadowing the component binding that the entity came from.
    Lifted,
    /// OVERLAP ~
    /// A waymark binding.
    /// The component is used to create an entity binding, that shadows the component binding that the entity came from.
    Overlap,
    /// LITERAL @
    /// A literal entity binding.
    Literal,
}