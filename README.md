# csv2json

CLI tool for converting CSV files to JSON arrays

## Tech Stack

- Rust
  - Crates:
    - Serde + serde_json
    - Clap
    - CSV
    - Thiserror

## Goal

- Read a CSV file from a path
- Output JSON array (standard: to stdout or optional --out)
- Handles headers
- Some error logging (bad file, bad CSV, parse issues)
- modular structure, organized by concerns
