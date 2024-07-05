use bevy_ref_caravan::*;

#[test]
fn pre_processing_test() {
    assert_ref_caravan!((
        $ _dest {
            let $ = Some(%.go()) else {
                continue;
            };
        }

        to_oranges :: oranges_q = oranges;
    ) (
        let to_oranges_dest = Some(to_oranges.go()) else {
            continue;
        };
        let Ok(oranges) = oranges_q.get(to_oranges_dest) else {
            continue;
        };
    ));
}

#[test]
fn dropped_pre_processing_test() {
    assert_ref_caravan!((
        $ _dest {
            let $ = Some(%.go()) else {
                continue;
            };
        }
        to_oranges :: oranges_q = oranges;

        $;
        to_apples :: apples_q = apples;
    ) (
        let to_oranges_dest = Some(to_oranges.go()) else {
            continue;
        };
        let Ok(oranges) = oranges_q.get(to_oranges_dest) else {
            continue;
        };

        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
    ));
}

#[test]
fn wildcard_with_pre_processing() {
    assert_ref_caravan!((
        $ _dest {
            let $ = Some(%.go()) else {
                continue;
            };
        }
        to_oranges :: oranges_q = oranges;

        | to_apples :: apples_q = apples; // The wildcard overrides the pre-processing statement
    ) (
        let to_oranges_dest = Some(to_oranges.go()) else {
            continue;
        };
        let Ok(oranges) = oranges_q.get(to_oranges_dest) else {
            continue;
        };

        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
    ));
}