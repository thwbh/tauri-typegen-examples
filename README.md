# tauri-typegen-examples

This project showcases the capabilities of [tauri-typegen](https://github.com/thwbh/tauri-typegen), a Rust crate that generates TypeScript bindings from Tauri commands. It demonstrates how tauri-typegen enables type-safe communication between your Rust backend and TypeScript frontend, with optional [Zod](https://zod.dev) schema validation support.

## Examples

### [`ts/`](./ts) - TypeScript Bindings Example
A demonstration of tauri-typegen's core features without validation. This example shows:
- Type generation from Rust structs and enums with serde attributes
- Command bindings for various patterns (simple, async, with parameters, Result types)
- Event handling with typed payloads
- Channel/streaming support with typed data
- Support for complex types (nested structs, Option, Vec, HashMap)

Perfect for projects that need type safety at compile-time without runtime validation overhead.

### [`zod/`](./zod) - Zod Validation Example
The same feature set as the TypeScript example, but with **Zod runtime validation** enabled. This example additionally demonstrates:
- Auto-generation of Zod schemas from Rust `validator` crate attributes
- Runtime input validation with custom error messages preserved from Rust
- Support for validation constraints: `range`, `length`, `email`, `url`
- Client-side validation before sending data to the backend

Ideal for forms, user input handling, and scenarios requiring data integrity validation.

## How to use in your project
The easiest way is copy/pasting the `tauri.conf.json` and `build.rs` from the example to your `src-tauri` directory and adding `tauri-typegen` as build dependency.

```bash
cargo add --build tauri-typegen
```

This way the bindings will be regenerated everytime your Rust backend changes.

Alternatively, follow the project instructions: [tauri-typegen](https://github.com/thwbh/tauri-typegen)
