# em_aleph_gen

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Emergence agent — generates Aleph source code from an [AlephTree](https://github.com/aleph-lang/aleph-syntax-tree) intermediate representation.

This agent is the Emergence network integration of the [`alegen`](https://crates.io/crates/alegen) aleph-lang crate.

---

## Role in the Aleph pipeline

```
Source code (any language)
      ↓  em_*_parser
  AlephTree (JSON)
      ↓  em_betareduction / em_constant_folding  (optional)
  AlephTree (JSON)
      ↓  em_aleph_gen  (this agent)
   Aleph source code
```

---

## Query protocol

**Input** (`body`): JSON-serialized `aleph_tree` embryo

**Output**: JSON array containing one `code` embryo:
```json
[{
  "type": "code",
  "properties": {
    "language": "aleph",
    "source_language": "python",
    "content": "-- generated aleph..."
  }
}]
```

---

## Capabilities

```rust
vec!["generator", "aleph"]
```

---

## Deployment

```bash
git clone https://github.com/EmergenceSystem/em_aleph_gen
cd em_aleph_gen
cargo build --release
./target/release/em_aleph_gen
```

Configure via environment variables:
- `EM_DISCO_NODES` — comma-separated `host:port` list (default: `localhost:8080`)
- `EM_FILTER_JWT` — optional JWT token
- `EM_FILTER_RECONNECT_MS` — reconnect delay in ms (default: 5000)
