use proc_macro::*;
use super::*;

fn single_query_step(caravan: Caravan, current: TokenTree, entity_input: TokenStream, exit_rule: &TokenStream) -> Result<Caravan, CaravanError> {

}

fn collect_query_clause(caravan: Caravan, current: TokenTree) -> Result<Caravan,T>