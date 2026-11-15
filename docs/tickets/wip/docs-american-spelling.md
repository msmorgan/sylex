---
needs: []
---
# American spelling throughout the repo

## Why

The repo is written in American English (`CLAUDE.md`, Writing), but British spellings remain in
docs, doc comments, comments, tickets, test names, file names and identifiers: behaviour, colour,
modelled, labelled, labelling, cancelled, recognise, summarise, fulfils, licence and others.

## Done when

- Docs, tickets, doc comments, comments and test names use the American spelling (behavior,
  color, modeled, labeled, labeling, canceled, recognize, summarize, fulfills, license, …).
- Identifiers and grammar names with a British spelling are renamed with their call sites, `.syl`
  uses and glossary entries, one rename each: for example `LabelledAbility`,
  `LabelledModalHeader` and `LabelledCostedMode` (`crates/english/grammar/english.fug`,
  `crates/english/src/profile.rs`), and the judgment files `devotion_to_colour.txt` and
  `color_word_categories.txt`'s contents.
- Verbatim quotations keep their spelling (CGEL excerpts, rules text, other quoted sources), and
  so do `data/` and generated files. A spelling the code lists on purpose, such as "Licence" in
  `crates/syntax/src/compile.rs`'s `FORBIDDEN_SUFFIXES`, stays.
- A grep for the respelled words finds only those exceptions. Words that are American already
  ("analyses" as a plural noun, "initialisms") are left alone.

## Pointers

- `crates/english/grammar/english.fug`, `crates/english/judgments/`, `crates/english/src/profile.rs`
