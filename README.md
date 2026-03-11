# rust-dependency-injection-example

[![Rust](https://img.shields.io/badge/rust-1.72+-orange)](https://www.rust-lang.org/) 
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](./LICENSE)

Zero-cost, compile-time Dependency Injection framework in Rust — with supporting examples demonstrating DI concepts.

---

## Overview

This repository demonstrates how to build a compile-time dependency injection (DI) framework in Rust.  
It is designed for educational purposes and to illustrate **modular application design** using Rust macros.

---

## Examples

The repo contains two crates:

- **`di_example`** — executable crate demonstrating DI mechanics at compile time.  
  Key examples:
  - [Best demonstration of the approach](https://github.com/amidukr/rust-dependency-injection-example/blob/main/di_example/src/examples/di_polymorphism.rs)
  - [Simplest example containing all DI elements](https://github.com/amidukr/rust-dependency-injection-example/blob/main/di_example/src/examples/di_init.rs)
  - [Intermediate examples comparing DI and manual initialization](https://github.com/amidukr/rust-dependency-injection-example/blob/main/di_example/src/examples/di_no_init.rs)  
  - [No-DI baseline](https://github.com/amidukr/rust-dependency-injection-example/blob/main/di_example/src/examples/no_di.rs)

- **`di_macro`** — procedural macro crate supporting compile-time DI. Rust requires macros to live in a separate crate.  
  Example showing what the macro generates: [struct_enumerator.rs](https://github.com/amidukr/rust-dependency-injection-example/blob/main/di_example/src/examples/struct_enumerator.rs)

**Note on examples:**  
Each example file is kept in a single file for simplicity, but contains multiple components organized into **logical groups**. Each group represents a mini-library and is annotated with comments. In a real application, these groups could be extracted into separate crates, and the procedural macro provided by the DI framework then **wires all components together at compile time**, producing a composite `ApplicationContext` that contains everything from all groups.

---
## Visual Overview of Application Profiles

    +------------------------+        +------------------------+
    |   Profile1 Context     |        |   Profile2 Context     |
    +------------------------+        +------------------------+
    | Configuration          |        | Configuration          |
    | PostgresDb             |        | Oracle                 |
    | RabbitMq               |        | RabbitMq               |
    | Kafka                  |        |                        |
    | ReadController         |        | ReadController         |
    | WriteController        |        | WriteController        |
    +------------------------+        +------------------------+

Each profile builds its own `ApplicationContext`.  
The DI procedural macro **automatically wires all components** within each context at compile time.

---

## How to Run

This will execute all examples sequentially and print their outputs:

```bash
# 1. Clone the repository
git clone https://github.com/amidukr/rust-dependency-injection-example

# 2. Navigate to di_example crate
cd rust-dependency-injection-example/di_example/

# 3. Run all examples sequentially
cargo run

# Runs all examples sequentially and prints their outputs
```

---

## Learn More

A detailed explanation and motivation will be provided in an accompanying Medium article.

(Medium article link placeholder — to be added when ready)