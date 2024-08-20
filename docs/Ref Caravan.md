--------
# [Ref Caravan]

--------
## Theory

Ref caravan is a representation of the [Path] taken and [Binding] output of a query chain, the traversal of which is expected to be afforded by [Waymarks].

In purity, ref caravan declares: 
- The [Waymark] and its input key (if there is a key).
- The query that the waymark's entity data is being input into.
- The bindings created from that query.

In reality, as a Rust macro, ref caravan also declares:
- The exit method upon a query access failure.
- The pre-processing statement for getting a waymark's entity.
- Whether or not the created entity binding from the waymark should shadow the entity's originating waymark binding.
- Whether or not to create a binding from a waymark, or to just input the result of the waymark's goto function into the query directly.
- The mutability of the bindings.
- The query access method.

Ref caravan accepts macros in its inputs, to allow for pure-er representations. 
It can also be augmented through attribute macros, to further the goal of a pure representation.

--------
## Design

The design of ref caravan's Rust-macro format.

As an automation of code; An abbreviation of code; A macro implicitly has the goals of being [Quick to Write] and [Readable].

Design goals:
[Rust-like]
[Minimal]
[Structured]
[Configurable]

### [Rust-like]

The syntax of the format should be relate-able to common Rust code; It should be [Readable] based on familiarity with Rust. A Rust developer should be able to come to grips with the macro quickly. It should be [Quick to Write], by utilising existing Rust language features. Like Rust it should be a [Declarative] format. The macro should be similar to and take advantage of Rust.

Given a non-Rust implementation you could call this goal [Relate-able]. Though a point like [Declarative] may be specific to Rust.

### [Minimal]

The syntax should be concise, it should be more [Quick to Write] a caravan statement than it is to write the Rust code that the caravan expands to. The macro should be [Readable] based on the focus and simplicity of its information. Writing the macro's format should be [Intuitive] and simple, it shouldn't require extensive prior knowledge into the intricacies of the format.

### [Structured]

The format should have a consistent structure; The macro should be [Readable] based on this consistency. It should be clear as to where inputs are inserted and this clarity should make it easy to input other macro statements into the macro. The macro should be [Quick to Write] based on this consistency and structure. The format should reflect the concepts that it is modelling, [Path]s and [Binding]s.

### [Configurable]

The macro should be configurable, so that it can be adapted to different project structures and abstractions. It should still hold firm in its default configuration, targeting [Waymarks], but it should be able support other structures. It should do this on a project-level scope, declaring the macro's configuration is ideally done once and is then consistent throughout a project.

Additionally, when it is free to do so ("free" as in the given change does not negatively affect the previously stated goals), the macro should aim to be [Flexible]; Flexibility, meaning that the macro should be able to be configured in smaller contextual scopes, like a single function.