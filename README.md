![Caravan Whiteboard Banner](https://github.com/user-attachments/assets/7785f155-1693-4a19-a601-3e5889af102c)
=
Caravan is a function-like procedural macro built to make expressing query.get statements easier. For this goal, it employs a minature programming language that is written into the function parameters; Example shown below.
```Rust
ref_caravan!(entity :: query = bindings);
```
This code would expand into the following:
```Rust
let Ok(bindings) = query.get(entity) else { return; };
```
That's the most basic, and expected to be the most common, use-case. The macro has a wide range of additional features, enabling it to be more flexible. Caravan is currently a work in progress but it's primary functions are finished. Documentation is planned but not finished; The best way to understand the macro as of now would be to take a look at the tests that have been written for it: https://github.com/orangutanrider/bevy_caravan/tree/main/tests
