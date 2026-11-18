---
needs: []
---

# A counter's kind name is read from the text

## Why

`crates/vocabulary/catalogs/counters.ron` lists every counter kind name before parsing, and
`KindName: Kind -> Lex(Catalog)` reads one. Counter kind lexemes have no part of speech of their
own: they are `Catalog`, as card names, token names and dungeon names are, so the Production
names more than counter kinds. A kind name is whatever word a card puts before *counter*, and a
card yet to be printed may use any word.

## Design

- A kind name is read from the text: the word standing directly before *counter* or *counters*
  is that counter's kind, with no inventory, as a Set Name is read before *expansion*
  (`docs/design/lexicon.md` §2). *Put a goddamn counter on it.* reads.
- A slash measure (*+1/+1*) [CR#122.1a] and a keyword a keyword counter can be (*flying*,
  *first strike*) [CR#122.1b] still resolve to themselves before *counter*, never to an
  arbitrary kind name.
- A keyword a keyword counter can be and *counter* stay a licensed term (`docs/design/lexicon.md`
  §5), and no Kind Name stands where the keyword ends. A Kind Name holds no license: given every
  Kind Name the license, *Players can't get counters.* (Solemnity) loses its only Reading, and
  *get* is a core monotransitive verb as *unlock* is, so no rule local to a line tells the verb
  before its Object from the kind without a list of kinds.
- `counters.ron`, its schema type and loader path, and the `counter:` Lexemes are deleted.

## Done when

- `counters.ron` and the `counter:` Lexemes are gone, and `KindName` reads the word before
  *counter*.
- Every unit that prints a counter kind keeps its kind Readings; a judgment entry shows an unlisted
  word read as a kind, and others show a measure and a keyword before *counter* read as
  themselves (real cards for the last two).
- A word that is also a verb or noun elsewhere (*time*, *plot*, *unlock*, *loyalty*) keeps its
  kind Reading (*Deadly Designs*, *Cryptex*, *Stalwart Successor*, *Ajani Unyielding*). Where
  the English also reads it as a verb before its Object (*plot*, *unlock*, *healing*, and
  *counters* after *five loyalty*), those Readings stand beside the kind's, and
  `english-kind-homograph-verb-readings` takes up the structures they ride on.
- The census shows which Readings changed and why, with no verdict loss, and
  `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings`
  pass for every changed crate.

## Pointers

- `crates/vocabulary/catalogs/counters.ron`; `crates/english/grammar/english.fug` (`Kind`,
  `KindName`, `KeywordKind`, `MeasureKind`, `KindPremodifier`, `feature KindLicense`).
- `crates/lexicon/src/set_name.rs` (a run read only before its host noun).
