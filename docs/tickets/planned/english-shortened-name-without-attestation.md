---
needs: [english-parse-corpus-load]
---

# A legendary card's Shortened Name does not depend on its text using it

## Why

A card's name is a proper name with its forms: the Full form and, for a legendary card, a
Shortened form. [CR#201.5c] reads: "Text printed on some cards refers to that card by a shortened
version of its name. Instances of a card’s shortened name used in this manner are treated as
though they used the card’s full name." The Oracle convention gives the shortened form to a
legendary name (`docs/oracle-style-guide.md` §5: "An unambiguous shortened form may replace a
long legendary name").

The lexicon gives a card-name Lexeme its Shortened form under two conditions: the face is a
legendary permanent with a candidate from the rule (`shortening::short_name`, a function of the
name and the type line), and the face's own Oracle text uses that candidate
(`shortening::attested_short_name`). The second condition is wrong: whether a card has a
Shortened Name does not depend on its printed text happening to use it.

- *Abuelo, Ancestral Echo* is a Legendary Creature whose text never says *Abuelo*, so
  `cargo xtask english parse --as-card "Abuelo, Ancestral Echo" "When Abuelo enters, draw a card."`
  has a Vocabulary Gap at *Abuelo*. It should read.
- Attestation is computed by scanning every card's text (`AttestedShortenings::from_corpus`),
  each time `LexicalEnvironment::load` runs. That scan is the one read of
  `data/scryfall/oracle-cards.jsonl` left on every `english parse` call, about 1 s of 2 s.
  Without attestation, a Shortened form needs only the name and that the card is legendary.

Over the 4,466 supported legendary permanent faces: 4,192 have a candidate; 1,911 are attested;
2,281 have a candidate and no Shortened form today. In 2,268 of those the text uses no
shortening of the name at all, and in 13 the candidate occurs only inside a longer title the
text names (*Mishra* in *Mishra, Lost to Phyrexia* on *Mishra, Claimed by Gix*; *Nissa* in
*Nissa's Chosen* on *Nissa Revane*; *Marit* in *Marit Lage* on *Marit Lage's Slumber*). No
legendary face's text uses a shortening other than its candidate: attestation never chooses the
spelling, it only withholds the form.

A name a card's text gives another object (*Rinoa Heartilly*'s *create Angelo, a legendary ...
token*) is a different matter with its own ticket; this one is about a card's own name.

## Open questions

- How a stated-name parse states that the card is legendary, the one printed characteristic the
  Shortened form needs beyond the name: on the command line (`--as-card NAME` with a type line
  or a `--legendary` flag) and in a judgment entry (`<card=NAME>` today states the name alone).
  A stated name that is a real card's could take it from the card-name list if that list
  carried it, which is again a fact prepared per card.
- Whether the Shortened form needs a permanent type as well as the legendary supertype, as
  `short_name` requires today.
- Which word is the Shortened form where the name does not show it. `SURNAME_TITLES` lists five
  titles by hand (*Captain*, *Commander*, *Dr.*, *Ensign*, *Lieutenant*: *Captain James T. Kirk*
  is *Captain Kirk*) because nothing in a name's words marks a title. 274 legendary faces have
  no candidate (one-word names, names starting *The*).
- The 13 faces whose candidate is spelled only inside a longer title in their own text: with
  the form in scope, *Mishra* is a second analysis of the first word of *Mishra, Lost to
  Phyrexia*. Whether the grammar refuses it there, or the form is withheld by another rule.
- Whether `names::shortened_name` still drops a Shortened form that other vocabulary in scope
  spells (*Sliver* in *Sliver Queen*); it analyzes the candidate against the vocabulary and
  reads no card text.
- Whether a Card Scope still holds a name form by form. Nothing builds a scope with a name in
  its Full form alone now, and `crates/syntax/tests/card_names.rs` pins that capability
  (`CardScope::insert` with one `NameForm`).
- Whether a stated name that is no line of `data/gen/catalogs/card-names.txt` (*Abuelo the
  Brave*) is read as a card's own name, with its forms, or stays refused.

## Done when

- `--as-card "Abuelo, Ancestral Echo" "When Abuelo enters, draw a card."` reads *Abuelo* as that
  card's own name in its Shortened form.
- `LexicalEnvironment::load` reads no card text, and an `english parse` call that fetches no
  card reads no `oracle-cards.jsonl`.
- The pool census is compared before and after, and every unit whose verdict or count changes is
  reported.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for every crate changed.

## Pointers

- `crates/vocabulary/src/shortening.rs` (`short_name`, `attested_short_name`,
  `AttestedShortenings`, `SURNAME_TITLES`), `crates/vocabulary/src/names.rs` (`shortened_name`,
  `card_names`), `crates/vocabulary/src/lib.rs` (`LexicalEnvironment::load`).
- `crates/english/src/judgment.rs` (`Judgment::scope`), `crates/xtask/src/english.rs`
  (`stated_items`), `crates/lexicon/src/scope.rs`.
- `docs/decisions/lexicon.md` §6, `docs/oracle-style-guide.md` §5.
