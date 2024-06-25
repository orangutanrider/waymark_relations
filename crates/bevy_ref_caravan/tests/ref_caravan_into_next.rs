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