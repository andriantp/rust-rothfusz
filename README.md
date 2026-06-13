# Rothfusz Heat Index (Rust)

A simple Rust implementation of the **Rothfusz Heat Index Regression Formula** with additional humidity-based comfort classification designed for tropical environments.

This project is intended for:

* Learning purposes
* Rust experimentation
* IoT prototypes
* Indoor comfort monitoring
* Tropical climate studies
* Smart room systems

---

# What is Heat Index?

Heat Index (HI) estimates how hot the environment actually feels to humans by combining:

* Air temperature
* Relative humidity (RH)

As humidity increases, sweat evaporates less efficiently, making the body perceive the environment as hotter than the measured air temperature.

---

# Features

* Pure Rust implementation
* No external weather services required
* Implements the NOAA Rothfusz regression formula
* Structured output with heat stress interpretation
* Configurable thresholds
* Additional humidity rules for tropical indoor environments
* Simple command-line interface

---

# Project Structure

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    └── rothfusz
        ├── mod.rs
        └── rothfusz.rs
```

---

# Formula

This project uses the NOAA Rothfusz regression formula.

The formula is generally considered valid when:

* Temperature ≥ 26.7°C (80°F)
* Relative humidity ≥ 40%

For lower temperatures, the implementation falls back to the actual air temperature.

Additional tropical humidity rules are applied to better represent perceived comfort in humid regions such as Indonesia.

---

# Heat Stress Categories

| Heat Index                               | Category        |
| ---------------------------------------- | --------------- |
| < 27°C                                   | Ideal           |
| < 27°C + RH ≥ configured humid threshold | Humid           |
| 27°C - 32°C                              | Caution         |
| 32°C - 41°C                              | Extreme Caution |
| 41°C - 54°C                              | Danger          |
| > 54°C                                   | Extreme Danger  |

---

# Run

```bash
cargo run -- <temp> <rh>
```

Example:

```bash
cargo run -- 32 70
```

---

# Example Output

```text
ResultData {
    data: Data {
        temp: 32.0,
        rh: 70.0,
    },
    heat_index_result: HeatIndexResult {
        heat_index_c: 40.409273679555774,
        category: ExtremeCaution,
        comfortable: false,
    },
}
```

Another example:

```bash
cargo run -- 24 90
```

Output:

```text
ResultData {
    data: Data {
        temp: 24.0,
        rh: 90.0,
    },
    heat_index_result: HeatIndexResult {
        heat_index_c: 24.0,
        category: Humid,
        comfortable: false,
    },
}
```

---

# Configuration

Thresholds can be customized when constructing the calculator:

```rust
let repo = Repo::new(
    26.7, // minimum valid temperature
    85.0, // humid RH threshold
);
```

---

# Why Add Tropical Humidity Rules?

Classic Heat Index formulas were primarily designed for hot outdoor environments.

However, in tropical regions, high humidity can significantly affect comfort even when temperatures remain relatively moderate.

Examples:

| Temperature | RH  | Perceived Condition |
| ----------- | --- | ------------------- |
| 24°C        | 50% | Comfortable         |
| 24°C        | 90% | Humid / Stuffy      |
| 32°C        | 70% | Very Uncomfortable  |

This implementation introduces a simple humidity-based adjustment to better represent indoor comfort in tropical climates.

---

# Future Ideas

Possible extensions include:

* Dew Point
* Humidex
* WBGT
* PMV comfort model
* Absolute humidity
* Adaptive comfort
* ESP32 integration
* Embedded implementation
* WASM version
* REST API

---

# References

* NOAA Heat Index
* Rothfusz Regression
* Steadman Heat Index
* ASHRAE Thermal Comfort

---

## Author

**Andrian Tri Putra**

* Medium: https://andriantriputra.medium.com/
* GitHub: https://github.com/andriantp
* GitHub (alt): https://github.com/AndrianTriPutra

---

## License

Licensed under the Apache License 2.0.
