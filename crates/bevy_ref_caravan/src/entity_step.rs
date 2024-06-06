mod single_entity_step;
//mod wildcard_step;
//mod nested_entity_step;

use single_entity_step::*;
//use wildcard_step::*;
//use nested_entity_step::*;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) fn entity_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
) -> Result<(TokenIter, TokenStream), ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok((caravan, package)); // Exit.
    };

    // To single entity step, remove when additional features are added.
    match single_entity_step(caravan, package, exit_rule, token) {
        Ok(caravan) => return Ok(caravan),
        Err(err) => return Err(err),
    }
    
    /* 
    match token {
        // Into nested entity step
        TokenTree::Group(group) => {
            let mut nested = match into_nested_entity_step(group, &mut caravan, exit_rule) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and continue.
            caravan.repack(nested.unpack());
            return entity_step_entrance(caravan, exit_rule);
        },
        // Into single entity step
        TokenTree::Ident(_) => {
            match single_entity_step(caravan, token, EntityBindingKind::Direct, exit_rule) {
                Ok(caravan) => return Ok(caravan),
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
    */
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