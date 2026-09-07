# Turso local fork

Source: https://github.com/quickwit-oss/tantivy

Release: `0.26.1` (tag has no `v` prefix).
Commit: `d8f4c0b703120ed98f06297724dc1522df6019b9`.
Imported from the GitHub source archive of that commit. Original MIT license,
copyright notices, tests, and workspace crates are retained. No remote fork
is required. Turso's `core/Cargo.toml` selects this directory by path; upstream
path dependencies select the included companion crates, including ownedbytes.
`tantivy-fst` remains an unchanged registry dependency.

This import does not change Tantivy behavior. Formatting normalization, if
needed by Turso's stable toolchain, is recorded in a separate commit.
