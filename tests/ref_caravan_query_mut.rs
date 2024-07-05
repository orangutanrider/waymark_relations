use bevy_ref_caravan::*;

#[test]
fn query_step_mutation_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: mut oranges_q = oranges;
    ) (
        let Ok(oranges) = oranges_q.get_mut(to_oranges.go()) else {
            continue;
        };
    ));
}