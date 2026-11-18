---
needs: [english-names-read-from-text]
---

# Anchor words are read from the text across a face's units

## Why

`crates/vocabulary/catalogs/anchor_words.ron` (15 words on 14 cards) prepares each Anchor Word
[CR#614.12c] and the cards that print it before parsing, loading two card-scoped Lexemes per entry
(`anchor_name:` as a Proper Name for `choose A or B`, and `anchor_word:` as a registered label for
`• A — …`). An Anchor Word has no game meaning of its own (`core-as-enters-choices`): the
replacement ability offers two words to choose between as the permanent enters, and each bulleted
ability `[Anchor word] — [ability]` is linked to that choice [CR#614.12c,607.2m]. `anchor_words.ron`
goes.

Neither Corpus Unit of an anchor-word block identifies the words in isolation. On the choice line,
*Roots of Life* prints "As this enchantment enters, choose Island or Swamp." with no bullets (a
linked subtype choice [CR#607.2d], one of four non-anchor `choose <Capital…> or <Capital…>` lines in
the corpus), and the 15 Anchor Words include a card face name (*Believe* on *Phenomenon
Investigators*, face of *Reason // Believe*), subtypes (*Dragons* on the five *Khans*/*Dragons*
Sieges, *Phyrexian* on *Mirrodin Besieged*) and an all-caps acronym (*NCR* on *Battle of Hoover
Dam*). On a bulleted line in isolation, `• Khans — …` has the same surface shape as a flavor-labeled
mode (`• Thay — …` on *Cult of Skaro*, `• Feed — …` on *Astarion, the Decadent* [CR#207.2d,700.2]),
and stays out of `Mode` only because `Lexicon::stands_in` refuses `FlavorWord` when `CardScope`
holds the label.

## Design

- Anchor Words are strings read from a face's text across its units, with no catalog or `.syl`
  declaration (`docs/design/lexicon.md` §3 drops its `khans.syl` `AnchorWord`-sort example).
- `CardScope` holds the Anchor Words in a unit's parse context beside the face's own card names, so
  `Lexicon::stands_in`, `Parse::admit` and round-trip law 2 (`docs/design/english.md` §7) keep
  one scope parameter. Each unit still parses into its own Packed Forest, so a face's Derivation
  count stays the sum of its units', never their product.
- Where `CardScope` holds an Anchor Word, `Lexicon::stands_in` admits the open Anchor Word reading
  and refuses `FlavorWord` on the same spelling; elsewhere `FlavorWord` stands and the Anchor Word
  is refused.
- Confirmation across units is paired in a shared `sylex_english` face-scope helper (called by
  `census`, `forests`, `english parse --card` and `english forestwalk --card`): candidate words
  `{A, B}` from an entry-choice unit (`As … enters, choose A or B.`) are committed only when
  bulleted units on the face begin with `• A — ` and `• B — ` [CR#614.12c]; otherwise the choice
  unit parses without anchor candidates (*Roots of Life* keeps its single `WordMention` Reading,
  and *Believe* needs no card-name refusal).
- `english.fug` licenses `choose A or B` through a dedicated binary `or`-coordination complement of
  `choose` whose arms are both read Anchor Words rather than general `NounPhrase`s (1 Derivation on
  the five `choose Khans or Dragons` Sieges).
- Standalone judgment entries and `english parse` state anchor context with `<anchor=Khans>` /
  `--anchor Khans` beside `<card=NAME>`.
- `anchor_words.ron`, `schema::AnchorWord`, and the `anchor_name:` and `anchor_word:` loader paths
  are deleted.

## Done when

- `anchor_words.ron`, `schema::AnchorWord` and the `anchor_name:` and `anchor_word:` loader paths
  are deleted, and `crates/vocabulary/README.md` no longer lists `anchor_words.ron`.
- Every unit of the 14 anchor-word cards (*Barrensteppe Siege*, *Citadel Siege*, *Frontier Siege*,
  *Frostcliff Siege*, *Glacierwood Siege*, *Hollowmurk Siege*, *Mirrodin Besieged*, *Monastery
  Siege*, *Outpost Siege*, *Palace Siege*, *Windcrag Siege*, *Battle of Hoover Dam*, *Phenomenon
  Investigators*, *Struggle for Project Purity*) is read, with each bulleted unit parsed as
  `AnchoredAbility` (`!Mode`, 1 Reading on the *Frontier Siege* bullets) and 1 Derivation on the
  five `choose Khans or Dragons` choice lines.
- Non-anchor choices (*Roots of Life*'s "As this enchantment enters, choose Island or Swamp.") and
  flavor-labeled modal bullets (*Dawnbringer Cleric*, *Cult of Skaro*, *Astarion, the Decadent*)
  keep their verdicts and Derivation counts.
- Standalone judgment and CLI tests pin `anchored_abilities.txt`, admission's refusal of an
  unintroduced Anchor Word, and `FlavorWord` standing on the same spelling off-scope, replacing the
  `anchor_words.ron`-backed tests in `crates/vocabulary/tests/labels.rs`,
  `crates/vocabulary/tests/loader.rs` and `crates/lexicon/tests/analysis.rs`.
- `docs/design/lexicon.md` (§2 open slots, §3 dropping the `khans.syl` `AnchorWord`-sort
  example) and `docs/contexts/oracle-english/CONTEXT.md` (Anchor Word, Card Scope, Proper Name)
  state the design.
- `cargo xtask english strata --check` passes, and `cargo test` and `cargo clippy --tests -- -D
  warnings` pass for `sylex_lexicon`, `sylex_vocabulary`, `sylex_syntax`, `sylex_english` and
  `xtask`.

## Not in scope

- Authoring or running anchor-word choices in `.syl` or the engine (`core-as-enters-choices`).
- `vote_options.ron` (`english-vote-options-read-from-text`).

## Pointers

- `crates/vocabulary/catalogs/anchor_words.ron`, `crates/vocabulary/src/lib.rs`,
  `crates/vocabulary/src/schema.rs`.
- `crates/lexicon/src/analysis.rs` (`ItalicHead`, `stands_in`, `flavor_word_matches`),
  `crates/lexicon/src/scope.rs` (`CardScope`).
- `crates/english/grammar/english.fug` (`AnchorWord`, `AnchoredAbility`, `FlavorLabeledAbility`,
  `ModeBullet`), `crates/english/src/census.rs` (`run`),
  `crates/english/src/profile.rs`, `crates/english/src/judgment.rs`,
  `crates/english/judgments/anchored_abilities.txt`.
- `crates/xtask/src/english.rs`, `crates/xtask/src/english/forests.rs`.
- `docs/design/lexicon.md` §2–3, `docs/design/english.md` §7,
  `docs/contexts/oracle-english/CONTEXT.md`.
