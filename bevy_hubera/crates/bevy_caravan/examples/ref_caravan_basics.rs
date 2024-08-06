fn main() { }

/* 

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use bevy_ref_caravan::ref_caravan;

#[derive(Component)]
struct Origin;

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

fn default_exit_rule_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn_empty().id(); // Nothing at all
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(assert_no_iters_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn assert_no_iters_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>, // Will result in nothing
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(to_oranges :: dest_q = oranges); 
        panic!();
    }
}

fn multi_carvan_breaks() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); // No oranges
    let middle = world.spawn(ToOranges(destination)).id();
    world.spawn((ToOranges(middle), Origin)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(multi_carvan_breaks_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn multi_carvan_breaks_sys(
    origin_q: Query<&ToOranges, With<Origin>>,
    to_oranges_q: Query<&ToOranges, Without<Origin>>,
    dest_q: Query<&Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(
            to_oranges :: to_oranges_q = to_oranges; 
            to_oranges :: dest_q = oranges;
        ); 
        assert!(oranges.0 == 0)
    }
}

fn multi_carvan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); // No oranges
    let middle = world.spawn(ToOranges(destination)).id();
    world.spawn((ToOranges(middle), Origin)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(multi_carvan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn multi_carvan_sys(
    origin_q: Query<&ToOranges, With<Origin>>,
    to_oranges_q: Query<&ToOranges, Without<Origin>>,
    dest_q: Query<&Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(to_oranges :: to_oranges_q = to_oranges => to_oranges :: dest_q = oranges); 
        assert!(oranges.0 == 0)
    }
}

fn no_break_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); // No oranges
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(assert_no_oranges_no_break_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn assert_no_oranges_no_break_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(to_oranges :: dest_q = oranges); // No line-break ';' in the caravan 
        assert!(oranges.0 == 0)
    }
}

fn read_carvan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); // No oranges
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(assert_no_oranges_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn assert_no_oranges_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(to_oranges :: dest_q = oranges;);
        assert!(oranges.0 == 0)
    }
}

fn inferred_mutability_caravan() {
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

*/