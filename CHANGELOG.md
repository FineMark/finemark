# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added the initial parser pipeline structure with `document`, `block`, `markdown`, `element`, `escape`, `token`, and parser utility modules.
- Added markdown block parsers for headings, blockquotes, horizontal lines, and lists.
- Added line parsing and recursion depth utilities for nested document parsing.
- Added source-segment based nested parsing support for markdown blockquotes, headings, and list items.
- Added AST modules split by role: block elements, leaf elements, and list elements.
- Added heading metadata for folded headings and section indexes.
- Added list kind modeling with unordered lists and ordered list styles.
- Added structured `@` command parsing for headings, quotes, horizontal lines, links, tables, table rows, table columns, and raw comments.
- Added bracketed command parameters with ordered `IndexMap` storage and span-preserving parameter values.
- Added balanced single-brace body parsing with preserved body delimiter spans for LSP and diagnostics.
- Added inline markdown text styles for bold, italic, strikethrough, underline, superscript, and subscript.
- Added text-style AST nodes with open and close delimiter spans.
- Added parser tests for inline markdown text styles and nested `***...***` bold/italic parsing.
- Added location-aware parser fixture tests with checked-in `.fm` inputs and `.json` expected outputs.
- Added fixture newline normalization so CRLF checkouts compare against LF-based spans consistently.
- Added parser coverage for bare `\r` as text rather than a FineMark line break.

### Changed

- Removed redirect-specific document parsing from the FineMark parser structure.
- Moved block dispatch to `parser/block.rs`, with markdown implementations under `parser/markdown/`.
- Renamed inline code AST support to `InlineCodeElement` / `Element::InlineCode`.
- Renamed thematic break terminology to `HLine`.
- Limited unordered markdown list markers to `- `.
- Replaced markdown block parsing dispatch with direct element parsing for the structured `@` grammar.
- Changed heading fold state from a dedicated AST field to command parameters.
- Changed command parameters from `Vec<Parameter>` to `IndexMap<String, Parameter>` for ordered key lookup.
- Changed unknown `@name[...]` forms to fall back to plain text instead of parser errors.
- Changed `@table` and `@row` parsing so each structural command owns its valid child grammar.
- Changed `@quote` AST to remove obsolete markdown blockquote marker spans.

### Removed

- Removed markdown block parser modules for headings, blockquotes, horizontal lines, and lists.
- Removed slash-based comment dispatch in favor of `@comment{...}`.
- Removed the `BoldItalic` parser/guard/AST path; `***...***` now parses as nested bold and italic, matching SevenMark behavior.

### Notes

- Token parsers for future inline constructs remain in place even when not yet wired into the active grammar.
