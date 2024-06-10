use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
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

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct TestSchedule;

#[test]
fn ref_caravan_basics() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); // No oranges
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(one_orange_sys);
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system

    // Get edited entity, and assert
    let oranges = world.entity(destination).get::<Oranges>().unwrap();
    assert!(oranges.0 == 1);
}

fn one_orange_sys(
    origin_q: Query<&ToOranges>,
    mut dest_q: Query<&mut Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(to_oranges :: dest_q = mut oranges;);

        oranges.0 = 1;
    }
}