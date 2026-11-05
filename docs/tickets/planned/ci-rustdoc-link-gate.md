---
needs: []
---

# Make `cargo doc` a gate: escape bracketed citations and placeholders

**Why.** A broken intra-doc link is invisible to clippy and the tests; only rustdoc reports it, and
no gate runs rustdoc. Deleting a type can therefore leave dead doc links behind a green run. The
check is unusable today because doc-comment citations in the mandated `[CR#…]` format parse as
intra-doc links to a nonexistent item `CR`: they are about 2,600 of the 2,750 warnings
`cargo doc --workspace --no-deps` emits, and they bury the real ones.

**Approach.** Escape only the opening bracket of a doc-comment citation: `\[CR#702.148a]`. Rustdoc
renders it as the literal citation, and `cargo xtask cite` extracts the rule numbers unchanged
(verified for single rules, comma lists and `..` ranges). Escaping the closing bracket too is wrong:
the checker reads the trailing backslash into the rule number and reports it malformed. Bracketed
placeholders in doc prose (`[2]`, `[Vampire]`, `[,]`, `[who]`) get the same opening-bracket escape;
code-like text goes in backticks.

**Done when.**
- Every `[CR#` in a `///` or `//!` doc comment is written `\[CR#`. This is a mechanical sweep
  (about 5,300 citations in about 210 files across 16 crates); plain `//` comments and non-Rust
  files are untouched.
- Bracketed placeholders in doc comments are escaped, and HTML-like text (`<effect>`, `<filter>`) is
  in backticks.
- Every remaining warning that is a real dead link is fixed (for example `Predicate::r#type`,
  `crate::Selection::That`, `SubtypeRef`, `DeonticAction::Cast`, `crate::render::template`,
  `sylex_plugin_v1::Strategy`, `VerbName`), and the private-item link
  (`discard_body_whose` → `hand_owner_ref`) is fixed.
- `cargo doc --workspace --no-deps` emits no warnings, and joins the CI gates.
- `cargo xtask cite check` reports 0 stale and 0 non-compliant, and the number of tracked citations
  is unchanged by the sweep.
- `CLAUDE.md`'s citation rule states the escaped form for doc comments.

**Not in scope.** Changing the citation format elsewhere (Markdown, `.syl`, `.ron`, plain comments).
A blanket `#![allow(rustdoc::broken_intra_doc_links)]`, which would hide exactly the breakage this
exists to expose.

**Pointers.** `crates/xtask/src/cite.rs` (`bracketed_refs`, `blank_bracketed_citations`),
`.github/workflows/ci.yml`, `CLAUDE.md` (CR citations).
