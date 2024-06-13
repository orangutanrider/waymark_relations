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
struct ToVegtableHub(Entity);
impl ToVegtableHub {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct ToFruitHub(Entity);
impl ToFruitHub {
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

#[derive(Component)]
struct ToCarrots(Entity);
impl ToCarrots {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct ToOnions(Entity);
impl ToOnions {
    fn go(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
struct Carrots(u32);

#[derive(Component)]
struct Onions(u32);

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct TestSchedule;

#[test]
fn semi_colon_into_scope() { // Only => is meant to be valid for going into a new scope.
    let mut world =  World::new();
    
    // Create entities + components
    let oranges = world.spawn(Oranges(2)).id(); 
    let apples = world.spawn(Apples(3)).id(); 
    let hub = world.spawn((ToApples(apples), ToOranges(oranges))).id();
    world.spawn(ToHub(hub)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(semi_colon_into_scope_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn semi_colon_into_scope_sys(
    origin_q: Query<&ToHub>,
    hub_q: Query<(&ToOranges, &ToApples)>,
    oranges_q: Query<&Oranges>,
    apples_q: Query<&Apples>,
) {
    for to_hub in origin_q.iter() {
        ref_caravan!(
            to_hub :: hub_q = (to_oranges, to_apples); {
                to_oranges :: oranges_q = oranges,
                to_apples :: apples_q = apples,
            }
        ); 
        assert!(oranges.0 == 2);
        assert!(apples.0 == 3);
    }
}

#[test]
fn comma_into_scope() { // Only => is meant to be valid for going into a new scope.
    let mut world =  World::new();
    
    // Create entities + components
    let oranges = world.spawn(Oranges(1)).id(); 
    let apples = world.spawn(Apples(2)).id(); 
    let carrots = world.spawn(Carrots(3)).id(); 
    let onions = world.spawn(Onions(4)).id(); 

    let vegtable_hub = world.spawn((ToCarrots(carrots), ToOnions(onions))).id();
    let fruit_hub = world.spawn((ToApples(apples), ToOranges(oranges))).id();

    let hub = world.spawn((ToFruitHub(fruit_hub), ToVegtableHub(vegtable_hub))).id();
    world.spawn(ToHub(hub)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(comma_into_scope_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn comma_into_scope_sys(
    origin_q: Query<&ToHub>,
    hub_q: Query<(&ToVegtableHub, &ToFruitHub)>,
        fruits_q: Query<(&ToApples, &ToOranges)>,
            oranges_q: Query<&Oranges>, apples_q: Query<&Apples>,
        vegtables_q: Query<(&ToCarrots, &ToOnions)>,
            carrots_q: Query<&Carrots>, onions_q: Query<&Onions>,
) {
    for to_hub in origin_q.iter() {
        ref_caravan!(
            to_hub :: hub_q = (to_vegtables, to_fruits) => {
                to_vegtables :: vegtables_q = (to_carrots, to_onions), { 
                    to_carrots :: carrots_q = carrots,
                    to_onions :: onions_q = onions,
                },
                to_fruits :: fruits_q = (to_oranges, to_apples), {
                    to_oranges :: oranges_q = oranges,
                    to_apples :: apples_q = apples,
                },
            }
        ); 
        assert!(oranges.0 == 1);
        assert!(apples.0 == 2);
        assert!(carrots.0 == 3);
        assert!(onions.0 == 4);
    }
}