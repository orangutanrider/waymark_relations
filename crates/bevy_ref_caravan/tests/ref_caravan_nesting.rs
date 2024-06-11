use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use bevy_ref_caravan::ref_caravan;

#[derive(Component)]
struct ToHub(Entity);
impl ToHub {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct ToOranges(Entity);
impl ToOranges {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct ToApples(Entity);
impl ToApples {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct Oranges(u32);

#[derive(Component)]
struct Apples(u32);

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct TestSchedule;

#[test]
fn nested_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let apples = world.spawn(Oranges(2)).id(); 
    let oranges = world.spawn(Apples(3)).id(); 
    let hub = world.spawn((ToApples(apples), ToOranges(oranges))).id();
    world.spawn(ToHub(hub)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(nested_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn nested_caravan_sys(
    origin_q: Query<&ToHub>,
    hub_q: Query<(&ToOranges, &ToApples)>,
    oranges_q: Query<&Oranges>,
    apples_q: Query<&Apples>,
) {
    for to_hub in origin_q.iter() {
        ref_caravan!(
            to_hub :: hub_q = (to_oranges, to_apples) => {
                to_oranges :: oranges_q = oranges,
                apples_q :: apples_q = apples,
            }
        ); 
        assert!(oranges.0 == 2);
        assert!(apples.0 == 3);
    }
}