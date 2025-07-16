
# Inside week-1 folder : Macros in Rust: A Practical Guide

![alt text](https://miro.medium.com/v2/resize%3Afit%3A1400/format%3Awebp/1%2AO5-UwtRWqDKBJIoDJzTUJQ.png)

This repository explores **macros in Rust** through hands-on examples, demonstrating their power in metaprogramming, serialization, and code generation.
Inspired by the SuperDev Fellowship Week-1 session.

---

## 🧠 What's Inside?

### 📁 `src/macros.rs`

A complete walkthrough of:

* **Declarative macros** using `macro_rules!` (e.g., a custom `vector!` macro)
* **Display & Debug traits** to enhance how structs are printed with `println!`
* **Custom serialization/deserialization** via traits like `Serializable` and `Deserialize`
* Example types like `Swap` and `User` used to demonstrate byte-level transformations and macro-friendly printing

### 📁 `src/main.rs`

* Example trait: `Shape` with `area()` and `perimeter()` methods
* Structs: `Rect` and `Square` implementing the `Shape` trait
* `get_area_perimeter()` function using trait-based polymorphism
* Execution of macros and trait-based logic via `macros::macro_print()`

---

## 📌 Key Concepts

### Declarative Macros (`macro_rules!`)

Macros in Rust let us write code that writes code. For example:

```rust
macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

This allows dynamic vector creation:

```rust
let v = vector![1, 2, 3];
println!("{:?}", v); // Output: [1, 2, 3]
```

### Trait-Based Serialization

Using traits like `Serializable` and `Deserialize`:

```rust
let s = Swap { qty_1: 10, qty_2: 20 };
let bytes = s.serialize(); // Converts to Vec<u8>
let recovered = Swap::deserialize(bytes).unwrap(); // Back to struct
```

Output:

```rust
Serialized Swap: [10, 0, 0, 0, 20, 0, 0, 0]
Deserialized Swap: qty_1 = 10, qty_2 = 20
```

### Printing Structs with Traits

Custom implementations of `Display` and `Debug` allow formatted and debug-friendly output:

```rust
println!("{}", user);   // Uses Display
println!("{:?}", user); // Uses Debug
```

---

## 🏗️ How to Run

```bash
git clone https://github.com/Yuvraj-cyborg/solana-fellowship-notes
cd solana-fellowship-notes
cargo run
```

---
