---
needs: []
---

# Rename `Category::Catalog` to `Category::ProperName`

**Why.** "Catalog" means the inventory of game vocabulary and the Provenance kind for it
(`docs/decisions/lexicon.md` §3). `Category::Catalog` is a different thing: the Category of opaque
names (card names, Shortened Names, Introduced Names, Anchor Words). One word with two meanings
invites confusion. The glossary names the Category Proper Name, CGEL's term for such expressions.
The variant's doc comment ("an explicitly named inventory whose grammatical distribution is not
yet mapped") is also out of date.

**Done when.**
- `Category::Catalog` is `Category::ProperName` everywhere it is used (code, tests, RON data,
  grammar data), with a doc comment that says what the Category holds.
- The glossary's Proper Name entry drops its "proposed" mark, and `lexicon.md` §2 uses the name.
- `cargo xtask english strata --check` passes, and tests and clippy pass for every crate touched.

**Pointers.** `crates/lexicon/src/types.rs` (`Category`), `crates/vocabulary/src/lib.rs`,
`docs/contexts/oracle-english/CONTEXT.md` (Proper Name), `docs/decisions/lexicon.md` §2.
