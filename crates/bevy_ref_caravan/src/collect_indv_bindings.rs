use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) fn collect_individual_bindings(bindings_clause: Vec<TokenTree>) -> Result<Vec<Vec<TokenTree>>, ()> {
    let caravan = bindings_clause.into_iter();
    let caravan = TokenStream::from_iter(caravan).into_iter();

    let mut collected = Vec::new();
    match entrance(caravan, &mut collected) {
        Ok(_) => {/* Proceed */},
        Err(_) => return Err(()),
    }

    return Ok(collected)
}

fn entrance(
    mut caravan: TokenIter,
    collected: &mut Vec<Vec<TokenTree>>
) -> Result<(), ()> {
    let Some(token) = caravan.next() else {
        return Ok(())
    };

    match token {
        TokenTree::Group(group) => {
            // Into nested
            let nested_caravan = group.stream().into_iter();
            match entrance(nested_caravan, collected) {
                Ok(_) => {/* Proceed */},
                Err(_) => {return Err(())},
            }

            // Continue across our own scope
            return entrance(caravan, collected)
        },
        TokenTree::Punct(token) => {
            if token == ',' { // If comma error
                return Err(())
            }

            let mut output= Vec::new();
            collect_unchecked(TokenTree::Punct(token), &mut caravan, &mut output);
            collected.push(output);

            return entrance(caravan, collected)
        },
        _ => {
            let mut output= Vec::new();
            collect_unchecked(token, &mut caravan, &mut output);
            collected.push(output);

            return entrance(caravan, collected)
        }
    }

}

/// First token is not checked to see whether it is a ',' or not.
fn collect_unchecked(
    current: TokenTree,
    caravan: &mut TokenIter,
    output: &mut Vec<TokenTree>,
) {    
    // Collect
    output.push(current);

    let Some(current) = caravan.next() else {
        return
    };

    return collect(current, caravan, output);
} 

fn collect(
    current: TokenTree,
    caravan: &mut TokenIter,
    output: &mut Vec<TokenTree>,
) {    
    match current {
        TokenTree::Punct(current) => { // Check for comma and collect
            if current == ',' {
                return
            }

            output.push(TokenTree::Punct(current));

            let Some(current) = caravan.next() else {
                return
            };
            return collect(current, caravan, output);
        },
        _ => { // Collect
            output.push(current);

            let Some(current) = caravan.next() else {
                return
            };
        
            return collect(current, caravan, output);
        },
    }
} 

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proc_macro::*;
    use proc_macro::token_stream::IntoIter as TokenIter;
    use super::*;

    #[test]
    fn simple_tuple() {
        let Ok(input) = TokenStream::from_str("(A, B, C)") else {
            panic!("Unexpected lex error on input creation.")
        };

        let input = {
            let vec: Vec<TokenTree> = input.into_iter().collect();
            vec
        };

        let Ok(indv_bindings) = collect_individual_bindings(input) else {
            panic!("Failed to collect individual bindings, using the function.")
        };

        let mut indv_bindings = indv_bindings.into_iter();

        let Some(mut vec_a) = indv_bindings.next() else {
            panic!("Failed to get the expected token A as a vec from the created individual bindings.")
        };

        let Some(a) = vec_a.pop() else {
            panic!("Failed to get the expected token A as a singular token from the created individual bindings.")
        };

        assert!(vec_a.len() == 0, "The vec for token A, was expected to have no more elements contained within, did have more contained elements.");

        assert!(a.to_string() == "A", "The expected token A was not an 'A' when converted to a string.");
    }
}