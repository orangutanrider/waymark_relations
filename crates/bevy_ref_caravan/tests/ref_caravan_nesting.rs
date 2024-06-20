use bevy_ref_caravan::*;

// Expected valid statements.
#[test]
fn nested_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples) => {
            to_oranges :: oranges_q = oranges,
            to_apples :: apples_q = apples,
        }
    ) (
        let Ok((to_oranges, to_apples)) = hub_q.get(to_hub.go()) else {
            continue;
        };
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
    ));
}

#[test]
fn mut_nested_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples) => {
            to_oranges :: oranges_q = mut oranges,
            to_apples :: apples_q = apples,
        }
    ) (
        let Ok((to_oranges, to_apples)) = hub_q.get(to_hub.go()) else {
            continue;
        };
        let Ok(mut oranges) = oranges_q.get_mut(to_oranges.go()) else {
            continue;
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
        
    ));
}

#[test]
fn double_nested_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_vegtables, to_fruits) => {
            to_vegtables :: vegtables_q = (to_carrots, to_onions) => {
                to_carrots :: carrots_q = carrots,
                to_onions :: onions_q = onions,
            },
            to_fruits :: fruits_q = (to_oranges, to_apples) => {
                to_oranges :: oranges_q = oranges,
                to_apples :: apples_q = apples,
            },
        }
    ) (
        let Ok((to_vegtables, to_fruits)) = hub_q.get(to_hub.go()) else {
            continue;
        };
        let Ok((to_carrots, to_onions)) = vegtables_q.get(to_vegtables.go()) else {
            continue;
        };
        let Ok(carrots) = carrots_q.get(to_carrots.go()) else {
            continue;
        };
        let Ok(onions) = onions_q.get(to_onions.go()) else {
            continue;
        };
        let Ok((to_oranges, to_apples)) = fruits_q.get(to_fruits.go()) else {
            continue;
        };
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
    ));
}

// Expected invalid statements.
#[test]
fn semi_colon_into_scope() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples); {
            to_oranges :: oranges_q = oranges,
            to_apples :: apples_q = apples,
        }
    ) (
        compile_error!("Undefined")
    ));
}

#[test]
fn nested_comma_into_scope() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_vegtables, to_fruits) => {
            to_vegtables :: vegtables_q = (to_carrots, to_onions), { 
                to_carrots :: carrots_q = carrots,
                to_onions :: onions_q = onions,
            },
            to_fruits :: fruits_q = (to_oranges, to_apples), {
                to_oranges :: oranges_q = oranges,
                to_apples :: apples_q = apples,
            },
        }
    ) (
        compile_error!("Undefined")
    ));
}

#[test]
fn immediate_nested_scope() {
    assert_ref_caravan!((
        {
            to_hub :: hub_q = (to_oranges, to_apples) => {
                to_oranges :: oranges_q = oranges,
                to_apples :: apples_q = apples,
            }
        }
    ) (
        compile_error!("Undefined")
    )); 
}

#[test]
fn diamond_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples, to_carrots) => {
            to_oranges :: oranges_q = oranges,
            to_apples :: apples_q = apples,
            to_carrots :: carrots_q = to_onions,
        } => to_onions :: onions_q = onions;
    ) (
        compile_error!("Undefined")
    )); 
}