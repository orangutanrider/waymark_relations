--------
# [Composable Transmission] 

--------
## Background

A pattern afforded by the [Waymark].
The design of this pattern is to allow a developer to create compositions, containing [Unique Transmissions], without needing to define new systems and components for the explicit purpose of transmitting data in that specific composition.

### [Unique Transmissions]

A unique transmission is defined by the usage of a single type of [Functionality] multiple times across a composition, for differing behavioural reasons.

### [Functionality]

The definition of functionality, that we are concerned with here: 
> An implementation defining components that are purposed for an explicit but configurable job.

For example, a physics engine implementation would define many components, related to its one functionality of simulating physics:
- RigidBody
- Collider
- Friction
- Mass
- Gravity
- ect.

It is a generic functionality, that can be used in multiple ways, configurable through the components you create and set.

### The Problem

What is it about a unique transmission that would require someone to define a new system and component?
If we wanted to transmit physics-data, and we wanted this to be composable so that we don't have to define systems and components related to this transmission, for each composition; We may consider defining the following waymark, for the physics functionality:

```
/// Component
/// Waymark
struct ToPhysics(Entity);
impl ToPhysics { ... }
```

If the transmitting systems have their own flagging components and are kept seperate from the composition definitions, you would be free to create compositions in this way. The problem you run into is, is when a composition features more than one entity that utilises physics functionality.

The "ToPhysics" waymark cannot point to two different entities at once, and you cannot have multiple of them per-entity. In the context of the composition, you could fix this by defining new components and systems, like so:

```
/// Component
/// Waymark
struct ToCharacterPhysics(Entity);
impl ToCharacterPhysics { ... }
```

```
/// Component
/// Waymark
struct ToTailPhysics(Entity);
impl ToTailPhysics { ... }
```

The goal of this pattern is so you don't need to do this; So you don't need to define explicit transmission related components and systems that are unique to a given composition.

--------
## [A-to-B Transmission]

For the purpose of explaining the pattern, I will only be concerned with explicit A-to-B transmission.

The transmission is defined by its targeting of two components A and B, and its mutation of B, while using data from A. Additionally, the transmission is defined by its transformation of the data being transmitted, you could call T; I call this the "Transformation Signature". 

The origin of the transmission is not necessarily in the context of either A or B, and the data local to the origin is potentially relevant to the mutation of B; I will only consider cases where all relevant data comes from A; The question of whether or not it is local to either isn't of concern in this case.

```
/// T.
/// An A to B transformation signature type definition.
trait TransformationSignature<A, B>
where 
    A: Component,
    B: Component, 
{
    fn transmit(a: &A, b: &mut B);
}
```

--------
## [Direct Composable Transmission]

*Unoptimized, Rust-based, declarative pseudo-code*

```
/// A transmitter that transmits its data directly.
trait DirectTransmitter<A, B> 
where
    A: Component,
    B: Component,
    Self: TransformationSignature<A, B> + Component, // Self is a waymark too
{
    fn read(&self) -> &Vec<(Entity, Entity)>; // A vec of Entity A and Entity B.
}

/// Component
struct TransmitterSwitch(bool);
impl TransmitterSwitch { ... } // It can be mutated; It can be switched on or off.

fn a_to_b_transmission<A, B, T>(
    main_q: Query<(&T, Option<&TransmitterSwitch>)>,
    a_q: Query<&A>,
    mut b_q: Query<&mut B>,
) where
    A: Component,
    B: Component,
    T: DirectTransmitter, 
{
    for ( ... ) in main_q {
        If the switch exists and is off, then skip this iteration.
            
        for ab_key_pair in direct_transmitter {
            Get A and B entities, and use them to get A and B components via the queries.
            With A and B, run the transmit() function, that T declares.
        }
    }
}
```

### Example usage

```
struct NavigationToLocomotion(Vec<(Entity, Entity)>)
impl TransformationSignature<A, B> for NavigationToLocomotion {
    fn transmit(a: &A, b: &mut B) { ... }
}
impl DirectTransmitter for NavigationToLocomotion {
    fn read(&self) -> &Vec<(Entity, Entity)> { ... }
}

fn plugin(app: &mut App) {
    app.add_systems(Update, a_to_b_transmission::<Navigation, Locomotion, NavigationToLocomotion>())
}
```

The relevance of the implementation, is that these transmission definitions can be kept separate from your composition definitions entirely. They are purely defined through what functionalities they target and how they transform the data, concepts unique to any given behaviour do not have to be involved.

With that, the transmitter components become generic functionality that can be used across many different compositions, where it is seen fit to do so. It allows for a modular approach.

--------
## [Hub-Based Composable Transmission]

*Unoptimized, Rust-based, declarative pseudo-code*

```
/// Unique transmission key
struct TKey{
    a: HubKey,
    b: HubKey,
}

/// A transmitter that transmits its data via a hub structure
trait ConstituentTransmitter<A, B> 
where
    A: Component,
    B: Component,
    Self: TransformationSignature<A, B> + Component,
{
    fn read(&self) -> &Vec<TKey>;
}

struct HubKey(usize);

/// Component, Waymark
struct ToHub(Entity);
impl ToHub {
    fn new(entity: Entity) -> Self { ... }
    fn read(&self) -> &Entity { ... }
}

/// Component, Waymark
struct Hub(Vec<Entity>);
impl Hub {
    fn new(entities: Vec<Entity>) -> Self { ... }
    fn read(key: usize) -> &Entity { ... } // Input a HubKey
}

/// Component
struct TransmitterSwitch(bool);
impl TransmitterSwitch { ... } // It can be mutated; It can be switched on or off.

fn a_to_b_hub_transmission<A, B, T>(
    main_q: Query<(&T, &ToHub, Option<&TransmitterSwitch>)>,
    hub_q: Query<&Hub>,
    a_q: Query<&A>,
    mut b_q: Query<&mut B>,
) where
    A: Component,
    B: Component,
    T: ConstituentTransmitter, 
{
    for ( ... ) in main_q {
        If the switch exists and is off, then skip this iteration.
            
        for ab_key_pair in constituent_transmitter {
            Go to the hub entity, using the ToHub waymark.
            Use keys to get entities, using the Hub component.
            Get A and B components, by using their queries.
            With A and B, run the transmit() function, that T declares.
        }
    }
}
```

### Example usage

```
// Composition

/// Component set
struct CompHub {
    hub: Hub,
}

/// Component-set
struct CompBehav {
    behav: Behaviour,
    
    switch: TransmitterSwitch, 
    nav_to_loco: NavigationToLocomotion,
    waystone: Waystone,
    to_hub: ToHub,
}

/// Component-set
struct CompNav {
    nav: Navigation,
}

/// Component-set
struct CompLoco {
    loco: Locomotion,
}

const NAV: usize = 0;
const LOCO: usize = 1;

fn spawn_comp(
    ...
) {
    Spawn CompNav on new entity, named "nav".
    Spawn CompLoco on new entity, named "loco".
    Spawn CompHub on new entity, named "hub".
        Add "nav" and "loco" to Hub component, in order (0 = "nav", 1 = "loco").
    Spawn CompBehav on new entity, named "behav".
        Add (NAV, LOCO) to NavigationToLocomotion component.
        Set ToHub as "hub".
}
```

In principle it is the same as the direct transmission, the difference is that every transmission must first travel through the hub. The advantage of this is in the flexibility and security that it could provide you.
