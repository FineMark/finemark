# finemark-parser

Core parsing engine for the [FineMark](https://github.com/FineMark/finemark) structured document language.

[![crates.io](https://img.shields.io/crates/v/finemark-parser.svg)](https://crates.io/crates/finemark-parser)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue)](https://github.com/FineMark/finemark/blob/main/LICENSE-APACHE)

## Usage

```rust
use finemark_parser::parse_document;

let input = "@h2{Hello} **world**";
let ast = parse_document(input);
```

The parser is fault-tolerant — it always returns a complete `Vec<Element>` tree.
Unrecognised syntax falls back to `Element::Text` nodes rather than hard errors.

## AT command syntax

```
@keyword [param, key="value"] { body }
```

Whitespace between `@keyword`, `[params]`, and `{body}` is optional.
Bodies are parsed recursively, so AT commands can be nested:

```
@quote [cite="https://example.com"] {
  Quoted text with @link[href="https://example.com"]{a link} inside.
}
```

## Features

| Feature | Description |
|---|---|
| *(default)* | Full parser with all element types |
| `include-locations` | Emits byte-offset spans in serialized AST output; required for fixture tests |

## Testing

Tests require the `include-locations` feature:

```bash
cargo test --features include-locations
```

### Regenerating expected fixtures

After parser changes that affect AST output, regenerate all `tests/fixtures/parser/*/expected/*.json` files:

```bash
cargo run --example gen_expected --features include-locations
```

## License

Licensed under either of [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
or [MIT](https://opensource.org/licenses/MIT) at your option.
