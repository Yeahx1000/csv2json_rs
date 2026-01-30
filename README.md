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

## Usage

```bash
csv2json-rs <input_file> [options]
```

so for example, in terminal to test you'd do:

```bash
cargo run --input_file samples/sample.csv --pretty --output-file output.json
```

so cargo run `<input_file_location>` `--option` (like `--pretty`) then `--output-file <path>`. you don't have to type flag `--input_file` or `--output-file` since clap automatically parses the arguments, added for examples and explanation.

if no output is specified, it will default to stdout, which is the terminal.

## Options

```bash
--pretty: Pretty-print the JSON output
--output-file <path>: Write output to a file
```

## Examples

```bash
csv2json-rs samples/sample.csv --pretty
csv2json-rs samples/sample.csv --output-file output.json
```

## Output

```json
[
  {
    "name": "John Doe",
    "age": "30",
    "address": "123 Main St Anytown USA"
  }
]
```

## License

MIT License
