# Installation

`runar-units` is not yet published on [crates.io](https://crates.io), so add
the Git repository as a dependency in your project's `Cargo.toml`:

```toml
[dependencies]
runar-units = { git = "https://github.com/runar-rs/runar-units.git" }
```

You can specify the version by the usage of the `tag` key:

```toml
[dependencies]
runar-units = { git = "https://github.com/runar-rs/runar-units.git", tag = "0.4.1" }
```


Cargo will fetch the crate and build it when you run `cargo build`, `cargo
check`, or another command that resolves dependencies. Once the crate is
published, the dependency can be changed to a crates.io version.
