## WIT method chaining


Resource methods implicitly use borrowed `self`, so return nothing in WIT and handle fluent chaining in bindings instead.

For Rust, use `wit-bindgen`'s `chainable_methods` support:

- [wit-bindgen#1586](https://github.com/bytecodealliance/wit-bindgen/pull/1586)
- [wit-bindgen#1602](https://github.com/bytecodealliance/wit-bindgen/pull/1602)

This enables `self -> Self` chaining without changing the WIT ABI or adding boundary traffic.

For other languages, use wrappers or equivalent binding-generator support.

- *Rule:* Do **not** implement builder patterns in WIT by returning the same resource.

## Rust guests on v0.2: bind `handle-event` synchronously

`wit-bindgen` (checked with 0.63) emits the lift of an async export's arguments and the lowering of its result inline in one `async move` block. `handle-event` takes and returns `event`, a variant that pulls in most of the API, so that block becomes huge and rustc needs more than 4 GB to type check it and gets OOM-killed, even for a crate that has only the `generate!` call. The 200+ async imports and the other async exports are not the problem: with only `handle-event` lifted synchronously a full plugin builds in about 1.4 GB.

Until `wit-bindgen` moves that code out of the async block, bind v0.2 like this:

```rust
wit_bindgen::generate!({
    path: "wit",
    world: "plugin",
    sync: ["export:handle-event"],
});
```

`handle_event` is then a plain `fn`. It can still call async imports with `wit_bindgen::block_on(...)`. Every other export stays `async`.

- *Rule:* Don't add another async export that takes or returns `event` or a type of similar size.
