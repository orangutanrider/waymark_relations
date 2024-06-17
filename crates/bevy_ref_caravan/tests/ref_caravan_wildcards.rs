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

#[derive(Component)]
struct Origin;

#[derive(Component)]
struct OrangesBasket{
    to_oranges: Entity,
    ants: u32,
}
impl OrangesBasket {
    fn go(&self) -> Entity {
        return self.to_oranges
    }
}

#[test]
fn literal_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    world.spawn((Origin, Oranges(3))); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(literal_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn literal_caravan_sys( // It's two queries onto the same entity.
    q: Query<Entity, With<Origin>>, 
    oranges_q: Query<&Oranges>, 
) {
    for entity in q.iter() {
        ref_caravan!(@entity :: oranges_q = oranges);
        assert!(oranges.0 == 3)
    }
}

#[test]
fn direct_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn(Oranges(0)).id(); // No oranges
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(direct_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn direct_caravan_sys(
    origin_q: Query<&ToOranges>,
    dest_q: Query<&Oranges>,
) {
    for to_oranges in origin_q.iter() {
        ref_caravan!(|to_oranges :: dest_q = oranges);
        assert!(oranges.0 == 0)
    }
}

#[test]
fn overlap_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn((Oranges(0), Apples(2))).id(); // No oranges
    world.spawn(ToOranges(destination)); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(overlap_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn overlap_caravan_sys(
    origin_q: Query<&ToOranges>,
    oranges_q: Query<&Oranges>,
    apples_q: Query<&Apples>
) {
    for oranges_entity in origin_q.iter() {
        ref_caravan!(~oranges_entity :: oranges_q = oranges);

        assert!(oranges.0 == 0);

        let Ok(apples) = apples_q.get(oranges_entity) else {
            panic!()
        };

        assert!(apples.0 == 2)
    }
}

#[test]
fn lift_caravan() {
    let mut world =  World::new();
    
    // Create entities + components
    let destination = world.spawn((Oranges(0), Apples(2))).id(); // No oranges
    world.spawn(OrangesBasket{
        to_oranges: destination,
        ants: 300,
    }); // Origin

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(lift_caravan_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn lift_caravan_sys(
    origin_q: Query<&OrangesBasket>,
    oranges_q: Query<&Oranges>,
    apples_q: Query<&Apples>
) {
    for to_oranges_entity in origin_q.iter() {
        ref_caravan!(^to_oranges_entity :: oranges_q = oranges);

        assert!(oranges.0 == 0);

        let Ok(apples) = apples_q.get(oranges_entity) else {
            panic!()
        };

        assert!(apples.0 == 2);

        assert!(to_oranges_entity.ants == 300);
    }
}

#[test]
fn nested_caravan_with_wildcards() {
    let mut world =  World::new();
    
    // Create entities + components
    let oranges = world.spawn(Oranges(2)).id(); 
    let apples = world.spawn(Apples(3)).id();
    let carrots = world.spawn(Carrots(4)).id();  
    let onions = world.spawn(Onions(5)).id();  
    world.spawn((ToApples(apples), ToOranges(oranges), ToCarrots(carrots), ToOnions(onions), Origin)); // Hub

    // Create system
    let mut schedule = Schedule::new(TestSchedule);
    schedule.add_systems(nested_caravan_with_wildcards_sys); // Assertion system
    schedule.initialize(&mut world).unwrap();
    schedule.run(&mut world); // Run system
}

fn nested_caravan_with_wildcards_sys(
    q: Query<Entity, With<Origin>>, // On the same entity
    hub_q: Query<(&ToOranges, &ToApples, &ToCarrots, &ToOnions)>, // On the same entity
    oranges_q: Query<&Oranges>,
    apples_q: Query<&Apples>,
    carrots_q: Query<&Carrots>,
    onions_q: Query<&Onions>,
) {
    for hub in q.iter() {
        ref_caravan!(
            @hub :: hub_q = (to_oranges_entity, apples_entity, to_carrots, to_onions) => {
                ^to_oranges_entity :: oranges_q = oranges,
                ~apples_entity :: apples_q = apples,
                to_carrots :: carrots_q = carrots,
                |to_onions :: onions_q = onions,
            }
        ); 
        assert!(oranges.0 == 2);
        assert!(apples.0 == 3);
        assert!(carrots.0 == 4);
        assert!(onions.0 == 5);
    }
}