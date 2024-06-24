use bevy_ref_caravan::ref_caravan;

#[derive(Clone, Copy)]
struct Entity(u32);

struct Query(Oranges);
impl Query {
    fn get(&self, _entity: Entity) -> Result<Oranges, ()> {
        return Ok(self.0)   
    }
}

#[derive(Clone, Copy)]
struct Oranges(u32);

#[test]
fn de_ref_literal_ref_caravan() {
    let entity = &Entity(0);
    let oranges_q = Query(Oranges(0));

    loop {
        ref_caravan!(*entity :: oranges_q = oranges);

        assert!(oranges.0 == 0);

        let mut entity = entity;
        entity.0 = 1;
        assert!(entity.0 == 1);
        
        break;
    }
}