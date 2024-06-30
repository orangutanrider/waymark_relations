use bevy_ref_caravan::*;

#[test]
fn err_match_ref_caravan() {
    assert_ref_caravan!((
        ?? return err;
        to_oranges :: oranges_q = oranges;
    ) (
        let oranges = match oranges_q.get(to_oranges.go()) {
            Ok(ok) => ok,
            Err(err) => { return err },
        };
    ));
}

#[test]
fn block_err_match_ref_caravan() {
    assert_ref_caravan!((
        ?? {
            let err = WAYMARK_ERR;
            return err;
        };
        to_oranges :: oranges_q = oranges;
    ) (
        let oranges = match oranges_q.get(to_oranges.go()) {
            Ok(ok) => ok,
            Err(err) => { 
                let err = WAYMARK_ERR;
                return err;
            },
        };
    ));
}

#[test]
fn override_err_match_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges ?? return err;
    ) (
        let oranges = match oranges_q.get(to_oranges.go()) {
            Ok(ok) => ok,
            Err(err) => { return err },
        };
    ));
}