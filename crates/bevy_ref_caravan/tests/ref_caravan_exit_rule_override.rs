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
fn exit_rule_return_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); 
    world.spawn(ToOranges(destination)); // Origin
    // 2nd batch
    let destination = world.spawn_empty().id();
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(exit_rule_return_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn exit_rule_return_caravan_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(
            to_oranges :: dest_q = oranges ? return;
        );

        assert!(oranges.0 == 0);
    }

    // Unreachable code, because it returns.
    panic!()
}

#[test]
fn exit_rule_count_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); 
    world.spawn(ToOranges(destination)); // Origin
    // 2nd batch
    let destination = world.spawn_empty().id();
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(exit_rule_count_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn exit_rule_count_caravan_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>,
) {
    let mut fails: u32 = 0;
    for to_oranges in origin_q.iter() {
        ref_caravan!(
            to_oranges :: dest_q = oranges ?{ fails = fails + 1; continue; };
        );

        assert!(oranges.0 == 0);
    }

    assert!(fails == 1);
}