# pngnator

A Rust CLI tool for hiding and extracting secret data in PNG files using PNG chunk manipulation.

## Usage

```bash
# Encode a secret message into a PNG
cargo run -- encode <input.png> <chunk_type> <message> [output.png]

# Decode a secret message from a PNG
cargo run -- decode <input.png> <chunk_type>

# Remove a chunk from a PNG
cargo run -- remove <input.png> <chunk_type>

# Print PNG chunk information
cargo run -- print <input.png>
```

The `chunk_type` must be exactly 4 ASCII letters (e.g. `ruSt`). For encoding, `output.png` defaults to `output.png` if not specified.
