use bevy_ref_caravan::*;

#[test]
fn abbreviated_return_exit_rule() {
    assert_ref_caravan!((
        ? r;
        to_oranges :: oranges_q = oranges;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            return
        };
    ));
}