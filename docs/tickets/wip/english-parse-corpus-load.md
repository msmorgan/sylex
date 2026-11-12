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

- Two ways to give `english parse` a card. By name alone: the tool reads that card's record and
  parses its text, with the scope derived from that record. As text with a stated name: nothing
  is looked up, and the name is an input like the text. A judgment entry is the second.
- `CardScopes` is deleted. `Judge` takes no scopes; an entry's scope is computed from the entry.
- The whole card's name leaves `corpus::own_names`.

## Open questions

- How a meld result's name is in scope where an entry states only the card's name:
  - the grammar admits Another Card's Name as the complement of *into* under *meld*, a fourth
    position beside *named*, *Partner with* and *from among*; no scope then holds a meld
    result, `own_names` reads no `all_parts`, and the Own Name entry of
    `oracle-english/CONTEXT.md` and `lexicon.md` §6 lose their meld clause;
  - the entry states it (`<card=Gisela, the Broken Blade; meld=Brisela, Voice of Nightmares>`),
    and the census keeps reading `all_parts`;
  - the ten entries stay as they are and the scope keeps a lookup for meld cards alone.
- Whether a stated name that is no card-name Lexeme is an error naming the entry, or is read as
  given (its own name is then a Vocabulary Gap, since the lexicon holds no Lexeme for it).
- How the two ways are spelled: `<card=NAME> text` stays the stated-name form in entries and on
  the command line; the by-name form is new (`english parse --card NAME [--line N]`, or it
  belongs to `english-forestwalk-survey`'s unit address and `parse` gets none).
- Whether `LexicalEnvironment::load`'s own read of the corpus is removed here or in its own
  ticket. It computes each legendary permanent's Shortened Name by checking the rule's candidate
  (`shortening::short_name`, a function of name and type line) against that face's text, and
  stores the result as a form of the card name's Lexeme. Until it goes, a `parse` call still
  reads the card JSON once.

## Done when

- `english parse` reads no card JSON for an entry that states its text; `judgment.rs` holds no
  map from card to scope, and `tests/judgments.rs` and `tests/ranking_keys.rs` load no corpus.
- Tests on real cards: *Jace, Cunning Castaway* (*Jace draws two cards.*), a double-faced card's
  back face (*Insectile Aberration*), a meld card as the first open question settles, and a
  line with no card.
- `english parse` prints the same output before and after for the sample in
  `docs/evidence/english-parse-corpus-load/`, and the census verdict counts do not change.
- The landing reports the wall time of one `parse` call with and without a card, before and
  after.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_english` and `xtask`.

## Not in scope

- The card-scoped catalogs (`introduced_names.ron`, `cited_names.ron`, `vote_options.ron`,
  `anchor_words.ron`) and the card-name list: names a card's own text introduces, listed with
  their cards in advance.

## Pointers

- `crates/english/src/judgment.rs` (`CardScopes`, `Judge`), `crates/english/src/corpus.rs`
  (`own_names`, `Face::scope`), `crates/english/src/census.rs` (`judge_parse`).
- `crates/lexicon/src/scope.rs`, `crates/lexicon/src/analysis.rs` (`Lexicon::stands_in`),
  `crates/syntax/src/step.rs` (`leaf_facts`, `other_card_name`).
- `crates/vocabulary/src/shortening.rs`, `crates/vocabulary/src/names.rs`.
- `crates/xtask/src/english.rs` (`parse`), `crates/english/judgments/README.md` (Card).
- `docs/decisions/lexicon.md` §6, `docs/contexts/oracle-english/CONTEXT.md` (Card Scope, Own
  Name, Another Card's Name).
