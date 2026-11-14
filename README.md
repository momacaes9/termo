# termo — Thermodynamics Principles Calculator

Capstone project for **Introduction to Computing — Universidade Presbiteriana Mackenzie**.

A menu-driven terminal program for studying and calculating:

| Option | Topic | What it calculates |
|---|---|---|
| 1 | **First Law** (Conservation of Energy) | Q, W or ΔU from `Q = ΔU + W`; cal → J conversion |
| 2 | **Transformations** (Gas Laws) | Isothermal (Boyle-Mariotte), Isobaric (Charles), Isochoric (Gay-Lussac), Adiabatic |
| 3 | **Second Law** (Heat Engines) | Carnot efficiency of refrigerators/heaters, rated GOOD / AVERAGE / POOR |
| 4 | **Centrifuge Machine** *(Rust version only)* | Force, rotations and power in Turbo/Eco modes and S/L capacities |

> The program itself (menus, prompts and output) is in Portuguese.

## Structure

```
python/   original project, exactly as submitted (Google Colab)
rust/     the same calculator rewritten in Rust
```

## Python version (original)

```bash
python3 python/projeto_de_programacao.py
```

## Rust version

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cd rust
cargo run      # run the calculator
cargo test     # run the automated formula tests
```

### What's different in the Rust version

- **Looping menu**: returns to the menu after each calculation (the Python version exited).
- **Validated input**: typing a letter doesn't crash the program; decimals work with either a comma or a dot.
- **Missing data**: in the First Law and Gas Laws calculators, leaving more than one field blank shows a message instead of an error.
- **Real adiabatic process**: uses `P·V^γ = constant` and `W = (P1·V1 − P2·V2)/(γ − 1)`, and also shows the original project's simplified result for comparison.
- **Second Law**: uses 273.15 for °C → K, rejects a hot source colder than the cold one, and shows the ideal COP for refrigerators.
- **Centrifuge**: the calculator that existed in the Python file but wasn't in the menu is now option 4.
- **Tests**: every formula has unit tests (`cargo test`).
