# Binder

[![Crates.io](https://img.shields.io/crates/v/binder)](https://crates.io/crates/binder)
[![Docs.rs](https://docs.rs/binder/badge.svg)](https://docs.rs/binder/)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
![build](https://github.com/trashbyte/binder/workflows/build/badge.svg)
[![Coverage](https://img.shields.io/codecov/c/github/trashbyte/binder/master.svg)](https://codecov.io/gh/trashbyte/binder)
![doc coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fdoc-coverage.s3-us-west-2.amazonaws.com%2Fbinder.json)

A simple, zero-dependency property-binding framework. Designed to use
[imgui-rs](https://github.com/imgui-rs/imgui-rs) without drowning in mutable
references to everything and constantly fighting with the borrow checker.
It uses internal mutability and runtime borrow checking to avoid lifetime
issues. Designed to be fully memory- and thread-safe, although there might be
bugs since it's brand-new.

### Usage

```rust
pub struct PropHaver {
    pub prop: binder::Property<f32>
}
fn use_prop(p: &PropHaver, ui: &imgui::Ui) {
    ui.slider("wow what a cool slider", &mut p.prop.bind());
}
```

### Stability

This thing is brand-new, so expect bugs and/or API-breaking changes in the near future.

### License

Binder is licensed under the [MIT License](https://opensource.org/licenses/MIT).