use bevy_ecs::prelude::*;
use bevy_ref_caravan::ref_caravan;

#[derive(Component)]
struct ToOranges(Entity);
impl ToOranges {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct Oranges(u32);

#[test]
fn ref_caravan_basics() {
    let mut world =  World::new();
    
    let destination = world.spawn(Oranges(3)).id();
    world.spawn(ToOranges(destination)); // Origin


}

fn to_oranges_sys(
    origin_q: Query<&ToOranges>,
    mut dest_q: Query<&mut Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(to_oranges :: dest_q = mut oranges;);

    }
}