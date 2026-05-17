# finemark-ast

AST types for the [FineMark](https://github.com/FineMark/finemark) structured document language.

[![crates.io](https://img.shields.io/crates/v/finemark-ast.svg)](https://crates.io/crates/finemark-ast)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue)](https://github.com/FineMark/finemark/blob/main/LICENSE-APACHE)

## Element types

The central `Element` enum covers all nodes produced by the FineMark parser:

| Category | Variants |
|---|---|
| Text | `Text`, `Escape`, `Error`, `Comment` |
| AT commands | `Heading`, `BlockQuote`, `Link`, `HLine`, `Table`, `TableRow`, `TableColumn` |
| Inline styles | `Bold`, `Italic`, `Strikethrough`, `Underline`, `Superscript`, `Subscript` |
| Inline content | `InlineCode`, `TeX` |
| Structure | `List`, `CodeBlock`, `SoftBreak`, `HardBreak` |

Every element carries a `Span { start, end }` of byte offsets into the original source.

## Features

| Feature | Description |
|---|---|
| *(default)* | All element types; spans are included in the struct fields |
| `include-locations` | Enables `Serialize` impls that include span fields in JSON output |

## License

Licensed under either of [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
or [MIT](https://opensource.org/licenses/MIT) at your option.
