---
needs: []
---

# Dungeon names are game vocabulary, generated from the card data

## Why

A dungeon is a card type [CR#309.1] whose cards begin outside the game and are brought in by
venturing into the dungeon [CR#309.2]. A player completes a dungeon as its card leaves the game
[CR#309.7], and a card's text can name the dungeon: *Acererak the Archlich*'s "if you haven't
completed Tomb of Annihilation". Its name is listed in `crates/vocabulary/catalogs/cited_names.ron`,
scoped to *Acererak the Archlich* alone, beside three expansion names. A dungeon's name names a game
object, so it is vocabulary in scope on every card, like a card name, and no card list belongs
on it.

The CR does not enumerate dungeons: it names only Undercity, as the one dungeon type [CR#205.3p].
The card data does. The Scryfall Oracle snapshot (`data/scryfall/oracle-cards.jsonl`, card type
Dungeon) and mtgjson (`data/mtgjson/AllPrintings.sqlite`, `tokens` table, type Dungeon) list the
four real non-oversized dungeon cards — Tomb of Annihilation, Lost Mine of Phandelver and Dungeon of
the Mad Mage (`tafr`), and Undercity (`tclb`, front face of double-faced token *Undercity // The
Initiative*) — plus one oversized Commander Party aid (*Baldur's Gate Wilderness*, `tclb`). No
in-scope card is a dungeon. Outside reminder text, the only in-scope card that names a dungeon is
*Acererak the Archlich*. Every other in-scope "Undercity" is part of a card name (*Undercity
Informer*). The initiative's inherent ability says *ventures into Undercity* [CR#727.2], a quality
the dungeon type Undercity matches [CR#701.49d,205.3p].

## Design

- Dungeon names come from the Oracle data, as card names do (`docs/design/lexicon.md` §3):
  `cargo xtask catalogs generate` iterates faces (handling double-faced *Undercity // The
  Initiative*) on `!oversized && set_type == "token"` entries whose face `type_line` has card type
  `Dungeon`, writing the four real dungeon names (*Dungeon of the Mad Mage*, *Lost Mine of
  Phandelver*, *Tomb of Annihilation*, *Undercity*) to `data/gen/catalogs/`, and `sylex_vocabulary`
  loads each as a Proper Name with no Card Scope.
- *Undercity* is generated alongside the other three: singular count `subtype:Undercity` cannot
  project a bare `NounPhrase` after *into*, so the `ProperName` entry gives `"Venture into
  Undercity."` [CR#727.2,701.49d] its single Reading.

## Done when

- The generated dungeon-name catalog exists with the four real dungeon names (excluding *Baldur's
  Gate Wilderness*) and loads, and the loader refuses a dungeon name that a Core word of the same
  Category spells.
- `Tomb of Annihilation` is gone from `cited_names.ron`. *Acererak the Archlich*'s first line
  reads "Tomb of Annihilation" as a dungeon name, its judgment in
  `crates/english/judgments/mentions_and_votes.txt` still holds, and `"Venture into Undercity."`
  parses with 1 Reading.
- The same line, parsed with no Card Scope, has the same Reading.
- The `cited_names.ron` header comment no longer mentions dungeons, and the glossary's Proper
  Name entry lists dungeon names.
- `cargo xtask english strata --check` passes, the census is unchanged, and tests and clippy
  pass for `sylex_vocabulary`, `sylex_english` and `xtask`.

## Not in scope

- Modeling dungeons, their rooms or venturing.
- The expansion names in `cited_names.ron`: `english-set-names-read-from-text`.

## Pointers

- `crates/vocabulary/catalogs/cited_names.ron`, `crates/vocabulary/src/lib.rs`
  (`scoped_name_catalogs`), `crates/xtask/src/catalogs.rs`.
- `docs/design/lexicon.md` §3; `docs/contexts/oracle-english/CONTEXT.md` (Proper Name).
- Evidence (gitignored): `docs/evidence/names-read-from-text/`.
