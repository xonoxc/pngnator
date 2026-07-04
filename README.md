# pngnator

A Rust CLI tool for hiding and extracting secret data in PNG files using PNG chunk manipulation.

## Status

Work in progress.

## Usage

```bash
# Encode a secret message into a PNG
cargo run -- encode <input.png> <output.png> <message>

# Decode a secret message from a PNG
cargo run -- decode <input.png>

# Remove a secret chunk from a PNG
cargo run -- remove <input.png> <chunk_type>

# Print PNG chunk information
cargo run -- print <input.png>
```
