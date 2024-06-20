fn main() { }

/* 

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

fn incorrect_delimiter() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); 
    world.spawn(ToOranges(destination)); // Origin
    // 2nd batch
    let destination = world.spawn_empty().id();
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(incorrect_delimiter_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn incorrect_delimiter_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>,
) {
    let mut fails: u32 = 0;
    for to_oranges in origin_q.iter() {
        ref_caravan!(
            ? (
                fails = fails + 1;
                continue;
            );
            to_oranges :: dest_q = oranges;
        );

        assert!(oranges.0 == 0);
    }

    assert!(fails == 1);
}

*/