use bevy_caravan::*;

#[test]
fn basic_into_next_ref_caravan() {
    assert_ref_caravan!((
        to_a :: query_a = to_b => query_b = bananas;
    ) (
        let Ok(to_b) = query_a.get(to_a.go()) else {
            continue;
        };
        let Ok(bananas) = query_b.get(to_b.go()) else {
            continue;
        };
    ));
}

#[test]
fn nested_into_next_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples) => {
            oranges_q = oranges,
            apples_q = apples,
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
fn wildcards_into_next_ref_caravan() {
    assert_ref_caravan!((
        @hub :: hub_q = (to_oranges_entity, apples_entity, to_carrots, to_onions) => {
            ^oranges_q = oranges,
            ~apples_q = apples,
            carrots_q = carrots,
            |onions_q = onions,
        }
    ) (
        let Ok((to_oranges_entity, apples_entity, to_carrots, to_onions)) = hub_q.get(hub) else {
            continue;
        };
        let oranges_entity = to_oranges_entity.go();
        let Ok(oranges) = oranges_q.get(oranges_entity) else {
            continue;
        };
        let apples_entity = apples_entity.go();
        let Ok(apples) = apples_q.get(apples_entity) else {
            continue;
        };
        let Ok(carrots) = carrots_q.get(to_carrots.go()) else {
            continue;
        };
        let Ok(onions) = onions_q.get(to_onions.go()) else {
            continue;
        };
    ));
}

#[test]
fn double_nested_into_next_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_vegtables, to_fruits) => {
            vegtables_q = (to_carrots, to_onions) => {
                carrots_q = carrots,
                onions_q = onions,
            },
            fruits_q = (to_oranges, to_apples) => {
                oranges_q = oranges,
                apples_q = apples,
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