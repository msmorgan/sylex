---
needs: []
---

# A judgment entry's Card Scope comes from what the entry states

## Why

`cargo xtask english parse` reads `data/scryfall/oracle-cards.jsonl` (211 MB) twice per call:
once inside `LexicalEnvironment::load` (`AttestedShortenings::load`) and once as
`Corpus::load(|_| true)`, to build `judgment::CardScopes`, a map from face name to a prepared
Card Scope that a `<card=NAME>` entry is then looked up in. `tests/judgments.rs`,
`tests/ranking_keys.rs` and the census build the same map. The entry supplies its own text, so
nothing of the card but its scope is taken from the corpus, and a name the corpus lacks is
refused.

What that lookup returns, over all 33,295 supported faces:

- the face's own name in every form, which `<card=NAME>` already states;
- the whole card's name (`A // B`) on 1,694 faces, which is no Lexeme (no card-name Lexeme
  contains ` // `), so no predicate ever asks the scope for it;
- on the 14 faces of the seven meld pairs, the meld result's name in its Full form, from the
  record's `all_parts`. Ten entries of `name_lexeme_scope.txt` and `comma_scope.txt` depend
  on it: every one that uses the name spells it after *meld them into*;
- the check that NAME is a supported face name, which is the same set as the card-name Lexemes
  of the lexicon (33,265 each).

So a Card Scope is `CardScope::own_names([card_name_id(NAME)])` for every face but those 14, and
the meld result's name is the one fact the name does not carry.

## Design

- The snapshot is read once per process. `sylex_upstream::scryfall::oracle_cards` holds the
  parsed records for the process's life, and `Corpus::load` and `AttestedShortenings::load` both
  read them from there. A call that names no card still reads the snapshot once, for the
  Shortened Names.
- A meld result's name is Another Card's Name on both cards of its pair, and the grammar admits
  it as the complement of *into* under *meld*, a fourth position beside *named*, *Partner with*
  and *from among*. No Card Scope holds a meld result; `corpus::own_names` reads no `all_parts`.
- `CardScopes` is deleted. An entry's scope is `CardScope::own_names` of the name it states; the
  census derives each face's scope from the record in hand. The whole card's name (`A // B`)
  leaves `own_names`.
- A stated name that is no card-name Lexeme is refused with an error naming the entry.
- `english parse --as-card NAME TEXT...` parses each text as if it were on the card so named,
  and fetches nothing. `english parse --card NAME`, with no text, reads the faces so named (a
  face's name, or its whole card's) from the supported corpus and parses their own units as the
  census reads them: under a `Face: NAME (card: CARD)` line, the type line and then each
  rules-text line, numbered from 1. `english parse TEXT...` reads with no card. `--card` with
  text, and `--as-card` without, are errors that name the other flag. An input in judgment entry
  syntax still states its own card with `<card=NAME>`; giving that and `--as-card` is an error.
- `english strata` reads the pinned snapshot from the process's records; `--data FILE` reads
  another file.

## Done when

- `judgment.rs` holds no map from card to scope; `tests/judgments.rs` and
  `tests/ranking_keys.rs` load no corpus; one `english parse` call reads the snapshot once.
- `docs/decisions/lexicon.md` §6, the Own Name and Another Card's Name entries of
  `oracle-english/CONTEXT.md`, the judgments README and the meld entries of
  `name_lexeme_scope.txt` state the meld analysis above. `comma_scope.txt` needs no edit: its
  two *Mishra, Claimed by Gix* entries assert only the Clause that is the Complement of *if*,
  which the analysis of *into*'s Complement does not touch.
- The pool census changes on no unit.
- Tests on real cards: *Jace, Cunning Castaway* (*Jace draws two cards.*), a double-faced card's
  back face (*Insectile Aberration*), a meld card (*Gisela, the Broken Blade*), a line with no
  card, and an unknown name refused.
- `english parse` prints the same output before and after for the sample in
  `docs/evidence/english-parse-corpus-load/`, apart from the meld lines.
- The landing reports the wall time of one `parse` call with and without a card, before and
  after.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for every crate changed.

## Not in scope

- The card-scoped catalogs (`introduced_names.ron`, `cited_names.ron`, `vote_options.ron`,
  `anchor_words.ron`) and the card-name list: names a card's own text introduces, listed with
  their cards in advance.
- The Shortened Names' own read of the snapshot, and a Shortened form for a legendary card whose
  text does not use it: `english-shortened-name-without-attestation`.

## Pointers

- `crates/english/src/judgment.rs` (`CardScopes`, `Judge`), `crates/english/src/corpus.rs`
  (`own_names`, `Face::scope`, `Corpus::load`), `crates/english/src/census.rs` (`judge_parse`).
- `crates/upstream/src/scryfall.rs` (`oracle_cards`).
- `crates/lexicon/src/scope.rs`, `crates/lexicon/src/analysis.rs` (`Lexicon::stands_in`),
  `crates/syntax/src/step.rs` (`leaf_facts`, `other_card_name`).
- `crates/vocabulary/src/shortening.rs`, `crates/vocabulary/src/names.rs`.
- `crates/xtask/src/english.rs` (`parse`), `crates/english/judgments/README.md` (Card).
- `docs/decisions/lexicon.md` §6, `docs/contexts/oracle-english/CONTEXT.md` (Card Scope, Own
  Name, Another Card's Name).
