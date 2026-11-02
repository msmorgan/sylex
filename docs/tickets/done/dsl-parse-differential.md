---
needs: [dsl-parser]
---
# Prove `deckmaste_dsl` and SylGen parse the `.syl` corpus identically

Standard constraints apply. `semantics-rust-rewrite` requires the Rust and Lean `.syl` parsers to
produce identical trees for the whole corpus. With `deckmaste_dsl` (`dsl-parser`) as the Rust
parser, this ticket closes that gate before the epic starts.

## Change

- **Lean side:** SylGen gains a way to dump the trees its parser produces
  (`SylGen.Parser`'s `Declaration` and `Value`, before resolution) for every `plugins_v3/**/*.syl`
  file, in a stable machine-readable form, to a gitignored path. It is a development tool and
  stays outside the Lake module-generation path.
- **Rust side:** a check that parses the same files with `deckmaste_dsl`, projects each tree onto
  what the Lean `Declaration` holds, and compares them file by file. The projection rebuilds the
  data Lean's parser derives instead of reading it from the source:
  - the result sort Lean fills in from the kind (`kindSort`) when none is written;
  - for a nameless `card`, Lean's `name` (`printedName`, which joins multi-face names with
    `" // "`) and its `cardValue`, with `fields` left empty;
  - Lean's `documentation`: each comment text followed by `"\n"` (so it ends in a newline),
    reset at a blank line.

  Positions, comments other than the doc comment, and anything else outside `Declaration` are left out. The check reads
  the Lean dump from its gitignored path at runtime.

  If moving `kindSort` and `printedName` out of `SylGen.Parser` into resolution turns out simpler
  than rebuilding them in the projection, do that instead. Either way, the comparison stays
  syntactic.

## Acceptance

- No differences across the corpus. A difference is a real defect in one of the parsers: fix it
  there, even if the fix is in SylGen.
- A test shows the check fails loudly on an injected mismatch.
- `cargo xtask lean check`, plus per-crate tests and clippy for every Rust crate touched.
