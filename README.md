# Near Rust tutorial

## Directory structure

Setup rust environment using `cargo init --lib`

```
.
├── Cargo.toml
└── src
   └── lib.rs
```

- **Cargo.toml**: Similar to `package.json` in Node. It contains dependency list, build settings and package metadata.
- **lib.rs**: Contains contract.

## Rust notes

### Rust binary vs library
1. Binaries are standalone, created using `cargo init`. They have a **main.rs**.
2. Libraries: Used by other programs. Create using `cargo init --lib`. They have a **lib.rs** file.

