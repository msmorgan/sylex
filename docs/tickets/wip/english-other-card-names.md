---
needs: []
---
# A card's own name and another card's name are distinct; another card's name reads only after *named*

## Why

Every card's full name is a word on every card (`docs/decisions/lexicon.md` §6), so the parser
offers the card *Exile* in Excise's "Exile target attacking creature". Today only a name's
Singular Number keeps "target" from being read as its plural verb (the comment on `category Name`
in `english.fug`). With number-neutral names, which let a parse decide a name's Number from its
agreement, six units' cheapest reading becomes "[Name] targets …": Excise, Soul Nova, Unified
Strike, Mystifying Maze ("Exile …"), Reconnaissance ("Remove …") and Kaust, Eyes of the Glade
("Turn …"): *Exile*, *Remove* and *Turn* (of *Turn // Burn*) are other cards' names. A card's text
mentions another card by name in the "named" construct ("a card named Relentless Rats").

## Design

- A card's own name (full or Shortened [CR#201.5c]) is in scope only on its own face, everywhere
  in that text.
- Another card's full name carries a feature that admits it only in the "named" construct, and
  nowhere else.

## Done when

- The grammar and lexicon make the two distinct as above, and `lexicon.md` §6 and the glossary say
  so.
- `cargo xtask english strata --check` passes, and the census lists every in-scope card whose text
  uses another card's name outside "named"; each is handled or reported to the owner.
- With card names number-neutral, none of the six units above reads "[Name] targets …".

## Pointers

- `crates/vocabulary/src/names.rs` (`card_names`), `crates/vocabulary/src/lib.rs`
  (`Sources::environment`)
- `crates/english/grammar/english.fug` (`category Name`, `NamedParameter`, `ProperName`,
  `ProperPossessor`)
- `docs/decisions/lexicon.md` §6, `docs/contexts/oracle-english/CONTEXT.md` (Shortened Name, Name
  Number)
