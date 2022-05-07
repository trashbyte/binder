# Binder

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/trashbyte/binder/build?logo=github)
[![Crates.io](https://img.shields.io/crates/v/binder)](https://crates.io/crates/binder)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
[![Docs.rs](https://img.shields.io/endpoint?url=https%3A%2F%2Fgist.githubusercontent.com%2Ftrashbyte%2F1cbfc846fa8473de52bf7beadf8e690d%2Fraw%2F8e78e0340c6c7d20a70e52210284d553ac0885b0%2Fcoverage-binder.json)](https://docs.rs/binder/)
[![Test Coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fgist.githubusercontent.com%2Ftrashbyte%2F1cbfc846fa8473de52bf7beadf8e690d%2Fraw%2F6061a221a5ed44082fc997a7cb70ce79dbf2c0b2%2Ftest-badge-binder.json)](https://codecov.io/gh/trashbyte/binder)

A simple, zero-dependency property-binding framework. It was originally designed
to use [imgui-rs](https://github.com/imgui-rs/imgui-rs) without drowning in mutable
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
