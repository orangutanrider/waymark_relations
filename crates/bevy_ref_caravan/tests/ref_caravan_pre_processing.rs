use bevy_ref_caravan::*;

#[test]
fn pre_processing_test() {
    assert_ref_caravan!((
        $ _peeled {
            let $ = Some(%) else {
                continue;
            };
        }

        oranges :: oranges_q = oranges;
    ) (
        let oranges_peeled = Some(oranges) else {
            continue;
        };
        let Ok(oranges) = oranges_q.get(oranges_peeled) else {
            continue;
        };
    ));
}