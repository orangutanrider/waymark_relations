use bevy_ref_caravan::*;

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