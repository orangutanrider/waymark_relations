--------
# [Waymark]

A waymark is a definition of component.
It is defined by the containing and exposing of immutable entity data, by the component.
They point to other entities in the ECS world.

``` Rust
/// Component
struct Waymark(Entity);
impl Waymark {
    fn new(entity: Entity) -> Self {
        return Self(entity)
    }
    
    fn goto(&self) -> &Entity { // Specifically not "&mut"
        return &self.0 
    }
}
```

They're purposed, for creating static cross-entity connections in pre-defined compositions.

A waymark is always expected to return its held entity data, it is never expected to return a Null or Error value; Even if their destination has become invalid, it is not the job of the waymark to detect that and update itself.

They are internally immutable, but in the context of the ECS world they can be created, destroyed, inserted, and so on; Externally, they are mutable.

Waymarks are like entity references as components; Systems use them to transmit data across different contexts of entity.

--------
Take note, a waymark can hold multiple entities. These waymarks are still expected to never return a Null or Error value; The responsibility of accessing them correctly is off-loaded to the systems interacting with them.

``` Rust
/// Component
struct Waymark(Vec<Entity>);
impl Waymark {
    fn new(entities: Vec<Entity>) -> Self {
        return Self(entities)
    }
    
    fn goto(&self, key: usize) -> &Entity {
        return &self.0[key]
    }
}
```
