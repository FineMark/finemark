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

### Changed

- Removed redirect-specific document parsing from the FineMark parser structure.
- Moved block dispatch to `parser/block.rs`, with markdown implementations under `parser/markdown/`.
- Renamed inline code AST support to `InlineCodeElement` / `Element::InlineCode`.
- Renamed thematic break terminology to `HLine`.
- Limited unordered markdown list markers to `- `.

### Notes

- `element_parser` is intentionally still a TODO single-element dispatcher; inline parser implementation is planned separately.
